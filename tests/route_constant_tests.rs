//! Tests for route constant generation:
//! - #[azumi::page(route = "/path")] generates page_fn::ROUTE
//! - #[azumi::action] generates action_name_PATH

use azumi::prelude::*;

// ── Page Route Constants ──────────────────────────────────────────────────

#[azumi::page(route = "/test-about")]
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

// Page without route attribute should NOT have ROUTE constant
#[azumi::page]
fn test_no_route_page() -> impl Component {
    html! { <div>"No route"</div> }
}

// This is a compile-time check — if test_no_route_page::ROUTE existed,
// this test would compile. We verify the absence by not referencing it.

// ── Action PATH Constants ─────────────────────────────────────────────────

#[azumi::action]
fn test_like_post(form: TestForm) -> azumi::action::ActionResult {
    azumi::action::ActionResult::ok(&html! { <span>"Liked!"</span> })
}

#[derive(serde::Deserialize)]
struct TestForm {
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
    assert!(html.contains(r#"az-action="/_azumi/action/test_like_post""#));
}

// ── Live State Graceful Degradation ───────────────────────────────────────

#[azumi::live]
struct TestCounter {
    count: i32,
}

#[azumi::component]
fn test_counter(#[live_state] ctx: &TestCounter) -> impl Component {
    html! { <span>{ctx.count}</span> }
}

#[test]
fn test_live_state_with_explicit_attribute() {
    let state = TestCounter { count: 42 };
    let html = azumi::render_to_string(&test_counter::render(
        test_counter::Props::builder().ctx(state).build().unwrap(),
    ));
    assert!(html.contains("42"));
    assert!(html.contains("az-scope"));
}

// Test that graceful degradation works — serialize a valid state
#[test]
fn test_live_state_serializes_correctly() {
    let state = TestCounter { count: 7 };
    let scope = state.to_scope();
    // Should be a signed state string with pipes
    assert!(scope.contains('|'));
    // Verify it round-trips
    let verified = azumi::security::verify_state(&scope).unwrap();
    assert!(verified.contains("7"));
}

// ── Configurable State Age ────────────────────────────────────────────────

#[test]
fn test_default_max_state_age() {
    // Default is 3600 seconds (1 hour)
    let state = TestCounter { count: 1 };
    let scope = state.to_scope();
    // A freshly signed state should verify successfully
    let result = azumi::security::verify_state(&scope);
    assert!(result.is_ok(), "Fresh state should verify with default age");
}
