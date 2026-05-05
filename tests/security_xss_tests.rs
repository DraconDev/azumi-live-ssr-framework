//! Security / XSS Attack Vector Tests
//!
//! These verify that known XSS attack patterns in injected content are properly
//! escaped through the safe macros. The macros specifically prevent breakout
//! from </script>/</style> tags. String data inside JSON/CSS/JS is just data.
//!
//! Run with: cargo test --test security_xss_tests

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Script Breakout Prevention (THE core security guarantee)
// json_data! and inline_script! must escape </script> to prevent breakout
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_json_data_escapes_closing_script_tag() {
    let data = serde_json::json!({"x": "<script>alert(1)</script>"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("<script>alert(1)</script>"), "Script tag should be escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_inline_script_escapes_closing_script_tag() {
    let js = "console.log('hi');</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    eprintln!("OUTPUT: {:?}", output);
    assert!(!output.contains("</script>"), "Closing script tag should be escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_inline_script_escapes_titlecase_script_tag() {
    let js = "data</Script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</Script>"), "Titlecase script tag should be escaped");
    assert!(output.contains(r"<\/Script>"));
}

#[test]
fn test_inline_script_escapes_uppercase_script_tag() {
    let js = "data</SCRIPT>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</SCRIPT>"), "Uppercase script tag should be escaped");
    assert!(output.contains(r"<\/SCRIPT>"));
}

#[test]
fn test_inline_script_escapes_script_with_space() {
    let js = "data</ script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</ script>"), "Script tag with space should be escaped");
    assert!(output.contains(r"<\/ script>"));
}

#[test]
fn test_inline_script_escapes_multiple_script_tags() {
    let js = "a</script>b</script>c";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert_eq!(output.matches(r"<\/script>").count(), 2, "Both script tags should be escaped");
}

#[test]
fn test_json_data_escapes_multiple_closing_tags() {
    let data = serde_json::json!({"x": "a</script>b</script>c"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("</script>"));
    assert!(output.matches(r"<\/script>").count() >= 2);
}

// ════════════════════════════════════════════════════════════════════════════
// CSS Breakout Prevention
// inline_css! must escape </style> to prevent breakout from <style> block
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_inline_css_escapes_closing_style_tag() {
    let css = ".x {}</style><script>alert(1)</script>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(!output.contains("</style><script>"), "Style breakout should be escaped");
    assert!(output.contains(r"<\/style>"));
}

#[test]
fn test_inline_css_escapes_titlecase_style_tag() {
    let css = ".x {}</Style>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(!output.contains("</Style>"), "Titlecase style tag should be escaped");
    assert!(output.contains(r"<\/Style>"));
}

#[test]
fn test_inline_css_escapes_uppercase_style_tag() {
    let css = ".x {}</STYLE>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(!output.contains("</STYLE>"), "Uppercase style tag should be escaped");
    assert!(output.contains(r"<\/STYLE>"));
}

#[test]
fn test_inline_css_escapes_style_with_space() {
    let css = ".x {}</ style>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(!output.contains("</ style>"), "Style tag with space should be escaped");
    assert!(output.contains(r"<\/ style>"));
}

#[test]
fn test_inline_css_escapes_multiple_style_tags() {
    let css = "a</style>b</style>c";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert_eq!(output.matches(r"<\/style>").count(), 2, "Both style tags should be escaped");
}

// ════════════════════════════════════════════════════════════════════════════
// HTML Comment Bypass Prevention
// --></script> trick to escape out of script block
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_inline_script_escapes_html_comment_bypass() {
    let js = "--></script><script>alert(1)</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</script><script>"), "HTML comment bypass should be escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_inline_script_comment_bypass_with_uppercase() {
    let js = "--></SCRIPT><SCRIPT>alert(1)</SCRIPT>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</SCRIPT><SCRIPT>"));
    assert!(output.contains(r"<\/SCRIPT>"));
}

// ════════════════════════════════════════════════════════════════════════════
// Double-Escape Prevention
// Already-escaped content should NOT be double-escaped
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_inline_script_no_double_escape() {
    let js = r"console.log('<\/script>');";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains(r"\\\/script"), "Already-escaped content should not be double-escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_inline_css_no_double_escape() {
    let css = r".btn { color: red; }<\/style>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(!output.contains(r"\\\/style"), "Already-escaped content should not be double-escaped");
    assert!(output.contains(r"<\/style>"));
}

#[test]
fn test_json_data_no_double_escape() {
    let data = serde_json::json!({"x": r"a<\/script>b"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains(r"<\\\/script>"), "Should not triple-escape");
}

// ════════════════════════════════════════════════════════════════════════════
// Null Byte Handling
// Null bytes in content should NOT enable bypass (they break string matching)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_null_byte_preserved_in_script() {
    let js = "console.log('test');\0</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(output.contains('\u{0}'), "Null byte should be preserved in output");
    assert!(!output.contains("</script>"), "The null byte prevents the script tag from being found");
}

#[test]
fn test_null_byte_preserved_in_css() {
    let css = ".test { color: red; }\0</style>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(output.contains('\u{0}'), "Null byte should be preserved in output");
    assert!(!output.contains("</style>"), "The null byte prevents the style tag from being found");
}

#[test]
fn test_control_chars_preserved() {
    let js = "data\x01\x02</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(output.contains('\u{01}'));
    assert!(output.contains('\u{02}'));
}

// ════════════════════════════════════════════════════════════════════════════
// Opening Tags NOT Escaped (correct behavior — only closing tags are dangerous)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_opening_script_tag_not_escaped() {
    let js = "<script>alert(1)</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(output.contains("<script>alert"), "Opening tag should NOT be escaped");
}

#[test]
fn test_opening_style_tag_not_escaped() {
    let css = "<style>.my_class {}</style>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(output.contains("<style>"), "Opening tag should NOT be escaped");
}

// ════════════════════════════════════════════════════════════════════════════
// Macro Ordering / Interaction
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_three_macros_together() {
    let css = ".test { color: red; }";
    let js = "console.log('hi');";
    let data = serde_json::json!({"key": "value"});

    let component = html! {
        <div>
            {azumi::inline_css!(css)}
            {azumi::inline_script!(js)}
            {azumi::json_data!("DATA" = &data)}
        </div>
    };
    let output = test::render(&component);

    assert!(output.contains(".test { color: red; }"));
    assert!(output.contains("console.log('hi')"));
    assert!(output.contains("DATA"));
    assert!(output.contains("value"));
}

#[test]
fn test_interleaved_script_with_breakout_payload() {
    let js1 = "console.log('a');</script>";
    let js2 = "console.log('b');</script>";

    let component = html! {
        <div>
            {azumi::inline_script!(js1)}
            <span>"separator"</span>
            {azumi::inline_script!(js2)}
        </div>
    };
    let output = test::render(&component);

    assert!(output.matches(r"<\/script>").count() >= 2);
    assert!(output.contains("separator"));
}

#[test]
fn test_css_and_script_with_same_breakout_tag() {
    let css = ".a {}</style>";
    let js = "var x;</style>";

    let component = html! {
        <div>
            {azumi::inline_css!(css)}
            {azumi::inline_script!(js)}
        </div>
    };
    let output = test::render(&component);

    assert!(!output.contains("</style>"), "Both should have escaped </style>");
    assert!(output.matches(r"<\/style>").count() >= 2);
}