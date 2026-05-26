use std::fs;
use std::path::Path;

// minify_js is defined via include — single source of truth in build_support/minify_js.rs
include!("build_support/minify_js.rs");

/// Compute a stable hash using FNV-1a algorithm.
/// Unlike DefaultHasher (SipHash), this is deterministic across Rust versions.
fn fnv_hash(data: &str) -> u64 {
    const INITIAL: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = INITIAL;
    for byte in data.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

/// Verify idiomorph.js integrity using FNV-1a hash.
/// Catches accidental modifications or corruption.
/// When updating idiomorph.js, update IDEMORPH_HASH below.
fn verify_idiomorph_integrity(content: &str) {
    const IDEMORPH_HASH: u64 = 0xf909dd266d21f20d; // FNV-1a of known-good idiomorph.js
    let computed = fnv_hash(content);
    if computed != IDEMORPH_HASH {
        panic!(
            "Azumi: idiomorph.js integrity check FAILED!\n\
             Expected FNV-1a hash: {:#018x}\n\
             Got:                  {:#018x}\n\
             If you intentionally updated idiomorph.js, update IDEMORPH_HASH in build.rs.\n\
             If not, the file may be corrupted — re-download from upstream.",
            IDEMORPH_HASH, computed
        );
    }
}

/// Verify azumi.js integrity using FNV-1a hash.
/// Catches accidental modifications or corruption.
/// When updating azumi.js, update AZUMI_JS_HASH below.
fn verify_azumi_js_integrity(content: &str) {
    const AZUMI_JS_HASH: u64 = 0xce8f7ceb819ea60c; // FNV-1a of known-good azumi.js
    let computed = fnv_hash(content);
    if computed != AZUMI_JS_HASH {
        panic!(
            "Azumi: azumi.js integrity check FAILED!\n\
             Expected FNV-1a hash: {:#018x}\n\
             Got:                  {:#018x}\n\
             If you intentionally updated azumi.js, update AZUMI_JS_HASH in build.rs.\n\
             If not, the file may be corrupted — restore from git.",
            AZUMI_JS_HASH, computed
        );
    }
}

fn main() {
    // Only run if client files change
    println!("cargo:rerun-if-changed=client/idiomorph.js");
    println!("cargo:rerun-if-changed=client/azumi.js");
    println!("cargo:rerun-if-changed=AI_RULES_HASH");

    let client_dir = Path::new("client");
    let src_dir = Path::new("src");

    // Read files - graceful handling if files are missing
    let idiomorph = match fs::read_to_string(client_dir.join("idiomorph.js")) {
        Ok(content) => {
            // Verify idiomorph.js integrity
            verify_idiomorph_integrity(&content);
            content
        }
        Err(e) => {
            eprintln!(
                "warning: Failed to read client/idiomorph.js: {}. Using empty content.",
                e
            );
            String::new()
        }
    };
    let azumi = match fs::read_to_string(client_dir.join("azumi.js")) {
        Ok(content) => {
            // Verify azumi.js integrity
            verify_azumi_js_integrity(&content);
            content
        }
        Err(e) => {
            eprintln!(
                "warning: Failed to read client/azumi.js: {}. Using empty content.",
                e
            );
            String::new()
        }
    };

    // Concatenate and minify
    let combined = format!("{}\n\n{}", idiomorph, azumi);
    let minified = minify_js(&combined);

    // Write to src/client.min.js so it can be included with include_str!
    if let Err(e) = fs::write(src_dir.join("client.min.js"), minified) {
        eprintln!("warning: Failed to write src/client.min.js: {}", e);
    }

    // ── AI Framework Fingerprint ──────────────────────────────────────────
    // Compute a deterministic hash from the version + strict rules.
    // AI assistants read AZUMI_AI_HASH to verify they're targeting the
    // correct framework version and rule set.
    //
    // If the env var AZUMI_AI_HASH is already set (e.g. in CI or .env),
    // use that value directly so teams can pin to a known-good hash.
    //
    // Otherwise, compute from the canonical rules list.
    let ai_hash = std::env::var("AZUMI_AI_HASH").unwrap_or_else(|_| compute_ai_hash());

    println!("cargo:rustc-env=AZUMI_AI_HASH={}", ai_hash);
}

/// Compute a hash from the framework version and strict AI rules.
/// This hash changes whenever the rules change, giving AI assistants
/// a way to verify they're generating code for the correct rule set.
fn compute_ai_hash() -> String {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".into());

    let rules: &[&str] = &[
        // Version is read from Cargo.toml at build time
        "version=dynamic",
        // Syntax rules
        "text_must_be_quoted=true",
        "css_values_must_be_quoted=true",
        "class_must_be_snake_case=true",
        "static_class_attr_banned=true",
        "static_style_attr_banned=true",
        "static_id_attr_banned=true",
        "dashes_in_css_banned=true",
        // Macro rules
        "style_block_after_html=true",
        "let_class_anti_pattern=true",
        "on_event_syntax=call",
        "component_builder_pattern=true",
        // Security rules
        "hmac_signed_state=true",
        "xss_escaping_seo=true",
        "secret_env_var=AZUMI_SECRET",
    ];

    let mut combined = version.to_string();
    for rule in rules {
        combined.push_str(rule);
    }
    format!("{:x}", fnv_hash(&combined))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify_js_strips_line_comments() {
        let src = "let x = 1; // this is a comment\nlet y = 2;";
        let result = minify_js(src);
        assert!(!result.contains("comment"), "line comment should be stripped");
        assert!(result.contains("let x = 1;"), "code should remain");
        assert!(result.contains("let y = 2;"), "code should remain");
    }

    #[test]
    fn test_minify_js_strips_block_comments() {
        let src = "let x = 1; /* block comment */ let y = 2;";
        let result = minify_js(src);
        assert!(!result.contains("block comment"), "block comment should be stripped");
        assert!(result.contains("let x = 1;"), "code should remain");
        assert!(result.contains("let y = 2;"), "code should remain");
    }

    #[test]
    fn test_minify_js_preserves_string_literals() {
        let src = r#"let s = "// not a comment"; let t = '/* not block */';"#;
        let result = minify_js(src);
        assert!(result.contains("// not a comment"), "string content should be preserved");
        assert!(result.contains("/* not block */"), "string content should be preserved");
    }

    #[test]
    fn test_minify_js_preserves_template_literals() {
        let src = "let s = `hello ${world}`;";
        let result = minify_js(src);
        assert!(result.contains("hello ${world}"), "template literal should be preserved");
    }

    #[test]
    fn test_minify_js_collapses_whitespace() {
        let src = "let   x   =   1;\n\n\nlet   y   =   2;";
        let result = minify_js(src);
        assert!(!result.contains("   "), "multiple spaces should collapse");
        assert!(!result.contains("\n\n"), "multiple blank lines should collapse");
    }

    #[test]
    fn test_minify_js_empty_input_falls_back() {
        let src = "   \n   \n   ";
        let result = minify_js(src);
        assert_eq!(result, src, "whitespace-only input should fall back to original");
    }

    #[test]
    fn test_minify_js_preserves_regex_slash() {
        let src = r#"let re = /foo/;"#;
        let result = minify_js(src);
        assert!(result.contains("/foo/"), "regex literal should be preserved");
    }

    #[test]
    fn test_minify_js_handles_escaped_quotes_in_strings() {
        let src = r#"let s = "he said \"hello\"";"#;
        let result = minify_js(src);
        assert!(result.contains(r#"he said \"hello\""#), "escaped quotes should be preserved");
    }

    #[test]
    fn test_minify_js_regex_with_escaped_slash() {
        let src = r#"let re = /https?:\/\//;"#;
        let result = minify_js(src);
        assert!(result.contains("/https?"), "regex with escaped slash should be preserved");
    }

    #[test]
    fn test_minify_js_regex_after_semicolon() {
        let src = "return /pattern/;";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after return should be preserved");
    }

    #[test]
    fn test_minify_js_division_not_regex() {
        let src = "let x = a / b;";
        let result = minify_js(src);
        assert!(result.contains("a / b"), "division should not be treated as regex");
    }

    #[test]
    fn test_minify_js_regex_char_class_with_slash() {
        let src = "let re = /[/]/;";
        let result = minify_js(src);
        assert!(result.contains("[/"), "regex char class with slash should be preserved");
    }

    #[test]
    fn test_minify_js_block_comment_at_eof() {
        let src = "let x = 1; /* unclosed";
        let result = minify_js(src);
        assert!(result.contains("let x = 1;"), "code before unclosed comment should survive");
        assert!(!result.contains("unclosed"), "unclosed comment content should be stripped");
    }

    #[test]
    fn test_minify_js_preserves_multibyte_utf8_in_strings() {
        let src = r#"let s = "café";"#;
        let result = minify_js(src);
        assert!(result.contains("café"), "multi-byte UTF-8 in strings should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_preserves_cjk_in_code() {
        let src = r#"let 名前 = "日本語"; // comment"#;
        let result = minify_js(src);
        assert!(result.contains("名前"), "CJK identifier should be preserved: {}", result);
        assert!(result.contains("日本語"), "CJK string should be preserved: {}", result);
        assert!(!result.contains("comment"), "comment should be stripped");
    }

    #[test]
    fn test_minify_js_preserves_emoji_in_strings() {
        let src = r#"let s = "hello 🌍";"#;
        let result = minify_js(src);
        assert!(result.contains("🌍"), "emoji in string should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_regex_after_return_keyword() {
        let src = "return /pattern/;";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after return keyword should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_regex_after_typeof_keyword() {
        let src = "typeof /pattern/";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after typeof keyword should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_regex_after_new_keyword() {
        let src = "new /pattern/";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after new keyword should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_regex_after_throw_keyword() {
        let src = "throw /pattern/";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after throw keyword should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_regex_after_delete_keyword() {
        let src = "delete /pattern/";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after delete keyword should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_regex_after_void_keyword() {
        let src = "void /pattern/";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after void keyword should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_identifier_prevents_regex() {
        let src = "foo / bar;";
        let result = minify_js(src);
        assert!(result.contains("foo / bar"), "identifier before / should be division, not regex: {}", result);
    }

    #[test]
    fn test_minify_js_closing_paren_prevents_regex() {
        let src = "foo() / 2;";
        let result = minify_js(src);
        assert!(result.contains("() / 2"), ") before / should be division: {}", result);
    }

    #[test]
    fn test_minify_js_return_regex_with_double_slash() {
        let src = "return /https://example/;";
        let result = minify_js(src);
        assert!(result.contains("/https://example/"), "regex with :// after return should not be mangled as comment: {}", result);
    }

    #[test]
    fn test_minify_js_number_then_division() {
        let src = "let x = 10 / 2; // real comment";
        let result = minify_js(src);
        assert!(result.contains("10 / 2"), "number / number should be division: {}", result);
        assert!(!result.contains("real comment"), "actual comment should be stripped");
    }

    #[test]
    fn test_minify_js_regex_after_in_keyword() {
        let src = "for (var k in /pattern/.test(x)) {}";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after 'in' keyword should be preserved: {}", result);
    }

    #[test]
    fn test_minify_js_regex_after_case_keyword() {
        let src = "case /pattern/:";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex after 'case' keyword should be preserved: {}", result);
    }
}
