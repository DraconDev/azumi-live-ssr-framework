// Tests for HTML attribute rendering and validation
// These verify that various HTML5 attributes, data-* attributes, aria-* attributes,
// and multiple attributes render correctly in the html! macro.

use azumi::{html, render_to_string};

#[test]
fn test_html_attributes_basic() {
    let simple = html! {
        <div>"Simple HTML test"</div>
    };

    let rendered = render_to_string(&simple);
    assert!(rendered.contains("Simple HTML test"));
}

#[test]
fn test_html_attributes_multiple() {
    let class_name = "test";
    let output = html! {
        <div class={class_name}>"content"</div>
    };

    let rendered = render_to_string(&output);
    assert!(rendered.contains(r#"class="test""#));
    assert!(rendered.contains("content"));
}

#[test]
fn test_multiple_attributes() {
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
    let output = html! {
        <div data-user-id="123" data-role="admin">"User"</div>
    };

    let rendered = render_to_string(&output);
    assert!(rendered.contains(r#"data-user-id="123""#));
    assert!(rendered.contains(r#"data-role="admin""#));
}

#[test]
fn test_aria_attributes() {
    let output = html! {
        <button aria-label="Close dialog" aria-pressed="false">"X"</button>
    };

    let rendered = render_to_string(&output);
    assert!(rendered.contains(r#"aria-label="Close dialog""#));
    assert!(rendered.contains(r#"aria-pressed="false""#));
}
