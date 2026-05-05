//! Security / XSS Attack Vector Tests
//!
//! These verify that the html! macro properly handles XSS attack patterns
//! for <script>{var}</script> and <style>{var}</style> auto-escaping.
//!
//! The key security property: content inside <script>/<style> tags is
//! automatically escaped to prevent breakout from the container tags.
//!
//! Note: The browser HTML parser closes <script> blocks when it sees </script>,
//! even if that sequence appears inside JavaScript string literals. The escaping
//! prevents the script content from BREAKING OUT.
//!
//! Run with: cargo test --test security_xss_tests --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// json_data! Escaping (macro — does real serde work)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_json_data_escapes_script_tags_in_json() {
    let data = serde_json::json!({"x": "<script>alert(1)</script>"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("<script>alert(1)</script>"), "Script tags in JSON should be escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_json_data_no_double_escape() {
    let data = serde_json::json!({"x": r"a<\/script>b"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains(r"<\\\/script>"), "Should not triple-escape");
}

// ════════════════════════════════════════════════════════════════════════════
// <script>{var}</script> Auto-Escaping Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_script_auto_escapes_basic_closing_tag() {
    let js = "console.log('hello');</script>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(!output.contains("console.log('hello');</script>"), "Script content should have closing tag escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_script_auto_escapes_titlecase_closing_tag() {
    let js = "data</Script>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(!output.contains("</Script>"), "Titlecase script tag should be escaped");
    assert!(output.contains(r"<\/Script>"));
}

#[test]
fn test_script_auto_escapes_uppercase_closing_tag() {
    let js = "data</SCRIPT>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(!output.contains("</SCRIPT>"), "Uppercase script tag should be escaped");
    assert!(output.contains(r"<\/SCRIPT>"));
}

#[test]
fn test_script_auto_escapes_closing_tag_with_space() {
    let js = "data</ script>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(!output.contains("</ script>"), "Script tag with space should be escaped");
    assert!(output.contains(r"<\/ script>"));
}

#[test]
fn test_script_auto_escapes_html_comment_bypass() {
    let js = "--></script><script>alert(1)</script>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(!output.contains("</script><script>"), "HTML comment bypass should be escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_script_comment_bypass_with_uppercase() {
    let js = "--></SCRIPT><SCRIPT>alert(1)</SCRIPT>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(!output.contains("</SCRIPT><SCRIPT>"));
    assert!(output.contains(r"<\/SCRIPT>"));
}

#[test]
fn test_script_no_double_escape() {
    let js = r"console.log('<\/script>');";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(!output.contains(r"\\\/script"), "Already-escaped content should not be double-escaped");
}

#[test]
fn test_script_null_byte_preserved() {
    let js = "console.log('test');\0</script>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(output.contains('\u{0}'), "Null byte should be preserved in output");
}

#[test]
fn test_script_control_chars_preserved() {
    let js = "data\x01\x02</script>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(output.contains('\u{01}'));
    assert!(output.contains('\u{02}'));
}

#[test]
fn test_script_opening_tag_not_escaped() {
    let js = "<script>alert(1)</script>";
    let component = html! { <script>{js}</script> };
    let output = test::render(&component);
    assert!(output.contains("<script>alert"), "Opening tag in content should NOT be escaped — only closing </script> matters");
}

#[test]
fn test_script_multiple_breakouts() {
    let js1 = "console.log('a');</script>";
    let js2 = "console.log('b');</script>";

    let component = html! {
        <div>
            <script>{js1}</script>
            <span>"separator"</span>
            <script>{js2}</script>
        </div>
    };
    let output = test::render(&component);

    assert!(output.contains("separator"));
    assert!(!output.contains("console.log('a');</script>console.log('b');</script>"));
}

// ════════════════════════════════════════════════════════════════════════════
// <style>{var}</style> Auto-Escaping Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_style_auto_escapes_closing_tag() {
    let css = ".x {}</style><script>alert(1)</script>";
    let component = html! { <style>{css}</style> };
    let output = test::render(&component);
    assert!(!output.contains("</style><script>"), "Style breakout should be escaped");
    assert!(output.contains(r"<\/style>"));
}

#[test]
fn test_style_auto_escapes_titlecase_closing_tag() {
    let css = ".x {}</Style>";
    let component = html! { <style>{css}</style> };
    let output = test::render(&component);
    assert!(!output.contains("</Style>"), "Titlecase style tag should be escaped");
    assert!(output.contains(r"<\/Style>"));
}

#[test]
fn test_style_auto_escapes_uppercase_closing_tag() {
    let css = ".x {}</STYLE>";
    let component = html! { <style>{css}</style> };
    let output = test::render(&component);
    assert!(!output.contains("</STYLE>"), "Uppercase style tag should be escaped");
    assert!(output.contains(r"<\/STYLE>"));
}

#[test]
fn test_style_auto_escapes_closing_tag_with_space() {
    let css = ".x {}</ style>";
    let component = html! { <style>{css}</style> };
    let output = test::render(&component);
    assert!(!output.contains("</ style>"), "Style tag with space should be escaped");
    assert!(output.contains(r"<\/ style>"));
}

#[test]
fn test_style_no_double_escape() {
    let css = r".btn { color: red; }<\/style>";
    let component = html! { <style>{css}</style> };
    let output = test::render(&component);
    assert!(!output.contains(r"\\\/style"), "Already-escaped content should not be double-escaped");
}

#[test]
fn test_style_null_byte_preserved() {
    let css = ".test { color: red; }\0</style>";
    let component = html! { <style>{css}</style> };
    let output = test::render(&component);
    assert!(output.contains('\u{0}'), "Null byte should be preserved in output");
}

#[test]
fn test_style_opening_tag_not_escaped() {
    let css = "<style>.my_class {}</style>";
    let component = html! { <style>{css}</style> };
    let output = test::render(&component);
    assert!(output.contains("<style>"), "Opening tag should NOT be escaped");
}

// ════════════════════════════════════════════════════════════════════════════
// Combined Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_injection_patterns_together() {
    let css = ".test { color: red; }";
    let js = "console.log('hi');";
    let data = serde_json::json!({"key": "value"});

    let component = html! {
        <div>
            <style>{css}</style>
            <script>{js}</script>
            {azumi::json_data!("DATA" = &data)}
        </div>
    };
    let output = test::render(&component);

    assert!(output.contains(".test { color: red; }"));
    assert!(output.contains("console.log('hi')"));
    assert!(output.contains("DATA"));
    assert!(output.contains("value"));
}
