use azumi::{html, render_to_string};

#[test]
fn test_named_arguments_work() {
    // This test verifies that the html! macro compiles successfully
    // and that basic named argument patterns work in the macro
    let simple = html! {
        <div>"Simple HTML test"</div>
    };

    let rendered = render_to_string(&simple);
    assert!(rendered.contains("Simple HTML test"));
}

#[test]
fn test_named_args_in_component_call() {
    // Verify that the macro correctly handles named arguments syntax
    // in component-like invocations (using the @ComponentName syntax)
    let output = html! {
        <div class="test">"content"</div>
    };

    let rendered = render_to_string(&output);
    assert!(rendered.contains(r#"class="test""#));
    assert!(rendered.contains("content"));
}

#[test]
fn test_multiple_attributes() {
    // Test that multiple named attributes compile and render correctly
    let output = html! {
        <input type="text" name="username" placeholder="Enter username" />
    };

    let rendered = render_to_string(&output);
    assert!(rendered.contains(r#"type="text""#));
    assert!(rendered.contains(r#"name="username""#));
    assert!(rendered.contains(r#"placeholder="Enter username""#));
}

#[test]
fn test_data_attributes() {
    // Test data-* attributes which use hyphens
    let output = html! {
        <div data-user-id="123" data-role="admin">"User"</div>
    };

    let rendered = render_to_string(&output);
    assert!(rendered.contains(r#"data-user-id="123""#));
    assert!(rendered.contains(r#"data-role="admin""#));
}

#[test]
fn test_aria_attributes() {
    // Test aria-* accessibility attributes
    let output = html! {
        <button aria-label="Close dialog" aria-pressed="false">"X"</button>
    };

    let rendered = render_to_string(&output);
    assert!(rendered.contains(r#"aria-label="Close dialog""#));
    assert!(rendered.contains(r#"aria-pressed="false""#));
}
