//! # Azumi Actions Module
//!
//! ## CSRF Protection for Actions
//!
//! `#[azumi::action]` handlers are plain JSON API endpoints. CSRF protection
//! is NOT built into Azumi because:
//!
//! 1. **Azumi doesn't own auth** - Authentication is application responsibility
//! 2. **Actions are JSON APIs** - They're designed to be called from the JS client
//!
//! ### CSRF Protection Options
//!
//! For applications using cookie-based authentication, add CSRF protection at the
//! Axum middleware layer (not in Azumi). Common approaches:
//!
//! 1. **SameSite cookies** - Set `SameSite=Strict` or `SameSite=Lax` on session cookies
//! 2. **Double Submit Cookie** - Check a CSRF token in both cookie and header
//! 3. **Custom Axum middleware** - Verify Origin/Referer headers
//!
//! ### LiveView State Protection
//!
//! `#[azumi::live_impl]` handlers are protected against CSRF by HMAC-signed state.
//! The state is generated server-side with a timestamp and signature - attackers
//! cannot forge valid state without the secret key.

use crate::Component;
use axum::response::{IntoResponse, Response};
use axum::routing::get;

/// Result type for Azumi actions.
///
/// On success, returns HTML rendered from a Component.
/// On error, returns an error message with optional form ID for retry UI.
///
/// # Example
///
/// ```rust,ignore
/// #[azumi::action]
/// pub async fn save(form: SaveForm) -> ActionResult {
///     if form.name.is_empty() {
///         return ActionResult::err("Name is required");
///     }
///     ActionResult::ok(html! { <div>"Saved!"</div> })
/// }
/// ```
pub enum ActionResult {
    Ok(String),
    Err { message: String, form_id: Option<String> },
    Redirect(String),
}

impl ActionResult {
    /// Create a success result from any Component.
    #[must_use = "ActionResult must be returned from a handler to produce a response"]
    pub fn ok<C: Component + ?Sized>(component: &C) -> Self {
        Self::Ok(crate::render_to_string(component))
    }

    /// Create an error result.
    #[must_use = "ActionResult must be returned from a handler to produce a response"]
    pub fn err(message: impl Into<String>) -> Self {
        Self::Err {
            message: message.into(),
            form_id: None,
        }
    }

    /// Create an error result with a form ID for retry UI.
    #[must_use = "ActionResult must be returned from a handler to produce a response"]
    pub fn err_with_form(message: impl Into<String>, form_id: impl Into<String>) -> Self {
        Self::Err {
            message: message.into(),
            form_id: Some(form_id.into()),
        }
    }

    /// Create a redirect result.
    #[must_use = "ActionResult must be returned from a handler to produce a response"]
    pub fn redirect(url: impl Into<String>) -> Self {
        Self::Redirect(url.into())
    }
}

impl IntoResponse for ActionResult {
    fn into_response(self) -> Response {
        match self {
            Self::Ok(html) => axum::response::Html(html).into_response(),
            Self::Err { message, form_id } => error_fragment(message, form_id.as_deref()),
            Self::Redirect(url) => axum::response::Redirect::to(&url).into_response(),
        }
    }
}

use axum::routing::MethodRouter;

/// Registry entry for an action
pub struct ActionEntry {
    pub path: &'static str,
    pub handler: fn() -> MethodRouter<()>,
}

inventory::collect!(ActionEntry);

/// Register all collected actions into the router.
/// Also registers the `/azumi.js` route to serve the client runtime.
pub fn register_actions(mut router: axum::Router) -> axum::Router {
    for entry in inventory::iter::<ActionEntry> {
        router = router.route(entry.path, (entry.handler)());
    }
    router.route("/azumi.js", get(azumi_js_handler))
}

/// Handler that serves the embedded Azumi client JavaScript
async fn azumi_js_handler() -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "application/javascript")],
        crate::AZUMI_JS,
    )
}

/// Escape a string for safe inclusion in a JavaScript string literal.
/// Escapes: backslash, backtick, quotes, newline, carriage return, angle brackets,
/// forward slash, and semicolon.
/// Backticks prevent template literal injection. Angle brackets and forward slash
/// prevent HTML breakout when the JS string will be embedded in an HTML attribute.
/// Semicolons prevent early statement termination in JS contexts.
#[must_use]
pub fn escape_js_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '`' => out.push_str("\\x60"),
            '"' => out.push_str("\\x22"),
            '\'' => out.push_str("\\x27"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '<' => out.push_str("\\x3c"),
            '>' => out.push_str("\\x3e"),
            '/' => out.push_str("\\/"),
            ';' => out.push_str("\\x3b"),
            _ => out.push(c),
        }
    }
    out
}

/// HTML fragment for successful form actions (az-target swapping).
///
/// Accepts any Component and renders it inside a `<div class="success_message">`.
/// Content is rendered as-is (not double-escaped) since Components already handle
/// their own escaping via `html!` interpolation.
///
/// # Migration from pre-v47
///
/// The signature changed from `impl Into<String>` to `&impl Component`.
/// **Old code that passed a raw string will need a small adjustment:**
///
/// ```rust,ignore
/// // Before (v46): success_fragment("<p>Saved!</p>".to_string())
/// // After  (v47): success_fragment(&html! { <p>"Saved!"</p> })
/// ```
///
/// For simple text, `&str` implements `Component` so `success_fragment("ok")` still works.
///
/// # Example
///
/// ```rust,ignore
/// async fn submit_form() -> impl axum::response::IntoResponse {
///     azumi::action::success_fragment(html! { <p>"Saved!"</p> })
/// }
/// ```
pub fn success_fragment<C: Component + ?Sized>(component: &C) -> Response {
    let html = crate::render_to_string(component);
    axum::response::Html(format!(
        r#"<div class="success_message">{}</div>"#,
        html
    ))
    .into_response()
}

/// HTML fragment for failed form actions (az-target swapping).
///
/// Wraps content in a `<div class="error_message">` with optional retry button.
/// If `form_id` is provided, includes a "Try Again" button that re-shows the form.
///
/// The `form_id` is HTML-escaped and placed in a `data-retry-form` attribute.
/// The button uses `az-on="click call __azumi_retry"` for framework-consistent
/// event delegation, which the Azumi client runtime handles as a built-in action.
pub fn error_fragment(message: impl Into<String>, form_id: Option<&str>) -> Response {
    let msg = crate::escape_html(&message.into());
    let retry = form_id.map(|id| {
        let safe_id = crate::escape_html(id);
        format!(
            r#"<button type="button" az-on="click call __azumi_retry -> .error_message" data-retry-form="{}" class="submit_btn" style="margin-top:1rem">Try Again</button>"#,
            safe_id
        )
    });

    axum::response::Html(match retry {
        Some(btn) => format!(
            r#"<div class="error_message"><p class="error_text">{}</p>{}</div>"#,
            msg, btn
        ),
        None => format!(
            r#"<div class="error_message"><p class="error_text">{}</p></div>"#,
            msg
        ),
    })
    .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_js_string_backslash() {
        assert_eq!(escape_js_string(r"a\b"), r"a\\b");
    }

    #[test]
    fn test_escape_js_string_backtick() {
        assert_eq!(escape_js_string("a`b"), r"a\x60b");
    }

    #[test]
    fn test_escape_js_string_double_quote() {
        assert_eq!(escape_js_string("a\"b"), r"a\x22b");
    }

    #[test]
    fn test_escape_js_string_single_quote() {
        assert_eq!(escape_js_string("a'b"), r"a\x27b");
    }

    #[test]
    fn test_escape_js_string_newline() {
        assert_eq!(escape_js_string("a\nb"), r"a\nb");
    }

    #[test]
    fn test_escape_js_string_carriage_return() {
        assert_eq!(escape_js_string("a\rb"), r"a\rb");
    }

    #[test]
    fn test_escape_js_string_angle_brackets() {
        assert_eq!(escape_js_string("<script>"), r"\x3cscript\x3e");
    }

    #[test]
    fn test_escape_js_string_forward_slash() {
        assert_eq!(escape_js_string("a/b"), r"a\/b");
    }

    #[test]
    fn test_escape_js_string_semicolon() {
        assert_eq!(escape_js_string("a;b"), r"a\x3bb");
    }

    #[test]
    fn test_escape_js_string_noop() {
        assert_eq!(escape_js_string("hello world"), "hello world");
    }
}
