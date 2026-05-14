//! Content-Security-Policy helpers for Azumi applications.
//!
//! Provides a builder for constructing CSP headers that work well with
//! Azumi's zero-hydration architecture and ~3KB runtime.
//!
//! # Example (static CSP)
//!
//! ```rust,ignore
//! use azumi::csp::ContentSecurityPolicy;
//!
//! let csp = ContentSecurityPolicy::new()
//!     .default_src("'self'")
//!     .script_src("'self'")
//!     .style_src("'self' 'unsafe-inline'")
//!     .img_src("'self' data:")
//!     .form_action("'self'")
//!     .build();
//!
//! // Use with Axum:
//! // ([("content-security-policy", csp)], body)
//! ```
//!
//! # Example (nonce-based CSP)
//!
//! For stronger XSS protection, use per-request nonces instead of `'unsafe-inline'`:
//!
//! ```rust,ignore
//! use azumi::csp::{CspNonce, ContentSecurityPolicy};
//!
//! // In your Axum handler:
//! async fn home_handler(nonce: CspNonce) -> impl IntoResponse {
//!     let csp = ContentSecurityPolicy::azumi_nonce_defaults(&nonce)
//!         .build();
//!
//!     // Access nonce for <style> tags:
//!     // <style nonce={nonce.as_str()}>
//!
//!     (
//!         [("content-security-policy", csp)],
//!         axum::response::Html(render_to_string(&HomePage))
//!     )
//! }
//! ```

use std::fmt;

/// Builder for Content-Security-Policy header values.
///
/// CSP prevents XSS by restricting which resources the browser can load.
/// This builder provides a fluent API for constructing policy strings.
#[derive(Debug, Clone)]
pub struct ContentSecurityPolicy {
    directives: Vec<(String, String)>,
}

impl ContentSecurityPolicy {
    /// Create a new empty CSP builder.
    #[must_use]
    pub fn new() -> Self {
        ContentSecurityPolicy {
            directives: Vec::new(),
        }
    }

    /// Create a CSP with Azumi-recommended defaults:
    /// - `default-src 'self'`
    /// - `script-src 'self'` (Azumi's runtime is served from `/azumi.js`)
    /// - `style-src 'self' 'unsafe-inline'` (scoped styles need inline)
    /// - `img-src 'self' data:`
    /// - `form-action 'self'`
    /// - `base-uri 'self'`
    /// - `frame-ancestors 'none'`
    #[must_use]
    pub fn azumi_defaults() -> Self {
        Self::new()
            .default_src("'self'")
            .script_src("'self'")
            .style_src("'self' 'unsafe-inline'")
            .img_src("'self' data:")
            .form_action("'self'")
            .base_uri("'self'")
            .frame_ancestors("'none'")
    }

    /// Create a CSP with nonce-based defaults (no `'unsafe-inline'`).
    ///
    /// This replaces `'unsafe-inline'` in `style-src` and `script-src`
    /// with per-request nonces for stronger XSS protection.
    ///
    /// Requires that `<style>` and `<script>` tags include `nonce="{nonce}"`.
    #[must_use]
    pub fn azumi_nonce_defaults(nonce: &CspNonce) -> Self {
        Self::new()
            .default_src("'self'")
            .script_src(&format!("'self' 'nonce-{}'", nonce.0))
            .style_src(&format!("'self' 'nonce-{}'", nonce.0))
            .img_src("'self' data:")
            .form_action("'self'")
            .base_uri("'self'")
            .frame_ancestors("'none'")
    }

    fn add_directive(mut self, name: &str, value: &str) -> Self {
        self.directives.push((name.to_string(), value.to_string()));
        self
    }

    #[must_use]
    pub fn default_src(self, value: &str) -> Self {
        self.add_directive("default-src", value)
    }

    #[must_use]
    pub fn script_src(self, value: &str) -> Self {
        self.add_directive("script-src", value)
    }

    #[must_use]
    pub fn style_src(self, value: &str) -> Self {
        self.add_directive("style-src", value)
    }

    #[must_use]
    pub fn img_src(self, value: &str) -> Self {
        self.add_directive("img-src", value)
    }

    #[must_use]
    pub fn font_src(self, value: &str) -> Self {
        self.add_directive("font-src", value)
    }

    #[must_use]
    pub fn connect_src(self, value: &str) -> Self {
        self.add_directive("connect-src", value)
    }

    #[must_use]
    pub fn media_src(self, value: &str) -> Self {
        self.add_directive("media-src", value)
    }

    #[must_use]
    pub fn object_src(self, value: &str) -> Self {
        self.add_directive("object-src", value)
    }

    pub fn frame_src(self, value: &str) -> Self {
        self.add_directive("frame-src", value)
    }

    pub fn form_action(self, value: &str) -> Self {
        self.add_directive("form-action", value)
    }

    pub fn base_uri(self, value: &str) -> Self {
        self.add_directive("base-uri", value)
    }

    pub fn frame_ancestors(self, value: &str) -> Self {
        self.add_directive("frame-ancestors", value)
    }

    pub fn upgrade_insecure_requests(mut self) -> Self {
        self.directives
            .push(("upgrade-insecure-requests".to_string(), String::new()));
        self
    }

    /// Build the CSP header value string.
    pub fn build(&self) -> String {
        self.directives
            .iter()
            .map(|(name, value)| {
                if value.is_empty() {
                    name.clone()
                } else {
                    format!("{} {}", name, value)
                }
            })
            .collect::<Vec<_>>()
            .join("; ")
    }
}

impl Default for ContentSecurityPolicy {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Nonce-based CSP
// ============================================================================

/// A cryptographically random CSP nonce for per-request Content-Security-Policy.
///
/// Generated once per request and inserted into CSP headers and HTML attributes.
/// Browsers only execute `<script nonce="...">` and `<style nonce="...">` tags
/// whose nonce matches the CSP header.
///
/// # Usage with Axum
///
/// ```rust,ignore
/// use azumi::csp::{CspNonce, ContentSecurityPolicy};
///
/// async fn handler(nonce: CspNonce) -> impl IntoResponse {
///     let csp = ContentSecurityPolicy::azumi_nonce_defaults(&nonce).build();
///     // Pass nonce to components via context or direct parameter
///     (
///         [("content-security-policy", csp)],
///         axum::response::Html(body)
///     )
/// }
/// ```
///
/// # Usage in html!
///
/// ```rust,ignore
/// html! {
///     <style nonce={nonce.as_str()}>
///         .my_class { color: "red"; }
///     </style>
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CspNonce(String);

impl CspNonce {
    /// Generate a new cryptographically random nonce.
    ///
    /// Uses 16 bytes of randomness base64-encoded (128 bits of entropy).
    /// This meets the [CSP spec recommendation](https://w3c.github.io/webappsec-csp/#security-nonce-size)
    /// of at least 128 bits.
    ///
    /// # Panics
    ///
    /// Panics if the system random number generator is unavailable.
    /// This is deliberate — a missing CSP nonce is a security downgrade.
    /// For environments where this may fail (e.g., early boot), use [`CspNonce::try_generate`].
    pub fn generate() -> Self {
        Self::try_generate().expect("failed to generate CSP nonce: system RNG unavailable")
    }

    /// Try to generate a nonce, returning `Err` if the system RNG is unavailable.
    ///
    /// Use this in environments where the RNG may not be available (e.g.,
    /// embedded, early boot, or sandboxed contexts). On success, the nonce
    /// has the same properties as [`CspNonce::generate`].
    pub fn try_generate() -> Result<Self, getrandom::Error> {
        use base64::engine::general_purpose::STANDARD;
        use base64::Engine;

        let mut bytes = [0u8; 16];
        getrandom::fill(&mut bytes)?;
        Ok(CspNonce(STANDARD.encode(bytes)))
    }

    /// Get the nonce value as a string for use in HTML attributes.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CspNonce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for CspNonce {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// Axum Middleware (gated behind "axum" feature)
// ============================================================================

#[cfg(feature = "axum")]
mod axum_middleware {
    use super::{ContentSecurityPolicy, CspNonce};
    use axum::{
        extract::{FromRequestParts, Request},
        http::{header, request::Parts, HeaderValue},
        middleware::Next,
        response::Response,
    };
    use std::convert::Infallible;

    /// Axum extractor for the CSP nonce.
    ///
    /// When the [`csp_nonce_layer`] middleware is active, handlers can extract
    /// `CspNonce` to get the per-request nonce for HTML attributes and CSP headers.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use azumi::csp::{CspNonce, ContentSecurityPolicy};
    ///
    /// async fn handler(nonce: CspNonce) -> impl IntoResponse {
    ///     let csp = ContentSecurityPolicy::azumi_nonce_defaults(&nonce).build();
    ///     (
    ///         [("content-security-policy", csp)],
    ///         axum::response::Html(body)
    ///     )
    /// }
    /// ```
    impl FromRequestParts<()> for CspNonce {
        type Rejection = Infallible;

        fn from_request_parts(
            parts: &mut Parts,
            _state: &(),
        ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
            let nonce = parts
                .extensions
                .get::<CspNonce>()
                .cloned()
                .unwrap_or_else(CspNonce::generate);
            std::future::ready(Ok(nonce))
        }
    }

    /// Axum middleware layer that generates a per-request CSP nonce and
    /// injects a `Content-Security-Policy` response header.
    ///
    /// The nonce is available to handlers via the [`CspNonce`] extractor.
    ///
    /// # Default Policy
    ///
    /// Uses [`ContentSecurityPolicy::azumi_nonce_defaults`] which replaces
    /// `'unsafe-inline'` with nonce-based directives. Customize with
    /// [`csp_nonce_layer_with`].
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use azumi::csp::csp_nonce_layer;
    ///
    /// let app = Router::new()
    ///     .route("/", home_handler)
    ///     .layer(csp_nonce_layer());
    /// ```
    pub fn csp_nonce_layer() -> impl Clone + Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> {
        csp_nonce_layer_with(|nonce| ContentSecurityPolicy::azumi_nonce_defaults(nonce))
    }

    /// Axum middleware layer with a custom CSP policy builder.
    ///
    /// The closure receives the generated nonce and returns a `ContentSecurityPolicy`.
    /// This allows full customization of directives while still using per-request nonces.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use azumi::csp::{csp_nonce_layer_with, ContentSecurityPolicy, CspNonce};
    ///
    /// let app = Router::new()
    ///     .route("/", home_handler)
    ///     .layer(csp_nonce_layer_with(|nonce| {
    ///         ContentSecurityPolicy::azumi_nonce_defaults(nonce)
    ///             .connect_src("'self' ws://localhost:8080")
    ///     }));
    /// ```
    pub fn csp_nonce_layer_with<F>(
        build_csp: F,
    ) -> impl Clone + Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
    where
        F: Fn(&CspNonce) -> ContentSecurityPolicy + Clone + Send + Sync + 'static,
    {
        move |req: Request, next: Next| {
            let build_csp = build_csp.clone();
            Box::pin(async move {
                let nonce = CspNonce::generate();
                let csp_value = build_csp(&nonce).build();

                let (mut parts, body) = req.into_parts();
                parts.extensions.insert(nonce.clone());

                let req = Request::from_parts(parts, body);
                let mut response = next.run(req).await;

                if let Ok(header_value) = HeaderValue::from_str(&csp_value) {
                    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, header_value);
                }

                response
            })
        }
    }
}

#[cfg(feature = "axum")]
pub use axum_middleware::{csp_nonce_layer, csp_nonce_layer_with};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_csp() {
        let csp = ContentSecurityPolicy::new().build();
        assert_eq!(csp, "");
    }

    #[test]
    fn test_single_directive() {
        let csp = ContentSecurityPolicy::new().default_src("'self'").build();
        assert_eq!(csp, "default-src 'self'");
    }

    #[test]
    fn test_multiple_directives() {
        let csp = ContentSecurityPolicy::new()
            .default_src("'self'")
            .script_src("'self'")
            .style_src("'self' 'unsafe-inline'")
            .build();
        assert_eq!(csp, "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'");
    }

    #[test]
    fn test_azumi_defaults() {
        let csp = ContentSecurityPolicy::azumi_defaults().build();
        assert!(csp.contains("default-src 'self'"));
        assert!(csp.contains("script-src 'self'"));
        assert!(csp.contains("style-src 'self' 'unsafe-inline'"));
        assert!(csp.contains("form-action 'self'"));
        assert!(csp.contains("frame-ancestors 'none'"));
    }

    #[test]
    fn test_upgrade_insecure() {
        let csp = ContentSecurityPolicy::new()
            .default_src("'self'")
            .upgrade_insecure_requests()
            .build();
        assert!(csp.contains("upgrade-insecure-requests"));
    }

    #[test]
    fn test_custom_connect_src() {
        let csp = ContentSecurityPolicy::azumi_defaults()
            .connect_src("'self' ws://localhost:8080")
            .build();
        assert!(csp.contains("connect-src 'self' ws://localhost:8080"));
    }

    #[test]
    fn test_builder_is_chainable() {
        let csp = ContentSecurityPolicy::new()
            .default_src("'self'")
            .script_src("'self' 'unsafe-eval'")
            .style_src("'self'")
            .img_src("'self' data: https:")
            .font_src("'self' https://fonts.gstatic.com")
            .form_action("'self'")
            .build();

        assert!(csp.contains("default-src 'self'"));
        assert!(csp.contains("script-src 'self' 'unsafe-eval'"));
        assert!(csp.contains("font-src 'self' https://fonts.gstatic.com"));
    }

    #[test]
    fn test_nonce_generate() {
        let nonce = CspNonce::generate();
        assert!(!nonce.as_str().is_empty());
        assert_eq!(nonce.as_str().len(), 24); // 16 bytes base64 = 24 chars
    }

    #[test]
    fn test_nonce_uniqueness() {
        let a = CspNonce::generate();
        let b = CspNonce::generate();
        assert_ne!(a.as_str(), b.as_str());
    }

    #[test]
    fn test_nonce_display() {
        let nonce = CspNonce::generate();
        assert_eq!(format!("{}", nonce), nonce.as_str());
    }

    #[test]
    fn test_nonce_as_ref_str() {
        let nonce = CspNonce::generate();
        let s: &str = nonce.as_ref();
        assert_eq!(s, nonce.as_str());
    }

    #[test]
    fn test_azumi_nonce_defaults() {
        let nonce = CspNonce::generate();
        let csp = ContentSecurityPolicy::azumi_nonce_defaults(&nonce).build();
        assert!(csp.contains("default-src 'self'"));
        assert!(csp.contains(&format!("script-src 'self' 'nonce-{}'", nonce.as_str())));
        assert!(csp.contains(&format!("style-src 'self' 'nonce-{}'", nonce.as_str())));
        assert!(!csp.contains("'unsafe-inline'"));
        assert!(csp.contains("form-action 'self'"));
        assert!(csp.contains("frame-ancestors 'none'"));
    }

    #[test]
    fn test_try_generate_ok() {
        let nonce = CspNonce::try_generate().expect("try_generate should succeed on standard systems");
        assert_eq!(nonce.as_str().len(), 24);
    }
}
