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

#[test]
fn test_class_external() {
    // class:external allows third-party CSS class names without validation
    let output = html! {
        <div class:external="payment-widget cms-card">"Button"</div>
    };

    let rendered = render_to_string(&output);
    // Renders as class="..." not class:external="..."
    assert!(rendered.contains(r#"class="payment-widget cms-card""#));
    assert!(!rendered.contains("class:external"));
}

#[test]
fn test_id_external() {
    // id:external allows third-party IDs without validation
    let output = html! {
        <div id:external="my-element">"Content"</div>
    };

    let rendered = render_to_string(&output);
    // Renders as id="..." not id:external="..."
    assert!(rendered.contains(r#"id="my-element""#));
    assert!(!rendered.contains("id:external"));
}

#[test]
fn test_class_external_with_azumi_class() {
    // Can combine class:external (third-party) with class={} (Azumi-managed)
    // Note: Both render as class="..." in output, so they're merged
    let extra = "flex";
    let output = html! {
        <div class:external="payment-widget" class={extra}>"Mixed"</div>
    };

    let rendered = render_to_string(&output);
    assert!(rendered.contains("payment-widget"));
    assert!(rendered.contains("flex"));
}

#[test]
fn test_class_external_xss_escaped() {
    // class:external values are HTML-escaped
    // Use string literal with malicious content
    let output = html! {
        <div class:external="bg-red&quot; onclick=&quot;alert(1)">"Test"</div>
    };

    let rendered = render_to_string(&output);
    // HTML entities in literal should be double-escaped
    assert!(rendered.contains("&amp;quot;"));
}
