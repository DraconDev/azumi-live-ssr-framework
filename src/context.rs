use std::future::Future;
use std::sync::Mutex;

tokio::task_local! {
    static CURRENT_PATH: String;
}

tokio::task_local! {
    static PAGE_META: Mutex<PageMeta>;
}

pub async fn with_path<F: Future>(path: String, f: F) -> F::Output {
    CURRENT_PATH.scope(path, f).await
}

pub fn get_current_path() -> Option<String> {
    CURRENT_PATH.try_with(|p| p.clone()).ok()
}

// ============================================================================
// Page Metadata (Task-Local, Async-Safe)
// ============================================================================

#[derive(Clone, Default, Debug)]
pub struct PageMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

/// Run a future with a page metadata scope.
///
/// This establishes a task-local `PAGE_META` that is automatically cleaned up
/// when the scope ends. Call this at the entry point of each request handler
/// (typically via `render_to_string` or the `#[azumi::page]` macro).
///
/// # Async Safety
///
/// Unlike the previous `thread_local!` implementation, `PAGE_META` is now
/// task-local. This means:
/// - Each tokio task has its own isolated metadata
/// - No data races when tasks migrate between threads at `.await` points
/// - Metadata is automatically cleaned up when the scope ends
///
/// # Example
///
/// ```ignore
/// async fn handler() -> impl IntoResponse {
///     azumi::context::with_page_meta_scope(async {
///         let component = my_page();
///         axum::response::Html(azumi::render_to_string(&component))
///     }).await
/// }
/// ```
pub async fn with_page_meta_scope<F: Future>(f: F) -> F::Output {
    PAGE_META.scope(Mutex::new(PageMeta::default()), f).await
}

/// RAII guard for page metadata. Kept for API compatibility.
///
/// With the task-local implementation, cleanup happens automatically when the
/// scope ends (via `with_page_meta_scope`). The guard no longer needs to
/// reset `PAGE_META` on drop, but is retained for backward compatibility.
#[derive(Clone, Default)]
pub struct PageMetaGuard(());

/// Set the metadata for the current page render and return a guard.
///
/// The guard is retained for API compatibility but no longer performs cleanup
/// on drop — the task-local scope handles that automatically.
///
/// # Panics
///
/// This function will not panic. If called outside a `with_page_meta_scope`,
/// the metadata is silently discarded and `get_page_meta()` returns defaults.
pub fn set_page_meta(
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
) -> PageMetaGuard {
    let _ = PAGE_META.try_with(|m| {
        if let Ok(mut guard) = m.lock() {
            *guard = PageMeta {
                title,
                description,
                image,
            };
        }
    });
    PageMetaGuard(())
}

/// Get the current page metadata.
///
/// Returns `PageMeta::default()` if called outside a `with_page_meta_scope`.
/// This is used by the `head!` macro or layout components.
pub fn get_page_meta() -> PageMeta {
    PAGE_META
        .try_with(|m| m.lock().map(|g| g.clone()).unwrap_or_default())
        .unwrap_or_default()
}
