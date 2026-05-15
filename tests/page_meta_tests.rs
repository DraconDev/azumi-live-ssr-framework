use azumi::{html, render_page, render_to_string};

#[tokio::test]
async fn test_render_page_sets_page_meta_scope() {
    let component = html! { <div>"Hello"</div> };
    let html = render_page(&component).await;
    assert!(html.contains("<div>"));
    assert!(html.contains("Hello"));
}

#[tokio::test]
async fn test_render_page_produces_same_output_as_render_to_string() {
    let sync_output = render_to_string(&html! { <p>"Test content"</p> });
    let async_output = render_page(&html! { <p>"Test content"</p> }).await;
    assert_eq!(sync_output, async_output);
}

#[tokio::test]
async fn test_with_page_meta_scope_isolation() {
    use azumi::context::{with_page_meta_scope, set_page_meta, get_page_meta, has_page_meta_scope};

    assert!(!has_page_meta_scope(), "No scope should be active before test");

    with_page_meta_scope(async {
        assert!(has_page_meta_scope(), "Scope should be active inside with_page_meta_scope");

        set_page_meta(Some("Test Title".to_string()), None, None);
        let meta = get_page_meta();
        assert_eq!(meta.title.as_deref(), Some("Test Title"));
    }).await;

    assert!(!has_page_meta_scope(), "Scope should be gone after with_page_meta_scope");
}

#[tokio::test]
async fn test_concurrent_tasks_have_isolated_page_meta() {
    use azumi::context::{with_page_meta_scope, set_page_meta, get_page_meta};

    let handle1 = tokio::spawn(with_page_meta_scope(async {
        set_page_meta(Some("Task 1".to_string()), None, None);
        tokio::task::yield_now().await;
        let meta = get_page_meta();
        meta.title
    }));

    let handle2 = tokio::spawn(with_page_meta_scope(async {
        set_page_meta(Some("Task 2".to_string()), None, None);
        tokio::task::yield_now().await;
        let meta = get_page_meta();
        meta.title
    }));

    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();

    assert_eq!(result1.as_deref(), Some("Task 1"));
    assert_eq!(result2.as_deref(), Some("Task 2"));
}

#[test]
fn test_thread_local_fallback_when_no_scope() {
    use azumi::context::{set_page_meta, get_page_meta, has_page_meta_scope};

    assert!(!has_page_meta_scope());

    {
        let _guard = set_page_meta(Some("Fallback Title".to_string()), Some("desc".to_string()), None);
        let meta = get_page_meta();
        assert_eq!(meta.title.as_deref(), Some("Fallback Title"));
        assert_eq!(meta.description.as_deref(), Some("desc"));
    }

    let meta_after_drop = get_page_meta();
    assert!(meta_after_drop.title.is_none(), "Thread-local should be reset after guard drops");
}
