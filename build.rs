use std::fs;
use std::path::Path;

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

/// Basic JS minification: strip comments and collapse whitespace.
///
/// Uses a state-machine approach that respects string literals and regex
/// literals, so it won't accidentally strip content inside quotes or regex.
/// Falls back to the original source if minification would produce empty output.
fn minify_js(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut prev_was_regex_possible = true;

    while i < len {
        let ch = bytes[i] as char;

        // String literals — pass through verbatim
        if ch == '\'' || ch == '"' || ch == '`' {
            let quote = ch;
            out.push(ch);
            i += 1;
            while i < len {
                let c = bytes[i] as char;
                out.push(c);
                if c == '\\' && i + 1 < len {
                    i += 1;
                    out.push(bytes[i] as char);
                } else if c == quote {
                    break;
                }
                i += 1;
            }
            i += 1;
            prev_was_regex_possible = false;
            continue;
        }

        // Regex literal: /pattern/flags
        // A '/' is a regex start when preceded by an operator, keyword, or
        // punctuation (not by an identifier, number, or closing bracket/paren).
        if ch == '/' && prev_was_regex_possible && i + 1 < len && bytes[i + 1] != b'/' && bytes[i + 1] != b'*' {
            out.push(ch);
            i += 1;
            while i < len {
                let c = bytes[i] as char;
                out.push(c);
                if c == '\\' && i + 1 < len {
                    i += 1;
                    out.push(bytes[i] as char);
                } else if c == '/' {
                    i += 1;
                    break;
                } else if c == '[' {
                    // Inside character class, '/' is not a regex closer
                    out.push(c);
                    i += 1;
                    while i < len {
                        let cc = bytes[i] as char;
                        out.push(cc);
                        if cc == '\\' && i + 1 < len {
                            i += 1;
                            out.push(bytes[i] as char);
                        } else if cc == ']' {
                            break;
                        }
                        i += 1;
                    }
                    continue;
                }
                i += 1;
            }
            prev_was_regex_possible = false;
            continue;
        }

        // Line comment
        if ch == '/' && i + 1 < len && bytes[i + 1] == b'/' {
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        // Block comment
        if ch == '/' && i + 1 < len && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            } else {
                i = len;
            }
            continue;
        }

        // Collapse runs of whitespace to a single space (or newline if original has one)
        if ch.is_whitespace() {
            let has_newline = src[i..].chars().take_while(|c| c.is_whitespace()).any(|c| c == '\n');
            out.push(if has_newline { '\n' } else { ' ' });
            while i < len && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            prev_was_regex_possible = true;
            continue;
        }

        out.push(ch);
        // After these tokens, a '/' could start a regex; after others, it can't
        prev_was_regex_possible = matches!(ch, '=' | '(' | '[' | '{' | ',' | ';' | '!' | '&' | '|' | '^' | '~' | '<' | '>' | '+' | '-' | '*' | '/' | '%' | '?' | ':' | '@');
        i += 1;
    }

    // Trim leading/trailing whitespace per line, then remove blank lines
    let result: String = out
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    if result.is_empty() { src.to_string() } else { result }
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
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "warning: Failed to read client/idiomorph.js: {}. Using empty content.",
                e
            );
            String::new()
        }
    };
    let azumi = match fs::read_to_string(client_dir.join("azumi.js")) {
        Ok(content) => content,
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
}
