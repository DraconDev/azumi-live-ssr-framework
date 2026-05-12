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

use std::future::Future;

/// Trait for Azumi Actions
/// This is implemented automatically by the `#[azumi::action]` macro
#[allow(dead_code)]
pub trait Action<Input, Output> {
    fn call(input: Input) -> impl Future<Output = Output> + Send;
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

/// Helper to wrap an action result into an Axum response with correct Content-Type
pub async fn handle_action_result<C: Component + ?Sized>(component: &C) -> impl IntoResponse {
    axum::response::Html(crate::render_to_string(component))
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
    axum::response::Html(format!(
        r#"<div class="success_message">{}</div>"#,
        html.into()
    ))
    .into_response()
}

/// HTML fragment for failed form actions (az-target swapping).
///
/// Wraps content in a `<div class="error_message">` with optional retry button.
/// If `form_id` is provided, includes a "Try Again" button that re-shows the form.
///
/// # Example
///
/// ```rust,ignore
/// async fn submit_form() -> impl axum::response::IntoResponse {
///     azumi::action::error_fragment("Invalid email", None)
/// }
/// ```
pub fn error_fragment(message: impl Into<String>, form_id: Option<&str>) -> Response {
    let msg = message.into();
    let retry = form_id.map(|id| {
        format!(
            r#"<button type="button" onclick="document.getElementById('{}').style.display='flex';this.parentElement.remove()" class="submit_btn" style="margin-top:1rem">Try Again</button>"#,
            id
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
