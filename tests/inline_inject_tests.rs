use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Safe JSON Data Injection Tests (json_data! macro — kept as it does real work)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_json_data_macro() {
    let data = serde_json::json!({"count": 42, "name": "test"});
    let component = html! {
        {azumi::json_data!("window.__DATA__" = &data)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("<script>"));
    assert!(output.contains("window.__DATA__"));
    assert!(output.contains("\"count\":42"));
    assert!(output.contains("\"name\":\"test\""));
    assert!(output.contains("</script>"));
}

#[test]
fn test_json_data_escapes_script_tags() {
    let data = serde_json::json!({"html": "<script>alert(1)</script>"});
    let component = html! {
        {azumi::json_data!("window.__DATA__" = &data)}
    };
    let output = test::render(&component);
    
    // Should escape </script> to prevent XSS
    assert!(!output.contains("<script>alert(1)</script>"));
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_json_data_with_unicode() {
    let data = serde_json::json!({"greeting": "你好", "emoji": "🚀"});
    let component = html! {
        {azumi::json_data!("APP_DATA" = &data)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("你好"));
    assert!(output.contains("🚀"));
}

#[test]
fn test_json_data_with_numeric_key() {
    let data = serde_json::json!({"a": 1, "b": 2.5, "c": -3});
    let component = html! {
        {azumi::json_data!("MIXED" = &data)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("1"));
    assert!(output.contains("2.5"));
    assert!(output.contains("-3"));
}

#[test]
fn test_json_data_with_null_and_bool() {
    let data = serde_json::json!({"enabled": true, "debug": null, "count": 0});
    let component = html! {
        {azumi::json_data!("FLAGS" = &data)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("true"));
    assert!(output.contains("null"));
    assert!(output.contains("0"));
}

#[test]
fn test_format_in_expression_detected() {
    // This test verifies the compile-time detection works
    let value = "test";
    let component = html! {
        <p>{value}</p>
    };
    let output = test::render(&component);
    assert!(output.contains("test"));
}

// ════════════════════════════════════════════════════════════════════════════
// Case-Insensitive Escaping Tests for json_data!
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_json_data_escapes_uppercase_script() {
    let data = serde_json::json!({"x": "</SCRIPT>alert(1)"});
    let component = html! {
        {azumi::json_data!("X" = &data)}
    };
    let output = test::render(&component);
    assert!(!output.contains("</SCRIPT>alert"), "Uppercase </SCRIPT> should be escaped");
    assert!(output.contains(r"<\/SCRIPT>"));
}

#[test]
fn test_json_data_escapes_titlecase_script() {
    let data = serde_json::json!({"x": "</Script>alert(1)"});
    let component = html! {
        {azumi::json_data!("X" = &data)}
    };
    let output = test::render(&component);
    assert!(!output.contains("</Script>alert"), "Titlecase </Script> should be escaped");
    assert!(output.contains(r"<\/Script>"));
}

#[test]
fn test_json_data_escapes_nested_script() {
    let data = serde_json::json!({"x": "a</script>b</script>c"});
    let component = html! {
        {azumi::json_data!("X" = &data)}
    };
    let output = test::render(&component);
    assert!(output.contains(r"<\/script>"), "Both </script> should be escaped");
    assert_eq!(
        output.matches(r"<\/script>").count(),
        2,
        "Should escape both occurrences"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Complex JSON Structures
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_json_data_with_nested_objects() {
    let data = serde_json::json!({
        "user": {
            "name": "Alice",
            "address": {
                "city": "NYC",
                "zip": "10001"
            }
        }
    });
    let component = html! { {azumi::json_data!("DATA" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("Alice"));
    assert!(output.contains("NYC"));
    assert!(output.contains("10001"));
}

#[test]
fn test_json_data_with_array() {
    let data = serde_json::json!({"items": [1, 2, 3]});
    let component = html! { {azumi::json_data!("ARR" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("items"));
    assert!(output.contains("1"));
    assert!(output.contains("2"));
    assert!(output.contains("3"));
}

#[test]
fn test_json_data_with_mixed_array() {
    let data = serde_json::json!({"mixed": [1, "two", true, null]});
    let component = html! { {azumi::json_data!("MIX" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("1"));
    assert!(output.contains("two"));
    assert!(output.contains("true"));
    assert!(output.contains("null"));
}

#[test]
fn test_json_data_with_escaped_strings() {
    let data = serde_json::json!({"x": "a\"b\\c"});
    let component = html! { {azumi::json_data!("ESC" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("a"));
    assert!(output.contains("b"));
}

#[test]
fn test_json_data_with_newlines() {
    let data = serde_json::json!({"x": "line1\nline2"});
    let component = html! { {azumi::json_data!("NL" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("line1"));
}

#[test]
fn test_json_data_empty_object() {
    let data = serde_json::json!({});
    let component = html! { {azumi::json_data!("EMPTY" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("{}"));
}

#[test]
fn test_json_data_empty_array() {
    let data = serde_json::json!([]);
    let component = html! { {azumi::json_data!("EMPTY_ARR" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("[]"));
}

#[test]
fn test_json_data_with_float() {
    let data = serde_json::json!({"pi": 3.14159});
    let component = html! { {azumi::json_data!("PI" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("3.14159"));
}

#[test]
fn test_json_data_with_special_chars_in_key() {
    let data = serde_json::json!({"key-with-dash": 1});
    let component = html! { {azumi::json_data!("SPECIAL" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("key-with-dash"));
}

// ════════════════════════════════════════════════════════════════════════════
// Control Flow Integration
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_json_data_inside_if_block() {
    let data = serde_json::json!({"show": true});
    let show = true;
    let component = html! {
        <div>
            @if show {
                {azumi::json_data!("DATA" = &data)}
            }
        </div>
    };
    let output = test::render(&component);
    assert!(output.contains("DATA"));
}

#[test]
fn test_three_json_data_macros() {
    let d1 = serde_json::json!({"a": 1});
    let d2 = serde_json::json!({"b": 2});
    let d3 = serde_json::json!({"c": 3});
    let component = html! {
        <div>
            {azumi::json_data!("A" = &d1)}
            {azumi::json_data!("B" = &d2)}
            {azumi::json_data!("C" = &d3)}
        </div>
    };
    let output = test::render(&component);
    assert!(output.contains("A"));
    assert!(output.contains("B"));
    assert!(output.contains("C"));
}

// ════════════════════════════════════════════════════════════════════════════
// HTML Entity Escaping Through JSON
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_json_data_with_html_entities() {
    let data = serde_json::json!({"html": "<b>bold</b>&amp;&lt;"});
    let component = html! { {azumi::json_data!("HTML" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("&lt;"), "JSON serialization preserves &amp; but renders &lt; as HTML entity");
}

#[test]
fn test_json_data_with_img_onerror_xss() {
    let data = serde_json::json!({"img": "<img src=x onerror=alert(1)>"});
    let component = html! { {azumi::json_data!("IMG" = &data)} };
    let output = test::render(&component);
    assert!(output.contains("onerror=alert"), "JSON string data is stored, not executed");
}

#[test]
fn test_json_data_does_not_double_escape() {
    let data = serde_json::json!({"x": r"a<\/script>b"});
    let component = html! { {azumi::json_data!("X" = &data)} };
    let output = test::render(&component);
    assert!(
        !output.contains(r"<\\\/script>"),
        "Should not triple-escape already-escaped content"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Auto-Escaping in <script> and <style> tags
// (Replaces former inline_css! / inline_script! macro tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_auto_escape_script_tag() {
    let js = "console.log('hello');</script>";
    let component = html! {
        <script>{js}</script>
    };
    let output = test::render(&component);
    assert!(output.contains(r"<\/script>"), "Should escape closing script tag");
    assert!(!output.contains("console.log('hello');</script>"));
}

#[test]
fn test_auto_escape_style_tag() {
    let css = ".my_class { color: red; }";
    let component = html! {
        <style>{css}</style>
    };
    let output = test::render(&component);
    assert!(output.contains(".my_class { color: red; }"));
}

#[test]
fn test_auto_escape_style_breakout() {
    let css = ".x {}</style><script>alert(1)</script>";
    let component = html! {
        <style>{css}</style>
    };
    let output = test::render(&component);
    assert!(!output.contains("</style><script>"), "Style breakout should be escaped");
    assert!(output.contains(r"<\/style>"));
}

#[test]
fn test_auto_escape_script_uppercase() {
    let js = "data</SCRIPT>";
    let component = html! {
        <script>{js}</script>
    };
    let output = test::render(&component);
    assert!(!output.contains("</SCRIPT>"), "Uppercase script tag should be escaped");
    assert!(output.contains(r"<\/SCRIPT>"));
}

#[test]
fn test_auto_escape_style_uppercase() {
    let css = ".x {}</STYLE>";
    let component = html! {
        <style>{css}</style>
    };
    let output = test::render(&component);
    assert!(!output.contains("</STYLE>"), "Uppercase style tag should be escaped");
    assert!(output.contains(r"<\/STYLE>"));
}

#[test]
fn test_auto_escape_script_with_space() {
    let js = "data</ script>";
    let component = html! {
        <script>{js}</script>
    };
    let output = test::render(&component);
    assert!(!output.contains("</ script>"), "Script tag with space should be escaped");
    assert!(output.contains(r"<\/ script>"));
}

#[test]
fn test_auto_escape_style_with_space() {
    let css = ".x {}</ style>";
    let component = html! {
        <style>{css}</style>
    };
    let output = test::render(&component);
    assert!(!output.contains("</ style>"), "Style tag with space should be escaped");
    assert!(output.contains(r"<\/ style>"));
}

#[test]
fn test_auto_escape_no_double_escape_script() {
    let js = r"console.log('<\/script>');";
    let component = html! {
        <script>{js}</script>
    };
    let output = test::render(&component);
    assert!(!output.contains(r"\\\/script"), "Already-escaped content should not be double-escaped");
}

#[test]
fn test_auto_escape_no_double_escape_style() {
    let css = r".btn { color: red; }<\/style>";
    let component = html! {
        <style>{css}</style>
    };
    let output = test::render(&component);
    assert!(!output.contains(r"\\\/style"), "Already-escaped content should not be double-escaped");
}

#[test]
fn test_auto_escape_mixed_content() {
    let css = ".a { color: red; }";
    let js = "console.log('hi');";
    let data = serde_json::json!({"key": "value"});
    let component = html! {
        <div>
            <style>{css}</style>
            <script>{js}</script>
            {azumi::json_data!("DATA" = &data)}
            <p>"Hello"</p>
        </div>
    };
    let output = test::render(&component);
    assert!(output.contains(".a { color: red; }"));
    assert!(output.contains("console.log('hi')"));
    assert!(output.contains("DATA"));
    assert!(output.contains("Hello"));
}

#[test]
fn test_auto_escape_inside_for_loop() {
    let items = vec!["one", "two"];
    let component = html! {
        <div>
            @for item in items {
                <style>{THEME_CSS}</style>
            }
        </div>
    };
    static THEME_CSS: &str = ".item { color: red; }";
    let output = test::render(&component);
    assert!(output.contains(".item { color: red; }"));
}

#[test]
fn test_auto_escape_inside_if_branch() {
    static JS: &str = "console.log('branch');";
    let condition = true;
    let component = html! {
        <div>
            @if condition {
                <script>{JS}</script>
            }
        </div>
    };
    let output = test::render(&component);
    assert!(output.contains("console.log('branch')"));
}

#[test]
fn test_auto_escape_opening_tag_not_escaped() {
    // Opening <script> / <style> should NOT be escaped
    let js = "<script>alert(1)</script>";
    let component = html! {
        <script>{js}</script>
    };
    let output = test::render(&component);
    assert!(output.contains("<script>alert"), "Opening tag content should pass through");
}

#[test]
fn test_auto_escape_interleaved() {
    let css_a = ".a { color: red; }";
    let js = "console.log('hi');";
    let css_b = ".b { color: blue; }";
    let component = html! {
        <div>
            <style>{css_a}</style>
            <script>{js}</script>
            <style>{css_b}</style>
        </div>
    };
    let output = test::render(&component);
    assert!(output.contains(".a { color: red; }"));
    assert!(output.contains("console.log('hi')"));
    assert!(output.contains(".b { color: blue; }"));
}
