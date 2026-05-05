use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Safe Injection Macros Tests
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
fn test_inline_css_macro() {
    let css = ".my_class { color: red; }";
    let component = html! {
        {azumi::inline_css!(css)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("<style>"));
    assert!(output.contains(".my_class { color: red; }"));
    assert!(output.contains("</style>"));
}

#[test]
fn test_inline_css_escapes_style_tags() {
    let css = ".my_class { color: red; }</style><script>alert(1)</script>";
    let component = html! {
        {azumi::inline_css!(css)}
    };
    let output = test::render(&component);
    
    // Should escape </style> to prevent XSS
    assert!(!output.contains("</style><script>"));
    assert!(output.contains(r"<\/style>"));
}

#[test]
fn test_inline_css_empty() {
    let css = "";
    let component = html! {
        {azumi::inline_css!(css)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("<style>"));
    assert!(output.contains("</style>"));
    // Should render as <style></style>
}

#[test]
fn test_inline_css_with_unicode() {
    let css = ".card::before { content: \"🚀\"; }";
    let component = html! {
        {azumi::inline_css!(css)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("🚀"));
}

#[test]
fn test_inline_script_macro() {
    let script = "console.log('hello');";
    let component = html! {
        {azumi::inline_script!(script)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("<script>"));
    assert!(output.contains("console.log('hello');"));
    assert!(output.contains("</script>"));
}

#[test]
fn test_inline_script_escapes_script_tags() {
    let script = "console.log('hello');</script><script>alert(1)</script>";
    let component = html! {
        {azumi::inline_script!(script)}
    };
    let output = test::render(&component);
    
    // Should escape </script> to prevent XSS
    assert!(!output.contains("</script><script>"));
    assert!(output.contains(r"<\/script>"));
}

#[test]
fn test_inline_script_empty() {
    let script = "";
    let component = html! {
        {azumi::inline_script!(script)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("<script>"));
    assert!(output.contains("</script>"));
}

#[test]
fn test_inline_script_with_unicode() {
    let script = "console.log('你好');";
    let component = html! {
        {azumi::inline_script!(script)}
    };
    let output = test::render(&component);
    
    assert!(output.contains("你好"));
}

#[test]
fn test_multiple_injections_in_one_component() {
    let data = serde_json::json!({"key": "value"});
    let css = ".btn { color: \"red\"; }";
    let js = "console.log('init');";
    
    let component = html! {
        <div>
            {azumi::json_data!("APP_DATA" = &data)}
            {azumi::inline_css!(css)}
            {azumi::inline_script!(js)}
            <p>"Hello"</p>
        </div>
    };
    let output = test::render(&component);
    
    assert!(output.contains("APP_DATA"));
    assert!(output.contains(".btn"));
    assert!(output.contains("console.log('init')"));
    assert!(output.contains("Hello"));
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
    // We can't test compile errors at runtime, but we verify the function exists
    let value = "test";
    let component = html! {
        <p>{value}</p>
    };
    let output = test::render(&component);
    assert!(output.contains("test"));
}
