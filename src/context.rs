use std::cell::RefCell;
use std::future::Future;
use std::sync::Mutex;

tokio::task_local! {
    static CURRENT_PATH: String;
}

tokio::task_local! {
    static PAGE_META: Mutex<PageMeta>;
}

thread_local! {
    static PAGE_META_FALLBACK: RefCell<PageMeta> = RefCell::new(PageMeta::default());
}

pub async fn with_path<F: Future>(path: String, f: F) -> F::Output {
    CURRENT_PATH.scope(path, f).await
}

pub fn get_current_path() -> Option<String> {
    CURRENT_PATH.try_with(|p| p.clone()).ok()
}

// ============================================================================
// Page Metadata (Task-Local primary, Thread-Local fallback)
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
/// when the scope ends. When the scope is active, `set_page_meta` and
/// `get_page_meta` use the task-local store, which is async-safe — no data
/// races when tasks yield at `.await` points.
///
/// When the scope is NOT active (e.g., synchronous `render_to_string` calls
/// outside an async context), the functions fall back to a thread-local store.
/// The thread-local fallback is NOT async-safe but preserves backward
/// compatibility for non-async rendering paths.
///
/// # Example
///
/// ```ignore
/// async fn handler() -> impl IntoResponse {
///     azumi::context::with_page_meta_scope(async {
///         let html = azumi::render_to_string(&my_page());
///         axum::response::Html(html)
///     }).await
/// }
/// ```
pub async fn with_page_meta_scope<F: Future>(f: F) -> F::Output {
    PAGE_META.scope(Mutex::new(PageMeta::default()), f).await
}

/// RAII guard that resets the fallback `PAGE_META` to default when all
/// guards are dropped.
///
/// When the task-local scope is active, this guard is a no-op — the scope
/// handles cleanup automatically. When using the thread-local fallback, the
/// guard resets the metadata on drop to prevent leakage between requests.
///
/// Cloning is supported: the thread-local is only reset when the last guard
/// (the one with `Arc` strong_count == 1) is dropped.
pub struct PageMetaGuard {
    _refcount: std::sync::Arc<()>,
}

impl Clone for PageMetaGuard {
    fn clone(&self) -> Self {
        PageMetaGuard {
            _refcount: std::sync::Arc::clone(&self._refcount),
        }
    }
}

impl Drop for PageMetaGuard {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self._refcount) == 1 {
            PAGE_META_FALLBACK.with(|params| *params.borrow_mut() = PageMeta::default());
        }
    }
}

/// Set the metadata for the current page render and return a guard.
///
/// When called inside a `with_page_meta_scope`, writes to the task-local
/// store (async-safe). Otherwise, falls back to the thread-local store.
///
/// The guard resets the thread-local fallback on drop. When using the
/// task-local scope, the guard is a no-op but retained for API compatibility.
pub fn set_page_meta(
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
) -> PageMetaGuard {
    let meta = PageMeta {
        title,
        description,
        image,
    };

    let task_local_ok = PAGE_META.try_with(|m| {
        if let Ok(mut guard) = m.lock() {
            *guard = meta.clone();
            true
        } else {
            false
        }
    })
    .unwrap_or(false);

    if !task_local_ok {
        PAGE_META_FALLBACK.with(|params| *params.borrow_mut() = meta);
    }

    PageMetaGuard {
        _refcount: std::sync::Arc::new(()),
    }
}

/// Get the current page metadata.
///
/// When called inside a `with_page_meta_scope`, reads from the task-local
/// store (async-safe). Otherwise, falls back to the thread-local store.
///
/// Returns `PageMeta::default()` if neither store has been set.
pub fn get_page_meta() -> PageMeta {
    PAGE_META
        .try_with(|m| m.lock().map(|g| g.clone()).unwrap_or_default())
        .unwrap_or_else(|_| {
            PAGE_META_FALLBACK.with(|params| {
                params.try_borrow().map(|b| b.clone()).unwrap_or_default()
            })
        })
}

/// Returns `true` if a page metadata task-local scope is active.
pub fn has_page_meta_scope() -> bool {
    PAGE_META.try_with(|_| true).unwrap_or(false)
}
