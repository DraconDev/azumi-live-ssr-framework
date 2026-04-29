//! Live Component Tests
//!
//! Tests for Azumi Live features: state serialization, simulation, az-on
//! Run with: cargo test --features test-utils

use azumi::{html, test};
use serde::{Deserialize, Serialize};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: State Serialization (8 tests)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SimpleCounter {
    count: i32,
}

#[test]
fn test_state_to_json() {
    let state = SimpleCounter { count: 0 };
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("count") && json.contains("0"));
}

#[test]
fn test_state_with_value() {
    let state = SimpleCounter { count: 42 };
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("42"));
}

#[test]
fn test_state_negative() {
    let state = SimpleCounter { count: -10 };
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("-10"));
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BoolState {
    active: bool,
}

#[test]
fn test_state_bool_true() {
    let state = BoolState { active: true };
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("true"));
}

#[test]
fn test_state_bool_false() {
    let state = BoolState { active: false };
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("false"));
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StringState {
    name: String,
}

#[test]
fn test_state_string() {
    let state = StringState {
        name: "Alice".into(),
    };
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("Alice"));
}

#[test]
fn test_state_empty_string() {
    let state = StringState { name: "".into() };
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("name"));
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MultiFieldState {
    count: i32,
    active: bool,
    label: String,
}

#[test]
fn test_state_multiple_fields() {
    let state = MultiFieldState {
        count: 5,
        active: true,
        label: "Test".into(),
    };
    let json = serde_json::to_string(&state).unwrap();
    assert!(json.contains("count") && json.contains("active") && json.contains("label"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: State Simulation (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }

    fn decrement(&mut self) {
        self.count -= 1;
    }

    fn add(&mut self, n: i32) {
        self.count += n;
    }

    fn reset(&mut self) {
        self.count = 0;
    }
}

#[test]
fn test_simulate_increment() {
    let mut sim = test::simulate(Counter { count: 0 });
    sim.act(|s| s.increment());
    assert_eq!(sim.state.count, 1);
}

#[test]
fn test_simulate_decrement() {
    let mut sim = test::simulate(Counter { count: 5 });
    sim.act(|s| s.decrement());
    assert_eq!(sim.state.count, 4);
}

#[test]
fn test_simulate_multiple_increments() {
    let mut sim = test::simulate(Counter { count: 0 });
    sim.act(|s| s.increment());
    sim.act(|s| s.increment());
    sim.act(|s| s.increment());
    assert_eq!(sim.state.count, 3);
}

#[test]
fn test_simulate_add() {
    let mut sim = test::simulate(Counter { count: 10 });
    sim.act(|s| s.add(5));
    assert_eq!(sim.state.count, 15);
}

#[test]
fn test_simulate_reset() {
    let mut sim = test::simulate(Counter { count: 100 });
    sim.act(|s| s.reset());
    assert_eq!(sim.state.count, 0);
}

#[test]
fn test_simulate_sequence() {
    let mut sim = test::simulate(Counter { count: 0 });
    sim.act(|s| s.increment());
    sim.act(|s| s.add(10));
    sim.act(|s| s.decrement());
    assert_eq!(sim.state.count, 10); // 0 + 1 + 10 - 1
}

#[derive(Clone, Debug)]
struct Toggle {
    active: bool,
}

impl Toggle {
    fn toggle(&mut self) {
        self.active = !self.active;
    }
}

#[test]
fn test_simulate_toggle_on() {
    let mut sim = test::simulate(Toggle { active: false });
    sim.act(|s| s.toggle());
    assert!(sim.state.active);
}

#[test]
fn test_simulate_toggle_off() {
    let mut sim = test::simulate(Toggle { active: true });
    sim.act(|s| s.toggle());
    assert!(!sim.state.active);
}

#[test]
fn test_simulate_double_toggle() {
    let mut sim = test::simulate(Toggle { active: false });
    sim.act(|s| s.toggle());
    sim.act(|s| s.toggle());
    assert!(!sim.state.active); // Back to original
}

#[derive(Clone, Debug)]
struct TextInput {
    value: String,
}

impl TextInput {
    fn set_value(&mut self, v: String) {
        self.value = v;
    }
}

#[test]
fn test_simulate_text_input() {
    let mut sim = test::simulate(TextInput { value: "".into() });
    sim.act(|s| s.set_value("Hello".into()));
    assert_eq!(sim.state.value, "Hello");
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: az-on Attribute (8 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_az_on_click() {
    let component = html! {
        <button az-on="click call increment">"Click"</button>
    };
    let html = test::render(&component);
    assert!(html.contains("az-on") && html.contains("click call increment"));
}

#[test]
fn test_az_on_submit() {
    let component = html! {
        <form az-on="submit call handle_submit">"Form"</form>
    };
    let html = test::render(&component);
    assert!(html.contains("az-on") && html.contains("submit call"));
}

#[test]
fn test_az_on_change() {
    let component = html! {
        <input az-on="change call on_change" />
    };
    let html = test::render(&component);
    assert!(html.contains("az-on") && html.contains("change call"));
}

#[test]
fn test_az_on_input() {
    let component = html! {
        <input az-on="input call on_input" />
    };
    let html = test::render(&component);
    assert!(html.contains("az-on") && html.contains("input call"));
}

#[test]
fn test_az_on_with_target() {
    let component = html! {
        <button az-on="click call update -> #target">"Update"</button>
    };
    let html = test::render(&component);
    assert!(html.contains("az-on") && html.contains("#target"));
}

#[test]
fn test_multiple_az_on_elements() {
    let component = html! {
        <div>
            <button az-on="click call method1">"Button 1"</button>
            <button az-on="click call method2">"Button 2"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("method1") && html.contains("method2"));
}

#[test]
fn test_az_on_in_form() {
    let component = html! {
        <form az-on="submit call save">
            <input type="text" name="name" />
            <button type="submit">"Save"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("az-on") && html.contains("save"));
}

#[test]
fn test_data_predict_attribute() {
    let component = html! {
        <button data-predict="count = count + 1">"Increment"</button>
    };
    let html = test::render(&component);
    assert!(html.contains("data-predict"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Security (6 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sign_state() {
    let json = r#"{"count":42}"#;
    let signed = azumi::security::sign_state(json);
    // Should contain separator
    assert!(signed.contains("|"));
}

#[test]
fn test_verify_valid_state() {
    let json = r#"{"count":42}"#;
    let signed = azumi::security::sign_state(json);
    let result = azumi::security::verify_state(&signed);
    assert!(result.is_ok());
}

#[test]
fn test_verify_returns_json() {
    let json = r#"{"count":42}"#;
    let signed = azumi::security::sign_state(json);
    let verified = azumi::security::verify_state(&signed).unwrap();
    assert!(verified.contains("42"));
}

#[test]
fn test_tampered_state_fails() {
    let json = r#"{"count":42}"#;
    let signed = azumi::security::sign_state(json);
    // Tamper with the state
    let tampered = signed.replace("42", "999");
    let result = azumi::security::verify_state(&tampered);
    assert!(result.is_err());
}

#[test]
fn test_missing_signature_fails() {
    let json = r#"{"count":42}"#;
    let result = azumi::security::verify_state(json);
    assert!(result.is_err());
}

#[test]
fn test_different_states_different_signatures() {
    let json1 = r#"{"count":1}"#;
    let json2 = r#"{"count":2}"#;
    let sig1 = azumi::security::sign_state(json1);
    let sig2 = azumi::security::sign_state(json2);
    assert_ne!(sig1, sig2);
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 6: Local State Tests
// ════════════════════════════════════════════════════════════════════════════

mod local_state_tests {
    use super::*;

    #[azumi::live]
    pub struct LocalCounterState {
        pub count: i32,
        #[local]
        pub input_value: String,
    }

    impl Default for LocalCounterState {
        fn default() -> Self {
            Self {
                count: 0,
                input_value: String::new(),
            }
        }
    }

    #[azumi::live]
    pub struct ServerOnlyState {
        pub name: String,
        #[local]
        pub scroll_pos: i32,
        #[computed]
        pub full_greeting: String,
    }

    impl Default for ServerOnlyState {
        fn default() -> Self {
            Self {
                name: String::new(),
                scroll_pos: 0,
                full_greeting: String::new(),
            }
        }
    }

    #[test]
    fn test_local_scope_only_includes_local_fields() {
        let state = LocalCounterState {
            count: 10,
            input_value: "test".to_string(),
        };
        let local_json = state.to_local_scope();
        assert!(local_json.contains("input_value"));
        assert!(!local_json.contains("count"));
    }

    #[test]
    fn test_scope_excludes_local_fields() {
        let state = LocalCounterState {
            count: 10,
            input_value: "test".to_string(),
        };
        let scope = state.to_scope();
        assert!(scope.contains("count"));
        assert!(!scope.contains("input_value"));
    }

    #[test]
    fn test_local_scope_empty_when_no_local_fields() {
        #[azumi::live]
        pub struct NoLocalState {
            pub value: i32,
        }
        impl Default for NoLocalState {
            fn default() -> Self {
                Self { value: 0 }
            }
        }
        let state = NoLocalState { value: 5 };
        let local = state.to_local_scope();
        assert!(local.is_empty());
    }

    #[test]
    fn test_computed_fields_not_serialized() {
        #[azumi::live]
        pub struct ComputedState {
            pub first: String,
            pub last: String,
            #[computed]
            pub full: String,
        }
        impl Default for ComputedState {
            fn default() -> Self {
                Self {
                    first: String::new(),
                    last: String::new(),
                    full: String::new(),
                }
            }
        }
        let state = ComputedState {
            first: "John".to_string(),
            last: "Doe".to_string(),
            full: "John Doe".to_string(),
        };
        let scope = state.to_scope();
        assert!(scope.contains("first"));
        assert!(scope.contains("last"));
        assert!(!scope.contains("full"));
    }

    #[test]
    fn test_local_and_computed_fields() {
        let state = ServerOnlyState {
            name: "Test".to_string(),
            scroll_pos: 100,
            full_greeting: "Hello Test".to_string(),
        };
        let local = state.to_local_scope();
        let scope = state.to_scope();

        assert!(local.contains("scroll_pos"));
        assert!(!local.contains("name"));
        assert!(!local.contains("full_greeting"));

        assert!(scope.contains("name"));
        assert!(!scope.contains("scroll_pos"));
        assert!(!scope.contains("full_greeting"));
    }
}
