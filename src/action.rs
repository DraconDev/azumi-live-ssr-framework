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

/// Escape HTML entities to prevent XSS in action fragments.
fn escape_html(s: &str) -> String {
    crate::escape_html(s)
}

/// Escape a string for safe inclusion in a JavaScript string literal.
/// Escapes: backslash, backtick, quotes, newline, carriage return, angle brackets.
/// Backticks prevent template literal injection. Angle brackets prevent HTML breakout
/// when the JS string will be embedded in an HTML attribute.
fn escape_js_string(s: &str) -> String {
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
            _ => out.push(c),
        }
    }
    out
}

/// HTML fragment for successful form actions (az-target swapping).
///
/// Wraps content in a `<div class="success_message">` for standard error handling.
/// Use this when an `az-action` form succeeds and you want to swap in a success message.
///
/// # Example
///
/// ```rust,ignore
/// async fn submit_form() -> impl axum::response::IntoResponse {
///     azumi::action::success_fragment("<p>Saved!</p>")
/// }
/// ```
pub fn success_fragment(html: impl Into<String>) -> Response {
    let safe_html = escape_html(&html.into());
    axum::response::Html(format!(
        r#"<div class="success_message">{}</div>"#,
        safe_html
    ))
    .into_response()
}

/// HTML fragment for failed form actions (az-target swapping).
///
/// Wraps content in a `<div class="error_message">` with optional retry button.
/// If `form_id` is provided, includes a "Try Again" button that re-shows the form.
///
/// # Escaping Order
///
/// When `form_id` is provided, it is escaped in two steps:
/// 1. `escape_js_string(id)` — escapes `\`, `/`, `*`, `"`, `'`, `;`, `<`, `>`, `` ` ``
/// 2. `escape_html(&safe_id)` — escapes `&`, `<`, `>`, `"`, `'` for HTML context
///
/// This matters because `form_id` goes into an HTML attribute (`id="..."`) which is
/// itself inside a JavaScript string literal inside an `onclick` attribute handler.
/// The double-escape ensures the value is safe in both contexts.
pub fn error_fragment(message: impl Into<String>, form_id: Option<&str>) -> Response {
    let msg = escape_html(&message.into());
    let retry = form_id.map(|id| {
        // Escape for BOTH JavaScript and HTML contexts since this goes into an HTML attribute
        let safe_id = escape_js_string(id);
        let safe_id_html = escape_html(&safe_id);
        format!(
            r#"<button type="button" onclick="document.getElementById('{}').style.display='flex';this.parentElement.remove()" class="submit_btn" style="margin-top:1rem">Try Again</button>"#,
            safe_id_html
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
