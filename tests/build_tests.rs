use http_body_util::BodyExt;
use azumi::action::error_fragment;

fn minify_js(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut prev_was_regex_possible = true;

    while i < len {
        let ch = bytes[i] as char;

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

        if ch == '/' && i + 1 < len && bytes[i + 1] == b'/' {
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }

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
        prev_was_regex_possible = matches!(ch, '=' | '(' | '[' | '{' | ',' | ';' | '!' | '&' | '|' | '^' | '~' | '<' | '>' | '+' | '-' | '*' | '/' | '%' | '?' | ':' | '@');
        i += 1;
    }

    let result: String = out
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    if result.is_empty() { src.to_string() } else { result }
}

async fn response_to_string(response: axum::response::Response) -> String {
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

mod minify_js_tests {
    use super::*;

    #[test]
    fn strips_line_comments() {
        let src = "let x = 1; // this is a comment\nlet y = 2;";
        let result = minify_js(src);
        assert!(!result.contains("comment"));
        assert!(result.contains("let x = 1;"));
        assert!(result.contains("let y = 2;"));
    }

    #[test]
    fn strips_block_comments() {
        let src = "let x = 1; /* block comment */ let y = 2;";
        let result = minify_js(src);
        assert!(!result.contains("block comment"));
        assert!(result.contains("let x = 1;"));
        assert!(result.contains("let y = 2;"));
    }

    #[test]
    fn preserves_string_literals() {
        let src = r#"let s = "// not a comment"; let t = '/* not block */';"#;
        let result = minify_js(src);
        assert!(result.contains("// not a comment"));
        assert!(result.contains("/* not block */"));
    }

    #[test]
    fn preserves_template_literals() {
        let src = "let s = `hello ${world}`;";
        let result = minify_js(src);
        assert!(result.contains("hello ${world}"));
    }

    #[test]
    fn collapses_whitespace() {
        let src = "let   x   =   1;\n\n\nlet   y   =   2;";
        let result = minify_js(src);
        assert!(!result.contains("   "));
        assert!(!result.contains("\n\n"));
    }

    #[test]
    fn empty_input_falls_back() {
        let src = "   \n   \n   ";
        let result = minify_js(src);
        assert_eq!(result, src);
    }

    #[test]
    fn handles_escaped_quotes_in_strings() {
        let src = r#"let s = "he said \"hello\"";"#;
        let result = minify_js(src);
        assert!(result.contains(r#"he said \"hello\""#));
    }

    #[test]
    fn no_false_positive_division_as_comment() {
        let src = "let x = 10 / 2; // real comment";
        let result = minify_js(src);
        assert!(result.contains("10 / 2"), "division operator should survive");
        assert!(!result.contains("real comment"), "actual comment should be stripped");
    }

    #[test]
    fn preserves_regex_literal() {
        let src = "let re = /foo/;";
        let result = minify_js(src);
        assert!(result.contains("/foo/"), "regex literal should be preserved");
    }

    #[test]
    fn preserves_regex_with_flags() {
        let src = "let re = /pattern/gi;";
        let result = minify_js(src);
        assert!(result.contains("/pattern/gi"), "regex with flags should be preserved");
    }

    #[test]
    fn preserves_regex_with_escaped_slash() {
        let src = r"let re = /https?:\/\//;";
        let result = minify_js(src);
        assert!(result.contains("/https?:\\/"), "regex with escaped slash should be preserved");
    }

    #[test]
    fn preserves_regex_in_test_call() {
        let src = "if (/pattern/.test(s)) {}";
        let result = minify_js(src);
        assert!(result.contains("/pattern/"), "regex in test() call should be preserved");
    }

    #[test]
    fn regex_after_semicolon() {
        let src = "return /abc/;";
        let result = minify_js(src);
        assert!(result.contains("/abc/"), "regex after return keyword should be preserved");
    }

    #[test]
    fn regex_after_equals() {
        let src = "let x = /foo/.test(s);";
        let result = minify_js(src);
        assert!(result.contains("/foo/"), "regex after = should be preserved");
    }

    #[test]
    fn regex_with_char_class_containing_slash() {
        let src = "let re = /[/]/;";
        let result = minify_js(src);
        assert!(result.contains("[/"), "regex char class with / should be preserved");
    }

    #[test]
    fn division_not_treated_as_regex() {
        let src = "let x = a / b;";
        let result = minify_js(src);
        assert!(result.contains("a / b"), "division should not become regex");
    }

    #[test]
    fn block_comment_at_eof_no_panic() {
        let src = "let x = 1; /* unclosed";
        let result = minify_js(src);
        assert!(result.contains("let x = 1;"), "code before unclosed comment should survive");
    }
}

mod error_fragment_tests {
    use super::*;

    #[tokio::test]
    async fn uses_az_on_instead_of_onclick() {
        let response = error_fragment("Error", Some("my_form"));
        let body = response_to_string(response).await;
        assert!(
            body.contains("az-on="),
            "should use az-on: {}",
            body
        );
        assert!(
            !body.contains("onclick="),
            "should NOT use onclick: {}",
            body
        );
    }

    #[tokio::test]
    async fn retry_button_calls_builtin_action() {
        let response = error_fragment("Error", Some("my_form"));
        let body = response_to_string(response).await;
        assert!(
            body.contains("__azumi_retry"),
            "should reference __azumi_retry: {}",
            body
        );
    }

    #[tokio::test]
    async fn stores_form_id_in_data_attribute() {
        let response = error_fragment("Error", Some("my_form"));
        let body = response_to_string(response).await;
        assert!(
            body.contains("data-retry-form=\"my_form\""),
            "should have data-retry-form: {}",
            body
        );
    }

    #[tokio::test]
    async fn no_form_id_means_no_retry_button() {
        let response = error_fragment("Error", None);
        let body = response_to_string(response).await;
        assert!(
            !body.contains("__azumi_retry"),
            "no retry button without form_id: {}",
            body
        );
        assert!(
            !body.contains("data-retry-form"),
            "no data-retry-form without form_id: {}",
            body
        );
    }
}
