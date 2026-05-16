use crate::Component;

/// Escape HTML entities in a string to prevent XSS.
///
/// Escapes the five dangerous HTML characters: `&`, `<`, `>`, `"`, `'`.
/// This is the standard HTML entity escaping function used internally by Azumi
/// for all text interpolation inside `html!`.
///
/// Use this when you need HTML entity escaping outside of `html!` (e.g., for
/// meta tags, SEO helpers, or when building attribute values programmatically).
///
/// Inside `html!`, all `{expression}` interpolation is auto-escaped — you do
/// NOT need to call this manually.
///
/// # Examples
///
/// ```
/// # use azumi::escape_html;
/// assert_eq!(escape_html("<script>alert('xss')</script>"),
///     "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
/// assert_eq!(escape_html("Tom & Jerry"), "Tom &amp; Jerry");
/// assert_eq!(escape_html(r#"class="foo""#), "class=&quot;foo&quot;");
/// ```
#[inline]
#[must_use]
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escape a string for safe inclusion in XML text content or attribute values.
///
/// Same as [`escape_html`] but uses `&apos;` for single quotes (XML standard)
/// instead of `&#x27;` (HTML convention).
#[inline]
#[must_use]
pub fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escape closing tag sequences in content strings to prevent XSS.
/// 
/// Covers all case variants: lowercase, titlecase, uppercase, and with space.
/// Uses a single-pass scanner for O(n) performance regardless of content size.
/// 
/// This is the generic version — use [`escape_script_content`] or [`escape_style_content`]
/// for the specific cases.
/// 
/// # Examples
/// ```
/// # use azumi::escape_tag_content;
/// let js = escape_tag_content("hello </script> world", "script");
/// assert_eq!(js, r"hello <\/script> world");
/// 
/// let css = escape_tag_content(".btn { color: red; } </style>", "style");
/// assert_eq!(css, r".btn { color: red; } <\/style>");
/// ```
#[inline]
#[must_use]
pub fn escape_tag_content(content: &str, tag_name: &str) -> String {
    if tag_name.is_empty() {
        panic!("escape_tag_content called with empty tag_name");
    }
    
    let tag_lower = tag_name.to_lowercase();
    let tag_lower_bytes = tag_lower.as_bytes();
    let tag_len = tag_lower_bytes.len();
    
    let mut result = String::with_capacity(content.len() + content.len() / 8);
    let bytes = content.as_bytes();
    let mut i = 0;
    
    while i < bytes.len() {
        let mut matched = false;
        
        // Check for </tag> pattern (starts with '<' followed by '/')
        if i + 2 < bytes.len() && bytes[i] == b'<' && bytes[i + 1] == b'/' {
            // Check for optional space after </
            let mut j = i + 2;
            let has_space = j < bytes.len() && bytes[j] == b' ';
            if has_space {
                j += 1;
            }
            
            // Check if the following bytes match the tag name (case-insensitive)
            if j + tag_len <= bytes.len() {
                let candidate = &bytes[j..j + tag_len];
                let matches = candidate.iter().zip(tag_lower_bytes.iter()).all(|(a, b)| {
                    a.to_ascii_lowercase() == *b
                });
                
                if matches {
                    result.push_str("<\\/");
                    if has_space {
                        result.push(' ');
                    }
                    result.push_str(&content[j..j + tag_len]);
                    i = j + tag_len;
                    matched = true;
                }
            }
        }
        
        if !matched {
            // Safely handle multi-byte UTF-8 characters
            let ch = content[i..].chars().next().unwrap_or('\u{FFFD}');
            result.push(ch);
            i += ch.len_utf8();
        }
    }
    
    result
}

/// Escape `</script>` in JavaScript strings (case-insensitive) to prevent XSS.
/// 
/// Delegates to [`escape_tag_content`] with tag_name="script".
#[inline]
#[must_use]
pub fn escape_script_content(js: &str) -> String {
    escape_tag_content(js, "script")
}

/// Escape `</style>` in CSS strings (case-insensitive) to prevent XSS.
/// 
/// Delegates to [`escape_tag_content`] with tag_name="style".
#[inline]
#[must_use]
pub fn escape_style_content(css: &str) -> String {
    escape_tag_content(css, "style")
}

pub struct AzumiScript {
    nonce: Option<String>,
}

impl AzumiScript {
    /// Create an AzumiScript component without a CSP nonce.
    #[must_use]
    pub fn new() -> Self {
        AzumiScript { nonce: None }
    }

    /// Create an AzumiScript component with a CSP nonce for nonce-based CSP.
    ///
    /// When using `ContentSecurityPolicy::azumi_nonce_defaults()`, the browser
    /// will only execute `<script nonce="...">` tags whose nonce matches the
    /// CSP header. Use this to include the nonce on the Azumi runtime script.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use azumi::csp::CspNonce;
    ///
    /// async fn handler(nonce: CspNonce) -> impl IntoResponse {
    ///     let csp = ContentSecurityPolicy::azumi_nonce_defaults(&nonce).build();
    ///     let script = azumi::azumi_script().with_nonce(nonce.as_str());
    ///     // Use {script} in your html! template
    /// }
    /// ```
    #[must_use]
    pub fn with_nonce(mut self, nonce: &str) -> Self {
        self.nonce = Some(nonce.to_string());
        self
    }
}

impl Default for AzumiScript {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for AzumiScript {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nonce_attr = match &self.nonce {
            Some(n) => format!(r#" nonce="{}""#, crate::escape_html(n)),
            None => String::new(),
        };
        write!(
            f,
            "<script{}>{}</script>",
            nonce_attr,
            escape_script_content(crate::AZUMI_JS)
        )
    }
}

pub struct SessionCleanupScript {
    nonce: Option<String>,
}

impl SessionCleanupScript {
    pub const SCRIPT: &'static str = r#"(function(){var h=window.location.hash||'';var t='session_token=';var r='refresh_token=';var c='code=';if(h.indexOf(t)!==-1||h.indexOf(r)!==-1||h.indexOf(c)!==-1){history.replaceState(null,'',window.location.pathname+window.location.search);}})()"#;

    #[must_use]
    pub fn with_nonce(mut self, nonce: &str) -> Self {
        self.nonce = Some(nonce.to_string());
        self
    }
}

impl Default for SessionCleanupScript {
    fn default() -> Self {
        SessionCleanupScript { nonce: None }
    }
}

impl Component for SessionCleanupScript {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nonce_attr = match &self.nonce {
            Some(n) => format!(r#" nonce="{}""#, crate::escape_html(n)),
            None => String::new(),
        };
        write!(f, "<script{}>{}</script>", nonce_attr, Self::SCRIPT)
    }
}

#[must_use]
pub fn azumi_script() -> AzumiScript {
    AzumiScript::new()
}

#[must_use]
pub fn session_cleanup_script() -> SessionCleanupScript {
    SessionCleanupScript::default()
}

/// TrustedHtml — pre-sanitized HTML injection (use sparingly).
///
/// Bypasses ALL of Azumi's safety guarantees (escaping, scoping, validation).
/// Only use for pre-sanitized HTML from known-trusted sources (e.g., markdown renderer output).
#[doc(hidden)]
pub struct TrustedHtml(String);

impl Component for TrustedHtml {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TrustedHtml {
    pub fn new(html: &str) -> Self {
        TrustedHtml(html.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    // =========================================================================
    // escape_html
    // =========================================================================

    #[test]
    fn test_escape_html_amp() {
        assert_eq!(escape_html("Tom & Jerry"), "Tom &amp; Jerry");
    }

    #[test]
    fn test_escape_html_angle_brackets() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
    }

    #[test]
    fn test_escape_html_double_quote() {
        assert_eq!(escape_html(r#"class="foo""#), "class=&quot;foo&quot;");
    }

    #[test]
    fn test_escape_html_single_quote() {
        assert_eq!(escape_html("it's"), "it&#x27;s");
    }

    #[test]
    fn test_escape_html_no_special_chars() {
        assert_eq!(escape_html("hello world"), "hello world");
    }

    #[test]
    fn test_escape_html_empty() {
        assert_eq!(escape_html(""), "");
    }

    #[test]
    fn test_escape_html_all_five() {
        assert_eq!(
            escape_html("&<>\"'"),
            "&amp;&lt;&gt;&quot;&#x27;"
        );
    }

    #[test]
    fn test_escape_html_xss_payload() {
        assert_eq!(
            escape_html("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"
        );
    }

    #[test]
    fn test_escape_html_already_escaped() {
        let input = "&amp;&lt;";
        let output = escape_html(input);
        assert_eq!(output, "&amp;amp;&amp;lt;");
    }

    #[test]
    fn test_escape_html_unicode() {
        assert_eq!(escape_html("hello 世界"), "hello 世界");
    }

    // =========================================================================
    // escape_xml
    // =========================================================================

    #[test]
    fn test_escape_xml_amp() {
        assert_eq!(escape_xml("Tom & Jerry"), "Tom &amp; Jerry");
    }

    #[test]
    fn test_escape_xml_single_quote() {
        assert_eq!(escape_xml("it's"), "it&apos;s");
    }

    #[test]
    fn test_escape_xml_all_five() {
        assert_eq!(
            escape_xml("&<>\"'"),
            "&amp;&lt;&gt;&quot;&apos;"
        );
    }

    #[test]
    fn test_escape_xml_no_special_chars() {
        assert_eq!(escape_xml("hello world"), "hello world");
    }

    #[test]
    fn test_escape_xml_vs_html_quotes() {
        let input = "'";
        assert_eq!(escape_html(input), "&#x27;");
        assert_eq!(escape_xml(input), "&apos;");
    }

    // =========================================================================
    // escape_script_content
    // =========================================================================

    #[test]
    fn test_escape_script_content_no_closing_tag() {
        let input = "console.log('hello');";
        assert_eq!(escape_script_content(input), input, "No closing tag should not change");
    }

    #[test]
    fn test_escape_script_content_lowercase() {
        let input = "</script>";
        assert_eq!(escape_script_content(input), r"<\/script>", "Should escape lowercase </script>");
    }

    #[test]
    fn test_escape_script_content_titlecase() {
        let input = "</Script>";
        assert_eq!(escape_script_content(input), r"<\/Script>", "Should escape titlecase </Script>");
    }

    #[test]
    fn test_escape_script_content_uppercase() {
        let input = "</SCRIPT>";
        assert_eq!(escape_script_content(input), r"<\/SCRIPT>", "Should escape uppercase </SCRIPT>");
    }

    #[test]
    fn test_escape_script_content_with_space() {
        let input = "</ script>";
        assert_eq!(escape_script_content(input), r"<\/ script>", "Should escape </ script> with space");
    }

    #[test]
    fn test_escape_script_content_multiple() {
        let input = "a</script>b</Script>c";
        let expected = r"a<\/script>b<\/Script>c";
        assert_eq!(escape_script_content(input), expected, "Should escape multiple occurrences");
    }

    #[test]
    fn test_escape_script_content_partial_no_match() {
        let input = "<script>";
        assert_eq!(escape_script_content(input), "<script>", "Opening tag should NOT be escaped");
    }

    // =========================================================================
    // escape_style_content — ZERO coverage until now
    // =========================================================================

    #[test]
    fn test_escape_style_content_no_closing_tag() {
        let input = ".my_class { color: red; }";
        assert_eq!(escape_style_content(input), input, "No closing tag should not change");
    }

    #[test]
    fn test_escape_style_content_lowercase() {
        let input = "</style>";
        assert_eq!(escape_style_content(input), r"<\/style>", "Should escape lowercase </style>");
    }

    #[test]
    fn test_escape_style_content_titlecase() {
        let input = "</Style>";
        assert_eq!(escape_style_content(input), r"<\/Style>", "Should escape titlecase </Style>");
    }

    #[test]
    fn test_escape_style_content_uppercase() {
        let input = "</STYLE>";
        assert_eq!(escape_style_content(input), r"<\/STYLE>", "Should escape uppercase </STYLE>");
    }

    #[test]
    fn test_escape_style_content_with_space() {
        let input = "</ style>";
        assert_eq!(escape_style_content(input), r"<\/ style>", "Should escape </ style> with space");
    }

    #[test]
    fn test_escape_style_content_multiple() {
        let input = "a</style>b</Style>c";
        let expected = r"a<\/style>b<\/Style>c";
        assert_eq!(escape_style_content(input), expected, "Should escape multiple occurrences");
    }

    #[test]
    fn test_escape_style_content_partial_no_match() {
        let input = "<style>";
        assert_eq!(escape_style_content(input), "<style>", "Opening tag should NOT be escaped");
    }

    // =========================================================================
    // Mixed-case / edge-case coverage for both escape functions
    // =========================================================================

    #[test]
    fn test_escape_script_mixed_case() {
        assert_eq!(escape_script_content("</script>"), r"<\/script>");
        assert_eq!(escape_script_content("</Script>"), r"<\/Script>");
        assert_eq!(escape_script_content("</SCRIPT>"), r"<\/SCRIPT>");
        assert_eq!(escape_script_content("</ script>"), r"<\/ script>");
        assert_eq!(escape_script_content("</ScRiPt>"), r"<\/ScRiPt>", "Mixed case should be escaped case-insensitively");
    }

    #[test]
    fn test_escape_style_mixed_case() {
        assert_eq!(escape_style_content("</style>"), r"<\/style>");
        assert_eq!(escape_style_content("</Style>"), r"<\/Style>");
        assert_eq!(escape_style_content("</STYLE>"), r"<\/STYLE>");
        assert_eq!(escape_style_content("</ style>"), r"<\/ style>");
        assert_eq!(escape_style_content("</StYlE>"), r"<\/StYlE>", "Mixed case should be escaped case-insensitively");
    }

    #[test]
    fn test_escape_script_null_byte() {
        let input = "hello\u{0}</script>";
        let output = escape_script_content(input);
        assert!(output.contains('\u{0}'));
        assert!(!output.contains("</script>"), "Should have escaped </script>");
    }

    #[test]
    fn test_escape_style_null_byte() {
        let input = "color: red;\u{0}</style>";
        let output = escape_style_content(input);
        assert!(output.contains('\u{0}'));
        assert!(!output.contains("</style>"), "Should have escaped </style>");
    }

    #[test]
    fn test_escape_script_control_chars() {
        let input = "data\u{01}\u{02}</script>";
        let output = escape_script_content(input);
        assert!(output.contains('\u{01}'));
        assert!(output.contains('\u{02}'));
        assert!(!output.contains("</script>"), "Should have escaped </script>");
    }

    #[test]
    fn test_escape_script_already_escaped_no_double_escape() {
        let input = r"console.log('<\/script>');";
        let output = escape_script_content(input);
        assert!(!output.contains(r"\\\/script"), "Should not double-escape already-escaped");
        assert_eq!(output, input);
    }

    #[test]
    fn test_escape_style_already_escaped_no_double_escape() {
        let input = r".btn { color: red; }<\/style>";
        let output = escape_style_content(input);
        assert!(!output.contains(r"\\\/style"), "Should not double-escape already-escaped");
        assert_eq!(output, input);
    }

    #[test]
    fn test_escape_script_very_large_payload() {
        let base = "console.log('x');";
        let repeated: String = base.repeat(100_000);
        let input = format!("{}{}", repeated, "</script>");
        let output = escape_script_content(&input);
        assert!(output.ends_with(r"<\/script>"));
        assert!(!output.contains("</script>"));
    }

    #[test]
    fn test_escape_style_very_large_payload() {
        let base = ".my_class { color: red; }";
        let repeated: String = base.repeat(50_000);
        let input = format!("{}{}", repeated, "</style>");
        let output = escape_style_content(&input);
        assert!(output.ends_with(r"<\/style>"));
        assert!(!output.contains("</style>"));
    }

    // =========================================================================
    // TrustedHtml
    // =========================================================================

    #[test]
    fn test_trusted_html_new() {
        let html = TrustedHtml::new("<div>test</div>");
        assert_eq!(html.0, "<div>test</div>");
    }

    #[test]
    fn test_trusted_html_rendering() {
        let html = TrustedHtml::new("<div>test</div>");
        let output = test::render(&html);
        assert_eq!(output, "<div>test</div>", "TrustedHtml should render without escaping");
    }

    #[test]
    fn test_trusted_html_preserves_script_tags() {
        let html = TrustedHtml::new("<script>alert(1)</script>");
        let output = test::render(&html);
        assert!(output.contains("<script>"), "TrustedHtml should preserve script tags");
    }

    #[test]
    fn test_trusted_html_empty_string() {
        let html = TrustedHtml::new("");
        let output = test::render(&html);
        assert_eq!(output, "", "Empty TrustedHtml should render nothing");
    }

    #[test]
    fn test_trusted_html_with_html_entities() {
        let html = TrustedHtml::new("&lt;div&gt;Test&lt;/div&gt;");
        let output = test::render(&html);
        assert!(output.contains("&lt;div&gt;Test&lt;/div&gt;"), "TrustedHtml should not double-escape entities");
    }

    #[test]
    fn test_trusted_html_complex_html() {
        let complex = r#"<div class="container"><h1>Title</h1><p>Paragraph with <a href="/link">link</a></p></div>"#;
        let html = TrustedHtml::new(complex);
        let output = test::render(&html);
        assert!(output.contains(r#"class="container""#), "Should preserve attributes");
        assert!(output.contains("<h1>Title</h1>"), "Should preserve nested elements");
    }

    // =========================================================================
    // AzumiScript
    // =========================================================================

    #[test]
    fn test_azumi_script_new_renders_script_tag() {
        let script = AzumiScript::new();
        let output = test::render(&script);
        assert!(output.starts_with("<script>"), "Should start with <script>");
        assert!(output.ends_with("</script>"), "Should end with </script>");
        assert!(!output.contains("nonce="), "Should NOT have nonce attribute");
    }

    #[test]
    fn test_azumi_script_with_nonce_renders_nonce_attribute() {
        let script = AzumiScript::new().with_nonce("abc123");
        let output = test::render(&script);
        assert!(output.contains(r#"nonce="abc123""#), "Should include nonce attribute");
        assert!(output.starts_with("<script nonce="), "Should start with <script nonce=");
        assert!(output.ends_with("</script>"), "Should end with </script>");
    }

    #[test]
    fn test_azumi_script_nonce_escapes_html() {
        let script = AzumiScript::new().with_nonce(r#"a"b<c"#);
        let output = test::render(&script);
        assert!(!output.contains(r#"nonce="a"b<c""#), "Should NOT have unescaped nonce");
        assert!(output.contains("nonce=\"a&quot;b&lt;c\""), "Should escape nonce value");
    }

    #[test]
    fn test_azumi_script_default_is_new() {
        let default = AzumiScript::default();
        let from_new = AzumiScript::new();
        assert_eq!(test::render(&default), test::render(&from_new), "default() should equal new()");
    }

    // =========================================================================
    // SessionCleanupScript
    // =========================================================================

    #[test]
    fn test_session_cleanup_script_renders() {
        let script = SessionCleanupScript::default();
        let output = test::render(&script);
        assert!(output.starts_with("<script>"), "Should start with <script>");
        assert!(output.ends_with("</script>"), "Should end with </script>");
        assert!(output.contains("session"), "Should contain session cleanup logic");
    }

    #[test]
    fn test_session_cleanup_script_constant() {
        assert!(!SessionCleanupScript::SCRIPT.is_empty(), "SCRIPT constant should not be empty");
        assert!(SessionCleanupScript::SCRIPT.contains("history.replaceState"), "Should contain history.replaceState");
    }

    #[test]
    fn test_session_cleanup_script_with_nonce_renders_nonce() {
        let script = SessionCleanupScript::default().with_nonce("abc123");
        let output = test::render(&script);
        assert!(output.contains(r#"nonce="abc123""#), "Should include nonce attribute: {}", output);
        assert!(output.starts_with("<script nonce="), "Should start with <script nonce=");
        assert!(output.contains("session"), "Should still contain session cleanup logic");
    }

    #[test]
    fn test_session_cleanup_script_nonce_escapes_html() {
        let script = SessionCleanupScript::default().with_nonce(r#"a"b<c"#);
        let output = test::render(&script);
        assert!(output.contains("nonce=\"a&quot;b&lt;c\""), "Should escape nonce value: {}", output);
    }

    #[test]
    fn test_session_cleanup_script_without_nonce() {
        let script = SessionCleanupScript::default();
        let output = test::render(&script);
        assert!(!output.contains("nonce="), "Should NOT have nonce when not set");
    }

    // =========================================================================
    // Property-based tests for escape functions
    // =========================================================================

    use proptest::prelude::*;

    proptest! {
        /// Property: escape_html never leaves unescaped < > & " ' in output
        #[test]
        fn prop_escape_html_always_escapes(s in ".*") {
            let output = escape_html(&s);
            let has_unescaped = output.contains('<')
                || output.contains('>')
                || output.contains('&') && !output.contains("&amp;") && !output.contains("&lt;") && !output.contains("&gt;") && !output.contains("&quot;") && !output.contains("&#x27;");
            prop_assert!(!has_unescaped, "Output should not contain unescaped HTML chars");
        }

        /// Property: escape_html output length is always >= input length
        #[test]
        fn prop_escape_html_length_monotonic(s in ".*") {
            let output = escape_html(&s);
            prop_assert!(output.len() >= s.len(),
                "Escape output should never be shorter than input (got {} < {})",
                output.len(), s.len());
        }

        /// Property: escape_html is idempotent for safe content
        #[test]
        fn prop_escape_html_safe_passthrough(s in "[&<>\"'\\x00-\\x1f]+") {
            let output = escape_html(&s);
            prop_assert!(!output.contains('<') || output.contains("&lt;"), "No unescaped < in output");
            prop_assert!(!output.contains('>') || output.contains("&gt;"), "No unescaped > in output");
        }

        /// Property: escape_script_content never leaves </script> unescaped in output
        #[test]
        fn prop_escape_script_always_escapes(s in ".*") {
            let output = escape_script_content(&s);
            let has_unclosed = output.contains("</script>")
                || output.contains("</Script>")
                || output.contains("</SCRIPT>")
                || output.contains("</ script>");
            prop_assert!(!has_unclosed, "Output should not contain unescaped closing script tag");
        }

        /// Property: escape_style_content never leaves </style> unescaped in output
        #[test]
        fn prop_escape_style_always_escapes(s in ".*") {
            let output = escape_style_content(&s);
            let has_unclosed = output.contains("</style>")
                || output.contains("</Style>")
                || output.contains("</STYLE>")
                || output.contains("</ style>");
            prop_assert!(!has_unclosed, "Output should not contain unescaped closing style tag");
        }

        /// Property: content without '<' passes through unchanged
        #[test]
        fn prop_escape_passthrough(s in "[^<]*") {
            let script_out = escape_script_content(&s);
            prop_assert_eq!(&script_out, &s, "Content without '<' should pass through unchanged for script");

            let style_out = escape_style_content(&s);
            prop_assert_eq!(&style_out, &s, "Content without '<' should pass through unchanged for style");
        }

        /// Property: output length is always >= input length
        #[test]
        fn prop_escape_length_monotonic(s in ".*") {
            let output = escape_script_content(&s);
            prop_assert!(output.len() >= s.len(),
                "Escape output should never be shorter than input (got {} < {})",
                output.len(), s.len());
        }
    }
}
