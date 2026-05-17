//! Tests for route constant generation:
//! - #[azumi::page(route = "/path")] generates page_name_ROUTE
//! - #[azumi::action] generates action_name_PATH

use azumi::prelude::*;

// ── Page Route Constants ──────────────────────────────────────────────────

#[azumi::page(route = "/test-about")]
#[allow(non_upper_case_globals)]
fn test_about_page() -> impl Component {
    html! { <div>"About"</div> }
}

#[test]
fn test_page_route_constant_generated() {
    assert_eq!(test_about_page_ROUTE, "/test-about");
}

#[test]
fn test_page_route_constant_usable_in_html() {
    let html = azumi::render_to_string(&html! {
        <a href={test_about_page_ROUTE}>"About"</a>
    });
    assert!(html.contains(r#"href="/test-about""#));
}

// ── Action PATH Constants ─────────────────────────────────────────────────

#[azumi::action]
#[allow(non_upper_case_globals)]
async fn test_like_post(form: TestLikeForm) -> azumi::action::ActionResult {
    let _post_id = form.post_id;
    azumi::action::ActionResult::ok(&html! { <span>"Liked!"</span> })
}

#[derive(serde::Deserialize)]
struct TestLikeForm {
    post_id: String,
}

#[test]
fn test_action_path_constant_generated() {
    assert_eq!(test_like_post_PATH, "/_azumi/action/test_like_post");
}

#[test]
fn test_action_path_constant_usable_in_html() {
    let html = azumi::render_to_string(&html! {
        <form az-action={test_like_post_PATH}>
            <button>"Like"</button>
        </form>
    });
    assert!(
        html.contains(r#"az-action="/_azumi/action/test_like_post""#),
        "Expected az-action with resolved path, got: {}", html
    );
}

// ── Live State Serialization ──────────────────────────────────────────────

#[test]
fn test_live_state_serializes_correctly() {
    #[derive(serde::Serialize, serde::Deserialize, Clone)]
    struct SimpleCounter { count: i32 }
    let counter = SimpleCounter { count: 42 };
    let json = serde_json::to_string(&counter).unwrap();
    let signed = azumi::security::sign_state(&json);
    assert!(signed.contains('|'));
    let verified = azumi::security::verify_state(&signed).unwrap();
    assert!(verified.contains("42"));
}

// ── Configurable State Age ────────────────────────────────────────────────

#[test]
fn test_default_max_state_age() {
    let json = r#"{"count": 1}"#;
    let signed = azumi::security::sign_state(json);
    let result = azumi::security::verify_state(&signed);
    assert!(result.is_ok(), "Fresh state should verify with default age");
}
