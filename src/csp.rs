//! Content-Security-Policy helpers for Azumi applications.
//!
//! Provides a builder for constructing CSP headers that work well with
//! Azumi's zero-hydration architecture and ~3KB runtime.
//!
//! # Example
//!
//! ```rust,ignore
//! use azumi::security::ContentSecurityPolicy;
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

    fn add_directive(mut self, name: &str, value: &str) -> Self {
        self.directives.push((name.to_string(), value.to_string()));
        self
    }

    pub fn default_src(self, value: &str) -> Self {
        self.add_directive("default-src", value)
    }

    pub fn script_src(self, value: &str) -> Self {
        self.add_directive("script-src", value)
    }

    pub fn style_src(self, value: &str) -> Self {
        self.add_directive("style-src", value)
    }

    pub fn img_src(self, value: &str) -> Self {
        self.add_directive("img-src", value)
    }

    pub fn font_src(self, value: &str) -> Self {
        self.add_directive("font-src", value)
    }

    pub fn connect_src(self, value: &str) -> Self {
        self.add_directive("connect-src", value)
    }

    pub fn media_src(self, value: &str) -> Self {
        self.add_directive("media-src", value)
    }

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
}
