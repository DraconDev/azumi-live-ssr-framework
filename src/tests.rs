#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::{
        azumi_script, compute_scope_id, scope_css, AZUMI_AI_HASH, AZUMI_RULES,
        AZUMI_VERSION,
    };

    #[test]
    fn test_scope_css_basic() {
        let css = ".button { color: red; }";
        let scoped = scope_css(css, "abc");
        assert!(scoped.contains(".button[data-abc]"));
        assert!(scoped.contains("color: red;"));
    }

    #[test]
    fn test_scope_css_multiple_selectors() {
        let css = ".button, .link { color: blue; }";
        let scoped = scope_css(css, "s123");
        assert!(scoped.contains(".button[data-s123], .link[data-s123]"));
    }

    #[test]
    fn test_scope_css_media_query_preserved() {
        let css = "@media (max-width: 768px) { .foo { color: red; } }";
        let scoped = scope_css(css, "xyz");
        assert!(scoped.contains("@media (max-width: 768px)"));
        assert!(scoped.contains(".foo[data-xyz]"));
    }

    #[test]
    fn test_scope_css_at_font_face_not_scoped() {
        let css = "@font-face { src: url(font.woff2); } .foo { color: blue; }";
        let scoped = scope_css(css, "q1");
        assert!(scoped.contains("@font-face"));
        assert!(scoped.contains(".foo[data-q1]"));
    }

    #[test]
    fn test_compute_scope_id_deterministic() {
        let id1 = compute_scope_id(10, 5);
        let id2 = compute_scope_id(10, 5);
        assert_eq!(id1, id2, "Same input should produce same scope ID");
        assert!(id1.starts_with('s'), "Scope ID should start with 's'");
    }

    #[test]
    fn test_compute_scope_id_different_inputs() {
        let id1 = compute_scope_id(10, 5);
        let id2 = compute_scope_id(10, 6);
        let id3 = compute_scope_id(11, 5);
        assert_ne!(id1, id2, "Different column should produce different ID");
        assert_ne!(id1, id3, "Different line should produce different ID");
    }

    #[test]
    fn test_scope_id_format_valid() {
        let id = compute_scope_id(100, 25);
        assert!(
            id.starts_with('s') && id.len() >= 2 && id.len() <= 18,
            "Scope ID should start with 's' and be reasonable length, got: {}",
            id
        );
        let hex_part = &id[1..];
        assert!(
            hex_part.chars().all(|c| c.is_ascii_hexdigit()),
            "Scope ID hex part should be hex chars only, got: {}",
            id
        );
    }

    #[test]
    fn test_scope_css_keyframes_content_not_scoped() {
        let css = "@keyframes fade { 0% { opacity: 0; } 100% { opacity: 1; } }";
        let scoped = scope_css(css, "kf1");
        assert!(scoped.contains("@keyframes fade"));
        assert!(scoped.contains("0%"), "keyframes percentages should be preserved");
    }

    #[test]
    fn test_ai_hash_is_populated() {
        assert!(!AZUMI_AI_HASH.is_empty(), "AZUMI_AI_HASH must not be empty");
        assert!(
            AZUMI_AI_HASH.len() >= 8,
            "AZUMI_AI_HASH should be at least 8 chars, got {}",
            AZUMI_AI_HASH.len()
        );
    }

    #[test]
    fn test_version_matches_cargo() {
        assert_eq!(AZUMI_VERSION, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_ai_rules_are_nonempty() {
        assert!(!AZUMI_RULES.is_empty(), "AZUMI_RULES must not be empty");
        assert!(
            AZUMI_RULES.len() >= 10,
            "Should have at least 10 strict rules"
        );
    }

    #[test]
    fn test_ai_rules_contain_key_rules() {
        let all_rules = AZUMI_RULES.join("\n");
        assert!(all_rules.contains("quoted"), "Rules must mention quoting");
        assert!(
            all_rules.contains("snake_case"),
            "Rules must mention snake_case"
        );
        assert!(all_rules.contains("HMAC"), "Rules must mention HMAC");
        assert!(all_rules.contains("@let"), "Rules must mention @let");
    }

    #[test]
    fn test_azumi_script_returns_component() {
        let script = azumi_script();
        let _ = script;
    }

    #[test]
    fn test_azumi_script_renders_correctly() {
        let script = azumi_script();
        let output = crate::render_to_string(&script);
        assert!(
            output.starts_with("<script>"),
            "Should start with <script>, got: {}",
            output
        );
        assert!(
            output.ends_with("</script>"),
            "Should end with </script>, got: {}",
            output
        );
    }

    #[test]
    fn test_azumi_script_escapes_script_end_tag() {
        let script = azumi_script();
        let output = crate::render_to_string(&script);
        let js_content = &output[8..output.len() - 9];
        if js_content.contains("</script>") {
            assert!(
                output.contains(r"<\/script>"),
                "If JS contains </script>, it should be escaped as <\\/script>"
            );
        }
        if js_content.contains("</SCRIPT>") {
            assert!(
                output.contains(r"<\/SCRIPT>"),
                "If JS contains </SCRIPT>, it should be escaped as <\\/SCRIPT>"
            );
        }
    }

    #[test]
    fn test_azumi_script_contains_azumi_code() {
        let script = azumi_script();
        let output = crate::render_to_string(&script);
        assert!(
            output.contains("azumi"),
            "Should contain 'azumi' identifier"
        );
    }

    #[test]
    fn test_session_cleanup_script_renders() {
        let script = crate::session_cleanup_script();
        let output = crate::render_to_string(&script);
        assert!(
            output.starts_with("<script>"),
            "Should start with <script>, got: {}",
            output
        );
        assert!(
            output.ends_with("</script>"),
            "Should end with </script>, got: {}",
            output
        );
        assert!(
            output.contains("window.location.hash"),
            "Should contain session cleanup logic"
        );
    }

    #[test]
    fn test_trusted_html_renders_without_escaping() {
        let html = crate::script::TrustedHtml::new("<div>Hello &amp; World</div>");
        let output = crate::render_to_string(&html);
        assert_eq!(
            output, "<div>Hello &amp; World</div>",
            "TrustedHtml should render without escaping"
        );
    }

    #[test]
    fn test_trusted_html_preserves_script_tags() {
        let html = crate::script::TrustedHtml::new("<script>alert('test')</script>");
        let output = crate::render_to_string(&html);
        assert!(
            output.contains("<script>"),
            "TrustedHtml should preserve script tags, got: {}",
            output
        );
    }

    // ============================================================================
    // Property-Based Tests for scope_css
    // ============================================================================

    #[cfg(test)]
    mod scope_css_proptest {
        use crate::scope_css;

        /// Property: Every class selector gets the scope attribute appended
        #[test]
        fn prop_scope_css_appends_attribute() {
            let css = ".btn { color: red; } .card { padding: 1rem; }";
            let scoped = scope_css(css, "sabc");
            assert!(scoped.contains(".btn[data-sabc]"), "Class selector should get scope attribute. Got: {}", scoped);
            assert!(scoped.contains(".card[data-sabc]"), "Class selector should get scope attribute. Got: {}", scoped);
        }

        /// Property: Keyframes content is NOT scoped (copied verbatim)
        #[test]
        fn prop_scope_css_keyframes_untouched() {
            let css = "@keyframes slide { from { transform: translateX(0); } to { transform: translateX(100%); } }";
            let scoped = scope_css(css, "sxyz");
            // The keyframes selector and its inner content should not be modified
            assert!(scoped.contains("@keyframes slide"), "Keyframes rule should be preserved");
            assert!(scoped.contains("from { transform: translateX(0); }"), "Keyframes content should not be scoped");
        }

        /// Property: Media queries are recursively scoped
        #[test]
        fn prop_scope_css_media_query_scoped() {
            let css = "@media (min-width: 768px) { .sidebar { width: 250px; } }";
            let scoped = scope_css(css, "s123");
            assert!(scoped.contains("@media (min-width: 768px)"), "Media query should be preserved");
            assert!(scoped.contains(".sidebar[data-s123]"), "Inner selectors should be scoped");
        }

        /// Property: Pseudo-elements (::after, ::before) get scoped correctly
        #[test]
        fn prop_scope_css_pseudo_element() {
            let css = ".btn::after { content: '>'; }";
            let scoped = scope_css(css, "spseudo");
            assert!(scoped.contains(".btn[data-spseudo]::after"), "Pseudo-element should come after scope attribute. Got: {}", scoped);
        }

        /// Property: Pseudo-classes (:hover, :focus) get scoped correctly
        #[test]
        fn prop_scope_css_pseudo_class() {
            let css = ".btn:hover { color: blue; }";
            let scoped = scope_css(css, "shover");
            assert!(scoped.contains(".btn[data-shover]:hover"), "Pseudo-class should come after scope attribute. Got: {}", scoped);
        }

        /// Property: Multiple comma-separated selectors all get scoped
        #[test]
        fn prop_scope_css_multiple_selectors() {
            let css = ".btn, .link, .card { margin: 0; }";
            let scoped = scope_css(css, "smulti");
            assert!(scoped.contains(".btn[data-smulti], .link[data-smulti], .card[data-smulti]"), "All selectors should be scoped. Got: {}", scoped);
        }

        /// Property: Attribute selectors are preserved and scoped
        #[test]
        fn prop_scope_css_attribute_selector() {
            let css = "input[type='text'] { border: 1px solid; }";
            let scoped = scope_css(css, "sattr");
            assert!(scoped.contains("input[type='text'][data-sattr]"), "Attribute selector should be scoped. Got: {}", scoped);
        }

        /// Property: Descendant combinators are scoped on the last simple selector
        #[test]
        fn prop_scope_css_descendant_combinator() {
            let css = ".parent .child { color: red; }";
            let scoped = scope_css(css, "sdesc");
            assert!(scoped.contains(".parent .child[data-sdesc]"), "Descendant selector should scope the last part. Got: {}", scoped);
        }

        /// Property: ID selectors are scoped
        #[test]
        fn prop_scope_css_id_selector() {
            let css = "#header { font-size: 2rem; }";
            let scoped = scope_css(css, "sid");
            assert!(scoped.contains("#header[data-sid]"), "ID selector should be scoped. Got: {}", scoped);
        }

        /// Property: Empty CSS produces empty output
        #[test]
        fn prop_scope_css_empty() {
            let scoped = scope_css("", "sempty");
            assert_eq!(scoped, "", "Empty CSS should produce empty output");
        }

        /// Property: @font-face is not scoped (has no selectors to scope)
        #[test]
        fn prop_scope_css_font_face() {
            let css = "@font-face { src: url('font.woff2'); }";
            let scoped = scope_css(css, "sfont");
            assert!(scoped.contains("@font-face"), "@font-face should be preserved");
        }

        /// Property: Nested media queries are recursively scoped
        #[test]
        fn prop_scope_css_nested_media() {
            let css = "@media screen { @media (min-width: 100px) { .deep { color: red; } } }";
            let scoped = scope_css(css, "snest");
            assert!(scoped.contains(".deep[data-snest]"), "Nested media query content should be scoped");
        }
    }
}
