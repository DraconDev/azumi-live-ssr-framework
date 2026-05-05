use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

tokio::task_local! {
    static CURRENT_PATH: String;
}

/// Run a future with the current request path derived from context
pub async fn with_path<F: Future>(path: String, f: F) -> F::Output {
    CURRENT_PATH.scope(path, f).await
}

/// Get the current request path if available in the task context
pub fn get_current_path() -> Option<String> {
    CURRENT_PATH.try_with(|p| p.clone()).ok()
}

// ============================================================================
// Sync Context for Rendering (Thread Local)
// ============================================================================

#[derive(Clone, Default, Debug)]
pub struct PageMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

thread_local! {
    static PAGE_META: RefCell<PageMeta> = RefCell::new(PageMeta::default());
}

#[derive(Clone)]
#[allow(dead_code)]
struct PageMetaState {
    refcount: Rc<std::sync::atomic::AtomicU32>,
}

impl PageMetaState {
    fn new() -> Self {
        Self {
            refcount: Rc::new(std::sync::atomic::AtomicU32::new(1)),
        }
    }

    fn clone(&self) -> Self {
        self.refcount.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self {
            refcount: Rc::clone(&self.refcount),
        }
    }
}

impl Drop for PageMetaState {
    fn drop(&mut self) {
        if self.refcount.fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1 {
            PAGE_META.with(|params| *params.borrow_mut() = PageMeta::default());
        }
    }
}

thread_local! {
    static PAGE_META_STATE: PageMetaState = PageMetaState::new();
}

/// RAII guard that resets PAGE_META to default when all guards are dropped.
/// Ensures metadata from one request cannot leak into another.
///
/// # Thread Safety
///
/// `PageMetaGuard` uses atomic reference counting and is `Sync`.
/// However, guards must stay on the thread where they were created because
/// `PAGE_META` is a `thread_local!` `RefCell` which is not thread-safe.
///
/// **Key limitation**: Guards must not cross thread boundaries.
///
/// This is intentional: each thread has its own `PAGE_META`, so developers must ensure
/// guards do not cross thread boundaries.
///
/// # Migration Path
///
/// To make this async-safe, `tokio::task_local!` should replace `thread_local!` for
/// `PAGE_META`. This requires:
/// 1. Changing `PageMetaState` from `Rc<AtomicU32>` to `Arc<AtomicU32>`
/// 2. Adapting the guard's `Drop` to use `tokio::task_local!` scope API
/// 3. The inner `RefCell` type does NOT need to be `Send` (tokio::task_local supports !Send)
/// 
/// This is a planned improvement but requires careful testing across async boundaries.
#[derive(Clone)]
#[allow(dead_code)]
pub struct PageMetaGuard(PageMetaState);

impl PageMetaGuard {
    fn new() -> Self {
        Self(PAGE_META_STATE.with(|s| s.clone()))
    }
}

/// Set the metadata for the current page render and return a guard.
/// The guard ensures metadata is reset when all copies of the guard are dropped.
///
/// # Safety / TOCTOU Warning
///
/// This function uses a `thread_local!` `RefCell` which is **not async-safe** when
/// the guarded value is accessed across `.await` points. Specifically:
///
/// - `set_page_meta()` writes to `PAGE_META`
/// - If the async code that reads `PAGE_META` (via `get_page_meta()`) yields at an `.await`,
///   another task on the same thread can overwrite `PAGE_META`
/// - When the guard drops, it resets `PAGE_META` — but by this point the original
///   async task may have resumed with the wrong metadata
///
/// **Rule**: Do not pass `PageMetaGuard` across `.await` points. The guard should
/// be dropped before any async operation that might yield.
///
/// Example of UNSAFE usage:
/// ```ignore
/// let guard = set_page_meta(...);
/// some_async_operation().await; // BAD: another task could overwrite PAGE_META here
/// let meta = get_page_meta();   // May read corrupted/wrong metadata
/// drop(guard);                  // Resets at wrong time
/// ```
///
/// Example of SAFE usage:
/// ```ignore
/// let guard = set_page_meta(...);
/// let meta = get_page_meta();   // Read immediately before any await
/// drop(guard);                  // Reset immediately
/// some_async_operation().await; // Now safe to await
/// ```
pub fn set_page_meta(
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
) -> PageMetaGuard {
    PAGE_META.with(|params| {
        *params.borrow_mut() = PageMeta {
            title,
            description,
            image,
        };
    });
    PageMetaGuard::new()
}

/// Get the current page metadata.
/// This is used by the `head!` macro or layout components.
pub fn get_page_meta() -> PageMeta {
    PAGE_META.with(|params| params.borrow().clone())
}
