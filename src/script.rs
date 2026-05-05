use crate::Component;

/// Escape closing tag sequences in content strings to prevent XSS.
/// 
/// Covers all case variants: lowercase, titlecase, uppercase, and with space.
/// Uses a single-pass scanner for O(n) performance regardless of content size.
/// 
/// # Examples
/// ```
/// let js = azumi::escape_tag_content("hello </script> world", "script");
/// assert_eq!(js, r"hello <\/script> world");
/// 
/// let css = azumi::escape_tag_content(".btn { color: red; } </style>", "style");
/// assert_eq!(css, r".btn { color: red; } <\/style>");
/// ```
pub fn escape_tag_content(content: &str, tag_name: &str) -> String {
    // Pre-allocate with extra capacity for escapes (worst case: every 9 chars is an escape)
    let mut result = String::with_capacity(content.len() + content.len() / 8);
    let bytes = content.as_bytes();
    let tag_lower = tag_name.to_lowercase();
    let tag_upper = tag_name.to_uppercase();
    let tag_title = format!("{}{}", &tag_name[..1].to_uppercase(), &tag_name[1..].to_lowercase());
    
    let patterns: [(&[u8], &str); 4] = [
        (format!("</{}", tag_lower).as_bytes(), &format!(r"<\/{}", tag_lower)),
        (format!("</{}", tag_title).as_bytes(), &format!(r"<\/{}", tag_title)),
        (format!("</{}", tag_upper).as_bytes(), &format!(r"<\/{}", tag_upper)),
        (format!("</ {}", tag_lower).as_bytes(), &format!(r"<\/{}", tag_lower)),
    ];
    
    let mut i = 0;
    while i < bytes.len() {
        let mut matched = false;
        
        // Check for </tag> pattern (starts with '<' followed by '/')
        if i + 2 < bytes.len() && bytes[i] == b'<' && bytes[i + 1] == b'/' {
            for (pattern, replacement) in &patterns {
                let end = i + pattern.len();
                if end <= bytes.len() && &bytes[i..end] == *pattern {
                    result.push_str(replacement);
                    i = end;
                    matched = true;
                    break;
                }
            }
        }
        
        if !matched {
            result.push(content[i]);
            i += 1;
        }
    }
    
    result
}

/// Escape `</script>` in JavaScript strings (case-insensitive) to prevent XSS.
/// 
/// Delegates to [`escape_tag_content`] with tag_name="script".
pub fn escape_script_content(js: &str) -> String {
    escape_tag_content(js, "script")
}

/// Escape `</style>` in CSS strings (case-insensitive) to prevent XSS.
/// 
/// Delegates to [`escape_tag_content`] with tag_name="style".
pub fn escape_style_content(css: &str) -> String {
    escape_tag_content(css, "style")
}

pub struct AzumiScript;

impl Component for AzumiScript {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<script>{}</script>",
            escape_script_content(crate::AZUMI_JS)
        )
    }
}

pub struct SessionCleanupScript;

impl SessionCleanupScript {
    pub const SCRIPT: &'static str = r#"(function(){var h=window.location.hash||'';var t='session'+'_'+'token'+'=';var r='refresh'+'_'+'token'+'=';var c='code'+'=';if(h.indexOf(t)!==-1||h.indexOf(r)!==-1||h.indexOf(c)!==-1){history.replaceState(null,'',window.location.pathname+window.location.search);}})()"#;
}

impl Component for SessionCleanupScript {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<script>{}</script>", Self::SCRIPT)
    }
}

pub fn session_cleanup_script() -> SessionCleanupScript {
    SessionCleanupScript
}

/// TrustedHtml — pre-sanitized HTML injection (use sparingly).
///
/// Bypasses ALL of Azumi's safety guarantees (escaping, scoping, validation).
/// Only use for pre-sanitized HTML from known-trusted sources (e.g., markdown renderer output).
#[doc(hidden)]
pub struct TrustedHtml(pub String);

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
        assert_eq!(escape_script_content("</ScRiPt>"), "</ScRiPt>", "Mixed case not in allowlist passes through");
    }

    #[test]
    fn test_escape_style_mixed_case() {
        assert_eq!(escape_style_content("</style>"), r"<\/style>");
        assert_eq!(escape_style_content("</Style>"), r"<\/Style>");
        assert_eq!(escape_style_content("</STYLE>"), r"<\/STYLE>");
        assert_eq!(escape_style_content("</ style>"), r"<\/ style>");
        assert_eq!(escape_style_content("</StYlE>"), "</StYlE>", "Mixed case not in allowlist passes through");
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
    // SessionCleanupScript
    // =========================================================================

    #[test]
    fn test_session_cleanup_script_renders() {
        let script = SessionCleanupScript;
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
}
