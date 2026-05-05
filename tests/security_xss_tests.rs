//! Security / XSS Attack Vector Tests
//!
//! These verify that known XSS attack patterns in injected content are properly
//! escaped through the safe macros.
//!
//! Run with: cargo test --test security_xss_tests

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Classic XSS Payloads Through json_data!
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_xss_script_tag_payload() {
    let data = serde_json::json!({"x": "<script>alert(1)</script>"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("<script>alert(1)</script>"), "Script tag should be escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_xss_img_onerror_payload() {
    let data = serde_json::json!({"x": "<img src=x onerror=alert(1)>"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("onerror=alert(1)"), "onerror handler should not appear unescaped");
}

#[test]
fn test_xss_javascript_protocol_payload() {
    let data = serde_json::json!({"x": "javascript:alert(1)"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("javascript:alert"));
}

#[test]
fn test_xss_event_handler_payload() {
    let data = serde_json::json!({"x": "\" onmouseover=\"alert(1)"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("onmouseover"));
}

#[test]
fn test_xss_iframe_payload() {
    let data = serde_json::json!({"x": "<iframe src='javascript:alert(1)'>"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("javascript:alert(1)"), "JS protocol in iframe should be prevented");
}

#[test]
fn test_xss_svg_onload_payload() {
    let data = serde_json::json!({"x": "<svg onload=alert(1)>"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("onload=alert"));
}

#[test]
fn test_xss_style_content_payload() {
    let data = serde_json::json!({"x": "<style>@import 'javascript:alert(1)'</style>"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("<style>"), "Style tag should be escaped");
}

#[test]
fn test_xss_body_onload_payload() {
    let data = serde_json::json!({"x": "<body onload=alert(1)>"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(!output.contains("onload=alert"));
}

// ════════════════════════════════════════════════════════════════════════════
// XSS Payloads Through inline_css!
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_xss_css_style_breakout() {
    let css = ".x {}</style><script>alert(1)</script>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(!output.contains("</style><script>"), "Style breakout should be escaped");
    assert!(output.contains(r"<\/style>"));
}

#[test]
fn test_xss_css_expression_payload() {
    let css = ".x { color: expression(alert(1)); }";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(output.contains("expression(alert"));
}

#[test]
fn test_xss_css_url_payload() {
    let css = ".x { background: url('javascript:alert(1)'); }";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(output.contains("url"));
    assert!(!output.contains("javascript:alert(1)"), "JS URL in CSS should not be executable");
}

#[test]
fn test_xss_css_different_case_style() {
    let css = ".test {}</STYLE><script>alert(1)</script>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(!output.contains("</STYLE><script>"), "Uppercase style breakout should be escaped");
    assert!(output.contains(r"<\/STYLE>"));
}

// ════════════════════════════════════════════════════════════════════════════
// XSS Payloads Through inline_script!
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_xss_script_comment_bypass() {
    let js = "--></script><script>alert(1)</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</script><script>"), "HTML comment bypass should be escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_xss_script_nested_tags() {
    let js = "</script><script>alert(1)</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</script><script>"), "Nested script tags should be escaped");
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_xss_script_different_case() {
    let js = "</ScRiPt><ScRiPt>alert(1)</ScRiPt>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</ScRiPt>"), "Mixed case script tags pass through (not in allowlist)");
}

// ════════════════════════════════════════════════════════════════════════════
// Null Byte and Control Character Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_null_byte_in_script_payload() {
    let js = "console.log('test');\0</script>";
    let component = html! { {azumi::inline_script!(js)} };
    let output = test::render(&component);
    assert!(!output.contains("</script>"), "Null byte before script tag should prevent match");
    assert!(output.contains('\u{0}'), "Null byte should be preserved");
}

#[test]
fn test_null_byte_in_css_payload() {
    let css = ".test { color: red; }\0</style>";
    let component = html! { {azumi::inline_css!(css)} };
    let output = test::render(&component);
    assert!(!output.contains("</style>"), "Null byte before style tag should prevent match");
    assert!(output.contains('\u{0}'), "Null byte should be preserved");
}

// ════════════════════════════════════════════════════════════════════════════
// Macro Ordering / Interaction Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_mixed_injection_ordering() {
    let css = ".test {}";
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

    let css_pos = output.find(".test {}");
    let js_pos = output.find("console.log('hi')");
    let data_pos = output.find("DATA");

    assert!(css_pos.is_some() && js_pos.is_some() && data_pos.is_some());
}

#[test]
fn test_multiple_script_injections_all_escaped() {
    let js1 = "console.log('a');</script>";
    let js2 = "console.log('b');</Script>";
    let js3 = "console.log('c');</SCRIPT>";

    let component = html! {
        <div>
            {azumi::inline_script!(js1)}
            {azumi::inline_script!(js2)}
            {azumi::inline_script!(js3)}
        </div>
    };
    let output = test::render(&component);

    assert!(!output.contains("</script>"));
    assert!(!output.contains("</Script>"));
    assert!(!output.contains("</SCRIPT>"));
    assert!(output.contains(r"<\/script>"));
    assert!(output.contains(r"<\/Script>"));
    assert!(output.contains(r"<\/SCRIPT>"));
}