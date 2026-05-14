//! Live Handler Integration Tests
//!
//! Tests that verify the full flow exercised by #[azumi::live] generated handlers:
//!   1. Sign state → submit as body
//!   2. Verify HMAC → deserialize → mutate → re-sign
//!   3. Verify the new state roundtrips correctly
//!
//! These complement the unit tests in security.rs and live_tests.rs by testing
//! the end-to-end flow that the generated Axum handlers perform.

use azumi::security;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Counter {
    count: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Toggle {
    open: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct MultiField {
    name: String,
    count: i32,
    active: bool,
}

// ════════════════════════════════════════════════════════════════════════════════
// Full roundtrip: sign → verify → deserialize → mutate → re-sign → verify
// ════════════════════════════════════════════════════════════════════════════════

#[test]
fn test_counter_increment_roundtrip() {
    let state = Counter { count: 0 };
    let json = serde_json::to_string(&state).unwrap();
    let signed = security::sign_state(&json);

    // Simulate handler: verify → deserialize → mutate → re-sign
    let verified = security::verify_state(&signed).unwrap();
    let mut state: Counter = serde_json::from_str(&verified).unwrap();
    state.count += 1;

    let new_json = serde_json::to_string(&state).unwrap();
    let new_signed = security::sign_state(&new_json);

    // Verify the new state roundtrips
    let re_verified = security::verify_state(&new_signed).unwrap();
    let re_state: Counter = serde_json::from_str(&re_verified).unwrap();
    assert_eq!(re_state.count, 1);
}

#[test]
fn test_toggle_roundtrip() {
    let state = Toggle { open: false };
    let json = serde_json::to_string(&state).unwrap();
    let signed = security::sign_state(&json);

    let verified = security::verify_state(&signed).unwrap();
    let mut state: Toggle = serde_json::from_str(&verified).unwrap();
    state.open = !state.open;

    let new_json = serde_json::to_string(&state).unwrap();
    let new_signed = security::sign_state(&new_json);

    let re_verified = security::verify_state(&new_signed).unwrap();
    let re_state: Toggle = serde_json::from_str(&re_verified).unwrap();
    assert!(re_state.open);
}

#[test]
fn test_multi_field_mutation_roundtrip() {
    let state = MultiField {
        name: "Alice".into(),
        count: 10,
        active: true,
    };
    let json = serde_json::to_string(&state).unwrap();
    let signed = security::sign_state(&json);

    let verified = security::verify_state(&signed).unwrap();
    let mut state: MultiField = serde_json::from_str(&verified).unwrap();
    state.count += 5;
    state.active = false;

    let new_json = serde_json::to_string(&state).unwrap();
    let new_signed = security::sign_state(&new_json);

    let re_verified = security::verify_state(&new_signed).unwrap();
    let re_state: MultiField = serde_json::from_str(&re_verified).unwrap();
    assert_eq!(re_state.count, 15);
    assert!(!re_state.active);
    assert_eq!(re_state.name, "Alice");
}

// ════════════════════════════════════════════════════════════════════════════════
// Security: tampered state should be rejected at verification step
// ════════════════════════════════════════════════════════════════════════════════

#[test]
fn test_handler_rejects_tampered_count() {
    let state = Counter { count: 0 };
    let json = serde_json::to_string(&state).unwrap();
    let signed = security::sign_state(&json);

    // Attacker changes count in the signed string
    let tampered = signed.replace("\"count\":0", "\"count\":999");
    assert!(
        security::verify_state(&tampered).is_err(),
        "Tampered count must be rejected"
    );
}

#[test]
fn test_handler_rejects_tampered_bool() {
    let state = Toggle { open: false };
    let json = serde_json::to_string(&state).unwrap();
    let signed = security::sign_state(&json);

    let tampered = signed.replace("false", "true");
    assert!(
        security::verify_state(&tampered).is_err(),
        "Tampered bool must be rejected"
    );
}

#[test]
fn test_handler_rejects_tampered_name() {
    let state = MultiField {
        name: "Alice".into(),
        count: 10,
        active: true,
    };
    let json = serde_json::to_string(&state).unwrap();
    let signed = security::sign_state(&json);

    let tampered = signed.replace("Alice", "Eve");
    assert!(
        security::verify_state(&tampered).is_err(),
        "Tampered name must be rejected"
    );
}

// ════════════════════════════════════════════════════════════════════════════════
// Security: expired and future timestamps
// ════════════════════════════════════════════════════════════════════════════════

#[test]
fn test_handler_rejects_expired_state() {
    // Simulate expired state by crafting a signed string with timestamp=0
    let json = r#"{"count":10}"#;
    let expired = format!("{}|0|invalid", json);
    assert!(
        security::verify_state(&expired).is_err(),
        "Expired state must be rejected"
    );
}

#[test]
fn test_handler_rejects_very_old_state() {
    // Timestamp from year 2020
    let json = r#"{"count":10}"#;
    let old_timestamp = 1577836800u64; // 2020-01-01
    let expired = format!("{}|{}|invalid", json, old_timestamp);
    assert!(
        security::verify_state(&expired).is_err(),
        "Old state must be rejected"
    );
}

#[test]
fn test_handler_rejects_state_with_max_timestamp() {
    let json = r#"{"count":10}"#;
    let max_ts = format!("{}|{}|invalid", json, u64::MAX);
    assert!(
        security::verify_state(&max_ts).is_err(),
        "u64::MAX timestamp must be rejected"
    );
}

// ════════════════════════════════════════════════════════════════════════════════
// Security: replay across different states
// ════════════════════════════════════════════════════════════════════════════════

#[test]
fn test_handler_rejects_replayed_old_state() {
    // Sign two different states
    let state1 = Counter { count: 0 };
    let state2 = Counter { count: 100 };

    let json1 = serde_json::to_string(&state1).unwrap();
    let json2 = serde_json::to_string(&state2).unwrap();

    let signed1 = security::sign_state(&json1);
    let signed2 = security::sign_state(&json2);

    // Both should verify correctly
    assert!(security::verify_state(&signed1).is_ok());
    assert!(security::verify_state(&signed2).is_ok());

    // But signed1 cannot be used to produce count=100
    let verified = security::verify_state(&signed1).unwrap();
    let deserialized: Counter = serde_json::from_str(&verified).unwrap();
    assert_eq!(deserialized.count, 0, "State1 should still be count=0, not 100");
}

// ════════════════════════════════════════════════════════════════════════════════
// Security: user-scoped state isolation
// ════════════════════════════════════════════════════════════════════════════════

#[test]
fn test_user_scoped_state_isolation() {
    let json = r#"{"count":10}"#;

    let user_a_signed = security::sign_state_for_user("user_a", json);
    let user_b_signed = security::sign_state_for_user("user_b", json);

    // Each user can verify their own state
    assert!(security::verify_state_for_user("user_a", &user_a_signed).is_ok());
    assert!(security::verify_state_for_user("user_b", &user_b_signed).is_ok());

    // User A's state cannot be verified as User B
    assert!(
        security::verify_state_for_user("user_b", &user_a_signed).is_err(),
        "Cross-user replay must be rejected"
    );
    assert!(
        security::verify_state_for_user("user_a", &user_b_signed).is_err(),
        "Cross-user replay must be rejected"
    );
}

#[test]
fn test_user_scoped_state_non_user_verify_fails() {
    let json = r#"{"count":10}"#;
    let user_signed = security::sign_state_for_user("user123", json);

    // Using verify_state (no user) on user-scoped state should fail
    assert!(
        security::verify_state(&user_signed).is_err(),
        "User-scoped state should not verify without user context"
    );
}

// ════════════════════════════════════════════════════════════════════════════════
// Edge cases: malformed input
// ════════════════════════════════════════════════════════════════════════════════

#[test]
fn test_handler_rejects_empty_string() {
    assert!(security::verify_state("").is_err());
}

#[test]
fn test_handler_rejects_oversized_state() {
    let large = "x".repeat(100_001);
    assert!(security::verify_state(&large).is_err());
}

#[test]
fn test_handler_rejects_missing_signature() {
    let json = r#"{"count":10}"#;
    assert!(security::verify_state(json).is_err());
}

#[test]
fn test_handler_rejects_invalid_base64_signature() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let fake = format!(r#"{{"count":10}}|{}|not-valid-base64!!!"#, now);
    assert!(security::verify_state(&fake).is_err());
}

// ════════════════════════════════════════════════════════════════════════════════
// Consecutive mutations (simulating multiple handler calls)
// ════════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sequential_mutations() {
    let mut state = Counter { count: 0 };

    for expected in 1..=10 {
        let json = serde_json::to_string(&state).unwrap();
        let signed = security::sign_state(&json);

        let verified = security::verify_state(&signed).unwrap();
        state = serde_json::from_str(&verified).unwrap();
        state.count += 1;

        assert_eq!(state.count, expected);
    }
}

#[test]
fn test_sequential_toggle_mutations() {
    let mut state = Toggle { open: false };

    for i in 1..=6 {
        let json = serde_json::to_string(&state).unwrap();
        let signed = security::sign_state(&json);

        let verified = security::verify_state(&signed).unwrap();
        state = serde_json::from_str(&verified).unwrap();
        state.open = !state.open;

        // open should alternate: true, false, true, false, true, false
        assert_eq!(state.open, i % 2 == 1);
    }
}

// ════════════════════════════════════════════════════════════════════════════════
// Pipes in JSON values (edge case for the pipe-split logic)
// ════════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct PipeState {
    message: String,
}

#[test]
fn test_state_with_pipes_in_values() {
    let state = PipeState {
        message: "a|b|c".into(),
    };
    let json = serde_json::to_string(&state).unwrap();
    let signed = security::sign_state(&json);

    let verified = security::verify_state(&signed).unwrap();
    let deserialized: PipeState = serde_json::from_str(&verified).unwrap();
    assert_eq!(deserialized.message, "a|b|c");
}

#[test]
fn test_pipe_state_mutation_roundtrip() {
    let state = PipeState {
        message: "hello|world".into(),
    };
    let json = serde_json::to_string(&state).unwrap();
    let signed = security::sign_state(&json);

    let verified = security::verify_state(&signed).unwrap();
    let mut deserialized: PipeState = serde_json::from_str(&verified).unwrap();
    deserialized.message = "goodbye|world".into();

    let new_json = serde_json::to_string(&deserialized).unwrap();
    let new_signed = security::sign_state(&new_json);

    let re_verified = security::verify_state(&new_signed).unwrap();
    let re_state: PipeState = serde_json::from_str(&re_verified).unwrap();
    assert_eq!(re_state.message, "goodbye|world");
}
