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

#[test]
fn test_cloned_page_meta_guard_preserves_state_until_all_dropped() {
    use azumi::context::{set_page_meta, get_page_meta, has_page_meta_scope};

    assert!(!has_page_meta_scope());

    let guard1 = set_page_meta(Some("Cloned Guard Test".to_string()), None, None);
    let meta = get_page_meta();
    assert_eq!(meta.title.as_deref(), Some("Cloned Guard Test"));

    let guard2 = guard1.clone();
    drop(guard1);
    let meta_after_first_drop = get_page_meta();
    assert_eq!(
        meta_after_first_drop.title.as_deref(),
        Some("Cloned Guard Test"),
        "State must persist after first guard drop when clone is still alive"
    );

    drop(guard2);
    let meta_after_second_drop = get_page_meta();
    assert!(
        meta_after_second_drop.title.is_none(),
        "State must reset only after ALL guards are dropped"
    );
}

#[test]
fn test_multiple_set_page_meta_calls_share_refcount() {
    use azumi::context::{set_page_meta, get_page_meta, has_page_meta_scope};

    assert!(!has_page_meta_scope());

    let _g1 = set_page_meta(Some("First".to_string()), None, None);
    let _g2 = set_page_meta(Some("Second".to_string()), None, None);

    let meta = get_page_meta();
    assert_eq!(meta.title.as_deref(), Some("Second"), "Last set_page_meta wins");

    drop(_g1);
    let meta_after_first_drop = get_page_meta();
    assert_eq!(
        meta_after_first_drop.title.as_deref(),
        Some("Second"),
        "Dropping first guard must not reset state while second guard is alive"
    );

    drop(_g2);
    let meta_after_second_drop = get_page_meta();
    assert!(
        meta_after_second_drop.title.is_none(),
        "State must reset only after ALL guards are dropped"
    );
}
