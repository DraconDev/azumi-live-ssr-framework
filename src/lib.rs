pub mod prelude {
    #[cfg(feature = "axum")]
    pub use crate::action::{ActionResult, error_fragment, success_fragment};
    pub use crate::csp::CspNonce;
    pub use crate::{
        azumi_script, component, html, json_data, live,
        session_cleanup_script, AzumiScript, Component, escape_css_string, escape_html, escape_xml,
        FnComponent, render_to_string, render_to_writer,
    };
    pub use crate::form::{FormValidator, ValidatedForm, ValidationErrors};
}

pub use azumi_macros::{
    component, html, json_data, live,
};
#[doc(hidden)]
pub use azumi_macros::{
    head, live_impl, page, predict,
};
#[cfg(feature = "axum")]
pub use azumi_macros::action;
#[cfg(feature = "axum")]
pub mod action;
pub mod context;
pub mod css_scoping;
#[cfg(feature = "devtools")]
pub mod hot_reload;
pub mod script;
pub mod security;
pub use security::VerifyStateError;
#[cfg(feature = "axum")]
pub use inventory;
#[cfg(feature = "devtools")]
pub mod devtools;

pub mod seo;
pub mod form;
pub mod streaming;
pub mod csp;
pub use script::{AzumiScript, escape_html, escape_script_content, escape_style_content, escape_tag_content, escape_xml, session_cleanup_script};
// Re-export CSS scoping for backward compatibility
pub use css_scoping::{compute_scope_id, scope_css};

// ── Re-exports for declarative macros ─────────────────────────────────────
/// Internal re-exports used by `azumi::routes!` and other declarative macros.
/// Not part of the public API.
#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "axum")]
    pub use axum;
}

#[cfg(feature = "test-utils")]
pub mod test;

// ── AI Framework Fingerprint ──────────────────────────────────────────────
/// Framework version (from Cargo.toml).
pub const AZUMI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// AI framework fingerprint hash.
///
/// This is a deterministic hash computed from the framework version + strict
/// rules. AI assistants should read this value to verify they're targeting
/// the correct framework version and rule set.
///
/// Override by setting `AZUMI_AI_HASH` env var before building.
///
/// # Usage in `.env` or CI
/// ```bash
/// AZUMI_AI_HASH=a]1b2c3d4e5f6
/// ```
///
/// # Usage in AI prompts
/// ```text
/// You are writing Azumi code. AZUMI_AI_HASH: {hash}
/// Verify the hash matches before generating code.
/// ```
pub const AZUMI_AI_HASH: &str = match option_env!("AZUMI_AI_HASH") {
    Some(v) => v,
    None => "dev-unconfigured",
};

/// Strict rules enforced by the framework. AI assistants should reference
/// these when generating Azumi code.
pub const AZUMI_RULES: &[&str] = &[
    "Text content MUST be quoted: <p>\"Hello\"</p>",
    "CSS values MUST be quoted: padding: \"1rem\";",
    "CSS classes MUST be snake_case: .my_class, NOT .my-class",
    "Static class=\"...\" is BANNED. Use class={variable}",
    "Static style=\"...\" is BANNED. Use style={--prop: val}",
    "Static id=\"...\" is BANNED. Use id={variable}",
    "Dashes are BANNED in CSS class/ID names",
    "<style> block MUST come AFTER the HTML structure",
    "Don't use @let for CSS class names — <style> creates variables automatically",
    "Use on:click={state.method} for event handlers",
    "Components use Props::builder() pattern",
    "State is HMAC-signed. Set AZUMI_SECRET for production",
    "For JSON data to JS: use json_data!(\"varname\" = &data), NOT format! or Raw()",
    "For CSS injection: use <style>{var}</style>, NOT Raw() with <style>",
    "For JS injection: use <script>{var}</script>, NOT Raw() with <script> — content is auto-escaped inside html!",
    "<style>{var}</style> and <script>{var}</script> auto-escape </style> and </script> sequences",
];

pub trait Component {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

/// Metadata for live state (predictions and namespacing)
/// Implemented for both the state struct and its references
pub trait LiveStateMetadata {
    /// Returns predictions for optimistic UI (method_name -> dsl)
    fn predictions() -> &'static [(&'static str, &'static str)];

    /// Returns the struct name for namespacing actions
    fn struct_name() -> &'static str;

    /// Returns the list of local field names (not serialized to az-scope)
    fn local_fields() -> &'static [&'static str] {
        &[]
    }

    /// Returns the list of computed field names (derived at runtime, never serialized)
    fn computed_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Marker trait for live state structs
/// Auto-implemented by `#[azumi::live]`
pub trait LiveState:
    LiveStateMetadata + serde::Serialize + for<'de> serde::de::Deserialize<'de> + Send + Sync + 'static
{
    fn to_scope(&self) -> String {
        let json = match serde_json::to_string(self) {
            Ok(j) => j,
            Err(e) => {
                panic!("FATAL: Failed to serialize LiveState to JSON: {}. \
                    This usually means a field doesn't implement Serialize. \
                    Check that all state fields implement serde::Serialize.", e);
            }
        };
        crate::security::sign_state(&json)
    }

    /// Attempt to serialize state to a signed scope string.
    /// Returns an error if serialization fails instead of panicking.
    ///
    /// Prefer this over `to_scope()` in production code where serialization
    /// failures should be handled gracefully rather than causing a panic.
    #[inline]
    fn try_to_scope(&self) -> Result<String, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(crate::security::sign_state(&json))
    }

    fn to_local_scope(&self) -> String {
        String::new()
    }
}

/// Runtime helper to look up a prediction for a method on a state
pub fn get_prediction<T: LiveStateMetadata>(_state: &T, method: &str) -> Option<&'static str> {
    T::predictions()
        .iter()
        .find(|(m, _)| *m == method)
        .map(|(_, p)| *p)
}

// Handle references for metadata (no Deserialize needed)
impl<T: LiveStateMetadata> LiveStateMetadata for &T {
    fn predictions() -> &'static [(&'static str, &'static str)] {
        T::predictions()
    }
    fn struct_name() -> &'static str {
        T::struct_name()
    }
    fn local_fields() -> &'static [&'static str] {
        T::local_fields()
    }
    fn computed_fields() -> &'static [&'static str] {
        T::computed_fields()
    }
}
impl<T: LiveStateMetadata> LiveStateMetadata for &mut T {
    fn predictions() -> &'static [(&'static str, &'static str)] {
        T::predictions()
    }
    fn struct_name() -> &'static str {
        T::struct_name()
    }
    fn local_fields() -> &'static [&'static str] {
        T::local_fields()
    }
    fn computed_fields() -> &'static [&'static str] {
        T::computed_fields()
    }
}

#[derive(Clone)]
pub struct FnComponent<F>(F);

/// A component backed by a function.
///
/// # Thread Safety
///
/// `FnComponent<F>` does **not** automatically implement `Send` or `Sync`
/// because the inner function `F` may capture non-`Send` state (e.g., `Rc`,
/// `RefCell`, or closures capturing thread-local data).
///
/// If you need `FnComponent` to be `Send + Sync`, ensure your function `F`
/// captures only `Send + Sync` types, or use `Arc<Mutex<...>>` for shared state.
///
/// # Example
///
/// ```ignore
/// // This FnComponent IS Send + Sync because the closure captures nothing
/// let comp = FnComponent::new(|f| write!(f, "Hello"));
///
/// // This FnComponent is NOT Send + Sync because it captures Rc
/// let data = std::rc::Rc::new("Hello".to_string());
/// let comp = FnComponent::new(move |f| write!(f, "{}", data)); // Not Send!
/// ```
impl<F> Component for FnComponent<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}

impl<F: Send + Sync> FnComponent<F> {
    /// Create a new `FnComponent` with a `Send + Sync` function.
    ///
    /// Use this when you need `FnComponent` to be thread-safe.
    pub fn new(f: F) -> Self {
        FnComponent(f)
    }
}

impl<T: Component + ?Sized> Component for &T {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

impl<T: Component + ?Sized> Component for Box<T> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

impl<T: Component + ?Sized> Component for std::rc::Rc<T> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

impl<T: Component + ?Sized> Component for std::sync::Arc<T> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).render(f)
    }
}

impl Component for String {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Escaped(self))
    }
}

impl Component for str {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Escaped(self))
    }
}

#[must_use]
#[doc(hidden)]
pub fn from_fn<F>(f: F) -> FnComponent<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    FnComponent(f)
}

/// A component backed by a `FnOnce` closure that can consume captured values.
///
/// This is useful when you need to move owned values into a component's children
/// closure, such as when both props AND children reference the same owned value.
///
/// # When to Use `FnOnceComponent`
///
/// Use `from_fn_once` when you have a value that needs to be used in both:
/// - Component props (which consume the value)
/// - Children body (which also needs ownership)
///
/// With `from_fn`, this would fail because `Fn` closures cannot move captured values.
/// With `from_fn_once`, the closure is called once and its result is cached.
///
/// # Thread Safety
///
/// `FnOnceComponent` is **NOT** `Send` or `Sync` because:
/// - `FnOnce` closures may capture non-Send types (`Rc`, `RefCell`, etc.)
/// - The internal caching uses `std::cell::OnceCell` which is not Sync
///
/// If you need thread-safety, use `from_fn` with `Arc<Mutex<T>>` or `Arc<RwLock<T>>`.
///
/// # Example
///
/// ```ignore
/// let owned_data = String::from("hello");
///
/// // This works - move owned_data into both props AND children closure
/// let component = FnOnceComponent::from_fn_once(move |f| {
///     write!(f, "<div>{}</div>", owned_data)  // owned_data is consumed here
/// });
/// ```
///
/// # Difference from `FnComponent`
///
/// | Aspect | `FnComponent` | `FnOnceComponent` |
/// |--------|----------------|-------------------|
/// | Closure trait | `Fn` | `FnOnce` |
/// | Can move values | No | Yes |
/// | Can be called multiple times | Yes (same result) | Yes (cached: first call only) |
/// | Interior mutability | None | `RefCell` |
/// | Thread-safe | Depends on captured types | Never |
///
/// # Limitations
///
/// - The closure can only be invoked **once** - subsequent renders return cached empty result
/// - This is designed for children closures in `html!` macros where the closure
///   is typically rendered exactly once during parent's render
pub struct FnOnceComponent<F>
where
    F: FnOnce(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    closure: std::cell::RefCell<Option<F>>,
}

impl<F> FnOnceComponent<F>
where
    F: FnOnce(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    /// Create a new `FnOnceComponent` from a `FnOnce` closure.
    ///
    /// The closure will be invoked on the first call to `render()`.
    /// Subsequent calls will be no-ops (return Ok(())).
    #[doc(hidden)]
    pub fn from_fn_once(f: F) -> Self {
        FnOnceComponent {
            closure: std::cell::RefCell::new(Some(f)),
        }
    }
}

impl<F> Component for FnOnceComponent<F>
where
    F: FnOnce(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Take ownership of the closure (first call) or return warning (subsequent calls).
        // RefCell tracks borrow state at runtime, eliminating the need for UnsafeCell.
        // Mark as consumed before calling (in case of panic, we don't want to retry).
        if let Some(c) = self.closure.borrow_mut().take() {
            c(f)
        } else {
            f.write_str("<!-- Azumi Warning: FnOnceComponent rendered more than once -->")
        }
    }
}

// NOTE: FnOnceComponent does NOT implement Send or Sync.
// This is intentional because:
// 1. FnOnce closures may capture non-Send types (like Rc)
// 2. RefCell is not thread-safe
// 3. Calling render() from multiple threads simultaneously would panic
//
// If you need thread-safety, use FnComponent with Arc<Mutex<T>> or Arc<RwLock<T>>.

#[doc(hidden)]
#[must_use]
pub fn from_fn_once<F>(f: F) -> FnOnceComponent<F>
where
    F: FnOnce(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    FnOnceComponent::from_fn_once(f)
}

#[inline]
#[must_use]
pub fn render_to_string<C: Component + ?Sized>(component: &C) -> String {
    struct DisplayWrapper<'a, C: Component + ?Sized>(&'a C);
    impl<'a, C: Component + ?Sized> std::fmt::Display for DisplayWrapper<'a, C> {
 fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
 self.0.render(_f)
        }
    }
    format!("{}", DisplayWrapper(component))
}

/// Render a component directly to a `Write` implementation.
///
/// Avoids the intermediate `String` allocation of `render_to_string`.
/// Useful for writing directly to a response body or buffer.
///
/// # Performance
///
/// For small components, `render_to_string` may be slightly faster due to
/// the `fmt::Write` → `io::Write` adapter overhead. For large pages with
/// many concurrent requests, `render_to_writer` avoids allocating a full
/// intermediate `String`, reducing memory pressure.
///
/// Benchmarks (release mode):
/// - Small component (1 div): `render_to_string` ~150ns, `render_to_writer` ~150ns
/// - Component with style: `render_to_string` ~340ns, `render_to_writer` ~580ns
/// - 1000 divs bulk: both ~124µs (memory advantage for `render_to_writer`)
///
/// # Example
///
/// ```rust,ignore
/// use azumi::render_to_writer;
///
/// let mut buf = Vec::new();
/// render_to_writer(&my_component, &mut buf)?;
/// // buf now contains the rendered HTML as bytes
/// ```
pub fn render_to_writer<C: Component + ?Sized, W: std::io::Write>(
    component: &C,
    writer: &mut W,
) -> std::io::Result<()> {
    use std::fmt::Write;
    struct IoAdapter<'a, W: std::io::Write> {
        inner: &'a mut W,
    }

    impl<W: std::io::Write> std::fmt::Write for IoAdapter<'_, W> {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.inner.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
        }
    }

    struct FmtToIoComponent<'a, C: Component + ?Sized>(&'a C);
    impl<C: Component + ?Sized> std::fmt::Display for FmtToIoComponent<'_, C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.render(f)
        }
    }

    let mut adapter = IoAdapter { inner: writer };
    write!(adapter, "{}", FmtToIoComponent(component)).map_err(|_| std::io::Error::new(
        std::io::ErrorKind::Other,
        "failed to render component to writer",
    ))
}

pub struct Escaped<T: std::fmt::Display>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for Escaped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        struct Escaper<'a, 'b>(&'a mut std::fmt::Formatter<'b>);
        impl std::fmt::Write for Escaper<'_, '_> {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                for c in s.chars() {
                    match c {
                        '<' => self.0.write_str("&lt;")?,
                        '>' => self.0.write_str("&gt;")?,
                        '&' => self.0.write_str("&amp;")?,
                        '"' => self.0.write_str("&quot;")?,
                        '\'' => self.0.write_str("&#x27;")?,
                        _ => {
                            let mut buf = [0u8; 4];
                            self.0.write_str(c.encode_utf8(&mut buf))?;
                        }
                    }
                }
                Ok(())
            }
        }
        write!(Escaper(f), "{}", self.0)
    }
}

/// Escape a string for safe inclusion in a CSS property value.
/// Prevents CSS injection by escaping semicolons, backslashes, braces, quotes, and forward slashes.
/// Forward slashes are escaped to prevent `</style>` injection attacks.
#[inline]
#[must_use]
pub fn escape_css_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            ';' | '\\' | '{' | '}' => {
                result.push('\\');
                result.push(c);
            }
            '"' => result.push_str("\\\""),
            '\'' => result.push_str("\\'"),
            '/' => result.push_str("\\/"),
            '\n' => result.push_str("\\a "),
            '\r' => result.push_str("\\d "),
            '\t' => result.push_str("\\9 "),
            '\0' => result.push_str("\\0 "),
            _ => result.push(c),
        }
    }
    result
}

// Smart Interpolation Machinery
// Allows {} to handle both Components (render) and Display types (escape)

pub struct RenderWrapper<T>(pub T);

impl<T: Component> RenderWrapper<T> {
    // Priority 1: Component (Render directly)
    // This inherent method takes precedence over the trait implementation below
    pub fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.render(f)
    }
}

pub trait FallbackRender {
    fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

// Priority 2: Display (Escape HTML)
impl<T: std::fmt::Display> FallbackRender for RenderWrapper<T> {
    fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Escaped(&self.0))
    }
}

/// **INTERNAL FRAMEWORK USE ONLY**
///
/// A wrapper to inject raw HTML/JS content without escaping.
///
/// **WARNING: XSS Vulnerability Risk**
///
/// This wrapper bypasses ALL Azumi escaping protections. It is intended
/// for internal framework use only (e.g., SEO generation, macro internals).
///
/// **For user code, use the safe alternatives:**
/// - `json_data!("var" = &data)` for JSON data
/// - `<style>{var}</style>` for CSS content (auto-escaped inside html!)
/// - `<script>{var}</script>` for JavaScript content (auto-escaped inside html!)
/// - `{value}` interpolation for text (auto-escaped)
///
/// **Never use Raw with:**
/// - User input (form data, URL parameters, cookies)
/// - Dynamic content from databases
/// - Content from external APIs
///
/// Raw in html! expressions is now a compile-time error.
#[doc(hidden)]
pub struct Raw<T: std::fmt::Display>(T);

impl<T: std::fmt::Display> Raw<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub(crate) fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Raw<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: std::fmt::Display> Component for Raw<T> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Schema.org JSON-LD Support (Optional Feature)
// ============================================================================

#[cfg(feature = "schema")]
pub use azumi_macros::Schema;

#[cfg(feature = "schema")]
pub trait Schema {
    /// Generate a complete <script type="application/ld+json"> tag
    fn to_schema_script(&self) -> String;

    /// Generate just the JSON value (for recursive nesting)
    fn to_schema_json_value(&self) -> serde_json::Value;
}

#[cfg(test)]
mod tests;

// ============================================================================
// Embedded Client Runtime
// ============================================================================

/// The Azumi client library (embedded at compile time)
/// This includes Idiomorph (DOM morphing) and the Azumi coordinator
pub const AZUMI_JS: &str = include_str!("client.min.js");

/// Helper to generate the `<script>` tag for the client runtime
///
/// Returns an AzumiScript Component that renders the Azumi client runtime.
/// Usage: html! { <head> { azumi::azumi_script() } </head> }
///
/// This is a Component, not a String. Use {azumi_script()} syntax (not @{azumi_script()})
/// to render it directly without escaping.
#[must_use]
pub fn azumi_script() -> AzumiScript {
    AzumiScript
}

pub struct HotReloadClosure<'a>(pub &'a dyn Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result);

impl<'a> FallbackRender for HotReloadClosure<'a> {
    fn render_azumi(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}

// ── Route declaration macro ──────────────────────────────────────────────

/// Declare page routes with clean syntax.
///
/// Eliminates the boilerplate of `Router::new().route(...)` chains.
/// Each entry maps a URL path to an Axum handler function.
///
/// # Example
///
/// ```rust,ignore
/// use azumi::routes;
///
/// let app = routes! {
///     "/"         => home_handler,
///     "/about"    => about_handler,
///     "/counter"  => counter_handler,
/// }
/// .merge(azumi::action::register_actions(Router::new()))
/// .merge(azumi::devtools::router());
/// ```
///
/// This is purely syntactic sugar — it expands to `Router::new().route(...)` chains.
/// Each handler must be a valid Axum handler (async fn returning impl IntoResponse).
#[macro_export]
macro_rules! routes {
    ($($path:expr => $handler:expr),+ $(,)?) => {{
        let mut _azumi_router = $crate::__private::axum::Router::new();
        $(
            _azumi_router = _azumi_router.route(
                $path,
                $crate::__private::axum::routing::get($handler),
            );
        )+
        _azumi_router
    }};
}
