// Integration tests for SessionCleanupScript Component

use azumi::{html, test};

#[test]
fn test_session_cleanup_script_renders_correctly() {
    let comp = html! {
        <head>{azumi::session_cleanup_script()}</head>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("<script>"),
        "Should contain opening script tag, got: {}",
        output
    );
    assert!(
        output.contains("</script>"),
        "Should contain closing script tag, got: {}",
        output
    );
    assert!(
        output.contains("window.location.hash"),
        "Should contain session cleanup logic"
    );
    assert!(
        output.contains("history.replaceState"),
        "Should contain history cleanup logic"
    );
}

#[test]
fn test_session_cleanup_script_in_layout() {
    let comp = html! {
        <html>
            <head>{azumi::session_cleanup_script()}</head>
            <body><div>"Content"</div></body>
        </html>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("<head><script>"),
        "Script should be inside head, got: {}",
        output
    );
}

#[test]
fn test_azumi_script_and_session_cleanup_together() {
    let comp = html! {
        <head>
            {azumi::azumi_script()}
            {azumi::session_cleanup_script()}
        </head>
    };
    let output = test::render(&comp);
    assert!(
        output.contains("azumi"),
        "Should contain azumi runtime"
    );
    assert!(
        output.contains("window.location.hash"),
        "Should contain session cleanup"
    );
}