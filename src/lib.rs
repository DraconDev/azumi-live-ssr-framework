pub mod prelude {
    pub use crate::action::{Action, error_fragment, success_fragment};
    pub use crate::{
        action, azumi_script, component, html, json_data, live,
        session_cleanup_script, AzumiScript, Component, escape_css_string,
        from_fn, FnComponent,
    };
}

pub use azumi_macros::{
    action, component, head, html, json_data, live, live_impl, page,
    predict,
};
pub mod action;
pub mod context;
#[cfg(feature = "devtools")]
pub mod hot_reload;
pub mod script;
pub mod security;
pub use security::VerifyStateError;
pub use inventory;
#[cfg(feature = "devtools")]
pub mod devtools;

pub mod seo;
pub use script::{AzumiScript, escape_script_content, escape_style_content, escape_tag_content, session_cleanup_script};

// ── Re-exports for declarative macros ─────────────────────────────────────
/// Internal re-exports used by `azumi::routes!` and other declarative macros.
/// Not part of the public API.
#[doc(hidden)]
pub mod __private {
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
        // Take ownership of the closure (first call) or return (subsequent calls).
        // RefCell tracks borrow state at runtime, eliminating the need for UnsafeCell.
        // Mark as consumed before calling (in case of panic, we don't want to retry).
        if let Some(c) = self.closure.borrow_mut().take() {
            c(f)
        } else {
            Ok(())
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

pub struct Escaped<T: std::fmt::Display>(pub T);

impl<T: std::fmt::Display> std::fmt::Display for Escaped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{}", self.0);
        for c in s.chars() {
            match c {
                '<' => f.write_str("&lt;")?,
                '>' => f.write_str("&gt;")?,
                '&' => f.write_str("&amp;")?,
                '"' => f.write_str("&quot;")?,
                '\'' => f.write_str("&#x27;")?,
                _ => {
                    let mut buf = [0u8; 4];
                    f.write_str(c.encode_utf8(&mut buf))?;
                }
            }
        }
        Ok(())
    }
}

/// Escape a string for safe inclusion in a CSS property value.
/// Prevents CSS injection by escaping semicolons, backslashes, braces, quotes, and forward slashes.
/// Forward slashes are escaped to prevent `</style>` injection attacks.
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
pub struct Raw<T: std::fmt::Display>(pub T);

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

/// Compute a deterministic scope ID from source position (line, column).
/// Used by both the proc-macro and the hot reload watcher to guarantee
/// that scope IDs match at compile time and runtime.
#[must_use]
pub fn compute_scope_id(line: usize, col: usize) -> String {
    use fnv::FnvHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = FnvHasher::default();
    line.hash(&mut hasher);
    col.hash(&mut hasher);
    format!("s{:x}", hasher.finish())
}

/// Transform CSS selectors to include scope attribute
/// All CSS is automatically scoped - no escape hatches!
#[must_use]
pub fn scope_css(css: &str, scope_id: &str) -> String {
    let scope_attr = format!("[data-{}]", scope_id);
    let mut iter = css.chars().peekable();
    scope_css_level(&mut iter, &scope_attr, false)
}

fn scope_css_level(
    iter: &mut std::iter::Peekable<std::str::Chars>,
    scope_attr: &str,
    finding_close: bool,
) -> String {
    let mut result = String::new();
    let mut buffer = String::new();

    while let Some(ch) = iter.next() {
        match ch {
            '{' => {
                let selector_raw = buffer.trim().to_string();

                if is_grouping_rule(&selector_raw) {
                    result.push_str(&buffer);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&scope_css_level(iter, scope_attr, true));
                    result.push('}');
                } else if is_keyframes(&selector_raw) {
                    result.push_str(&buffer);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                } else {
                    let scoped_selector_str = if selector_raw.starts_with('@') {
                        selector_raw.to_string()
                    } else {
                        let selectors: Vec<&str> = split_selector_list(&selector_raw);
                        selectors
                            .iter()
                            .filter(|s| !s.trim().is_empty())
                            .map(|s| scope_selector(s.trim(), scope_attr))
                            .collect::<Vec<_>>()
                            .join(", ")
                    };

                    result.push_str(&scoped_selector_str);
                    result.push('{');
                    buffer.clear();
                    result.push_str(&extract_balanced_block(iter));
                    result.push('}');
                }
            }
            '}' => {
                if finding_close {
                    result.push_str(&buffer);
                    return result;
                }
                buffer.push(ch);
            }
            ';' => {
                buffer.push(ch);
                result.push_str(&buffer);
                buffer.clear();
            }
            _ => {
                buffer.push(ch);
            }
        }
    }
    result.push_str(&buffer);
    result
}

fn is_grouping_rule(s: &str) -> bool {
    s.starts_with("@media")
        || s.starts_with("@supports")
        || s.starts_with("@layer")
        || s.starts_with("@container")
}

fn is_keyframes(s: &str) -> bool {
    s.starts_with("@keyframes") || s.starts_with("@-webkit-keyframes")
}

fn extract_balanced_block(iter: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut content = String::new();
    let mut depth = 1;
    for ch in iter.by_ref() {
        match ch {
            '{' => {
                depth += 1;
                content.push(ch);
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return content;
                }
                content.push(ch);
            }
            _ => content.push(ch),
        }
    }
    content
}

fn split_selector_list(selector_raw: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut paren_depth: usize = 0;
    let mut bracket_depth: usize = 0;
    let mut last_start = 0;
    for (byte_idx, ch) in selector_raw.char_indices() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ',' if paren_depth == 0 && bracket_depth == 0 => {
                let sel = selector_raw[last_start..byte_idx].trim();
                if !sel.is_empty() {
                    result.push(sel);
                }
                last_start = byte_idx + ch.len_utf8();
            }
            _ => {}
        }
    }
    let last = selector_raw[last_start..].trim();
    if !last.is_empty() {
        result.push(last);
    }
    result
}

fn scope_selector(selector: &str, scope_attr: &str) -> String {
    if selector.starts_with('@') || selector.starts_with("/*") {
        return selector.to_string();
    }
    if selector.starts_with(":host") || selector.starts_with("::slotted") || selector.starts_with("::part") {
        return selector.to_string();
    }
    
    fn find_last_real_colon(s: &str) -> Option<usize> {
        let mut bracket_depth = 0usize;
        let mut last_colon = None;
        
        for (i, ch) in s.char_indices() {
            match ch {
                '[' => bracket_depth = bracket_depth.saturating_add(1),
                ']' => bracket_depth = bracket_depth.saturating_sub(1),
                ':' if bracket_depth == 0 => { last_colon = Some(i); }
                _ => {}
            }
        }
        last_colon
    }
    
    if let Some(pseudo_pos) = selector.find("::") {
        let base_and_pseudos = &selector[..pseudo_pos];
        let pseudo_element = &selector[pseudo_pos..];
        if let Some(class_pos) = find_last_real_colon(base_and_pseudos) {
            let base = &base_and_pseudos[..class_pos];
            let pseudo_classes = &base_and_pseudos[class_pos..];
            return format!("{}{}{}{}", base, pseudo_classes, scope_attr, pseudo_element);
        }
        return format!("{}{}{}", base_and_pseudos, scope_attr, pseudo_element);
    }
    if let Some(pseudo_pos) = find_last_real_colon(selector) {
        let base = &selector[..pseudo_pos];
        let pseudo = &selector[pseudo_pos..];
        return format!("{}{}{}", base, scope_attr, pseudo);
    }
    format!("{}{}", selector, scope_attr)
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
        let mut __azumi_router = $crate::__private::axum::Router::new();
        $(
            __azumi_router = __azumi_router.route(
                $path,
                $crate::__private::axum::routing::get($handler),
            );
        )+
        __azumi_router
    }};
}
