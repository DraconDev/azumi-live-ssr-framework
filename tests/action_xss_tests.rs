//! XSS Prevention Tests for Action Fragments
//!
//! Verifies that error_fragment and success_fragment properly escape
//! HTML in messages and form IDs.

use azumi::action::{error_fragment, success_fragment};
use http_body_util::BodyExt;

// ════════════════════════════════════════════════════════════════════════════
// success_fragment XSS Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_success_fragment_escapes_html_in_message() {
    let xss = "<script>alert('xss')</script>";
    let response = success_fragment(xss);
    let body = response_to_string(response);
    assert!(
        !body.contains("<script>"),
        "Script tag should be escaped in success_fragment: {}",
        body
    );
    assert!(
        body.contains("&lt;script&gt;"),
        "Script tag should be HTML-escaped: {}",
        body
    );
}

#[test]
fn test_success_fragment_escapes_quotes() {
    let xss = r#"" onclick="alert('xss')"#;
    let response = success_fragment(xss);
    let body = response_to_string(response);
    assert!(
        !body.contains("onclick"),
        "onclick handler should be escaped: {}",
        body
    );
    assert!(
        body.contains("&quot;"),
        "Quotes should be HTML-escaped: {}",
        body
    );
}

// ════════════════════════════════════════════════════════════════════════════
// error_fragment XSS Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_error_fragment_escapes_html_in_message() {
    let xss = "<img src=x onerror=alert('xss')>";
    let response = error_fragment(xss, None);
    let body = response_to_string(response);
    assert!(
        !body.contains("<img"),
        "IMG tag should be escaped in error_fragment: {}",
        body
    );
    assert!(
        body.contains("&lt;img"),
        "IMG tag should be HTML-escaped: {}",
        body
    );
}

#[test]
fn test_error_fragment_escapes_single_quotes() {
    let xss = "'"; 
    let response = error_fragment(xss, None);
    let body = response_to_string(response);
    assert!(
        body.contains("&#x27;"),
        "Single quote should be escaped: {}",
        body
    );
}

#[test]
fn test_error_fragment_escapes_form_id_in_onclick() {
    let form_id = "x');alert(1);//";
    let response = error_fragment("Error message", Some(form_id));
    let body = response_to_string(response);
    assert!(
        !body.contains("x');alert(1);//"),
        "form_id should be JS-escaped in onclick: {}",
        body
    );
    assert!(
        body.contains("\\x27") || body.contains("&#x27;"),
        "form_id should be escaped in onclick handler: {}",
        body
    );
}

#[test]
fn test_error_fragment_escapes_ampersand() {
    let xss = "Tom &amp; Jerry";
    let response = error_fragment(xss, None);
    let body = response_to_string(response);
    assert!(
        body.contains("&amp;"),
        "Ampersand should be HTML-escaped: {}",
        body
    );
}

#[test]
fn test_error_fragment_with_both_xss_message_and_form_id() {
    let msg = "<b>Bold</b>";
    let form_id = r#"form');alert('xss');//"#;
    let response = error_fragment(msg, Some(form_id));
    let body = response_to_string(response);
    assert!(
        !body.contains("<b>"),
        "Message HTML should be escaped: {}",
        body
    );
    assert!(
        !body.contains("alert('xss')"),
        "form_id JS should be escaped: {}",
        body
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Helpers
// ════════════════════════════════════════════════════════════════════════════

fn response_to_string(response: axum::response::Response) -> String {
    let body = response.into_body();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let bytes = rt.block_on(async {
        use http_body_util::BodyExt;
        body.collect().await.unwrap().to_bytes()
    });
    String::from_utf8(bytes.to_vec()).unwrap()
}
