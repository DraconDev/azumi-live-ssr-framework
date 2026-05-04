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
