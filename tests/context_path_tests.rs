use azumi::context::{get_current_path, with_path};

#[tokio::test]
async fn test_get_current_path_returns_none_outside_scope() {
    assert!(
        get_current_path().is_none(),
        "get_current_path should return None when no with_path scope is active"
    );
}

#[tokio::test]
async fn test_with_path_sets_current_path() {
    let result = with_path("/about".to_string(), async {
        get_current_path()
    })
    .await;

    assert_eq!(result, Some("/about".to_string()));
}

#[tokio::test]
async fn test_with_path_cleans_up_after_scope() {
    with_path("/temp".to_string(), async {
        assert_eq!(get_current_path(), Some("/temp".to_string()));
    })
    .await;

    assert!(
        get_current_path().is_none(),
        "get_current_path must return None after with_path scope ends"
    );
}

#[tokio::test]
async fn test_nested_with_path_overrides_inner() {
    let result = with_path("/outer".to_string(), async {
        assert_eq!(get_current_path(), Some("/outer".to_string()));

        with_path("/inner".to_string(), async {
            get_current_path()
        })
        .await
    })
    .await;

    assert_eq!(result, Some("/inner".to_string()));
}

#[tokio::test]
async fn test_nested_with_path_restores_outer() {
    with_path("/outer".to_string(), async {
        let inner = with_path("/inner".to_string(), async {
            get_current_path()
        })
        .await;
        assert_eq!(inner, Some("/inner".to_string()));

        let after_inner = get_current_path();
        assert_eq!(
            after_inner,
            Some("/outer".to_string()),
            "Outer path must be restored after inner scope exits"
        );
    })
    .await;
}

#[tokio::test]
async fn test_concurrent_tasks_have_isolated_paths() {
    let h1 = tokio::spawn(with_path("/path-a".to_string(), async {
        tokio::task::yield_now().await;
        get_current_path()
    }));

    let h2 = tokio::spawn(with_path("/path-b".to_string(), async {
        tokio::task::yield_now().await;
        get_current_path()
    }));

    assert_eq!(h1.await.unwrap(), Some("/path-a".to_string()));
    assert_eq!(h2.await.unwrap(), Some("/path-b".to_string()));
}
