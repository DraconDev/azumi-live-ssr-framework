use azumi::{html, test, Component};

// ════════════════════════════════════════════════════════════════════════════
// Counter Component Module
// ════════════════════════════════════════════════════════════════════════════

mod counter {
    use super::*;

    #[azumi::live]
    #[derive(Default)] // Serialize/Deserialize are added by #[azumi::live]
    pub struct CounterState {
        pub count: i32,
        pub active: bool,
    }

    #[azumi::live_impl(component = "counter_view")]
    impl CounterState {
        pub fn increment(&mut self) {
            self.count += 1;
        }

        pub fn toggle(&mut self) {
            self.active = !self.active;
        }

        #[azumi::predict("count = 0")]
        pub fn reset(&mut self) {
            self.count = 0;
        }
    }

    #[azumi::component]
    pub fn counter_view<'a>(state: &'a CounterState) -> impl Component + 'a {
        html! {
            <div>
                <span data-bind="count">{state.count}</span>
                <button on:click={state.increment} data-predict="count = count + 1">"+1"</button>
                <button on:click={state.toggle} data-predict="active = !active">"Toggle"</button>
                <button on:click={state.reset} data-predict="count = 0">"Reset"</button>
            </div>
        }
    }
}

use counter::*;

// ════════════════════════════════════════════════════════════════════════════
// Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_live_scope_and_struct_attributes() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // Check for az-scope and az-struct
    assert!(output.contains("az-scope=\""), "az-scope attribute missing");
    assert!(
        output.contains("az-struct=\"CounterState\""),
        "az-struct attribute missing"
    );
}

#[test]
fn test_automatic_predictions() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // az-predictions should be present on the scope div
    assert!(
        output.contains("az-predictions="),
        "az-predictions attribute missing from scope div"
    );

    // Should contain the prediction data as JSON
    assert!(
        output.contains("count = count + 1"),
        "increment prediction missing from az-predictions"
    );
    assert!(
        output.contains("active = !active"),
        "toggle prediction missing from az-predictions"
    );
    assert!(
        output.contains("count = 0"),
        "reset prediction missing from az-predictions"
    );
}

#[test]
fn test_manual_predictions() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // reset() has #[azumi::predict("count = 0")]
    // Manual data-predict still works alongside auto-detected ones
    assert!(
        output.contains("data-predict=\"count = 0\""),
        "Manual prediction for reset missing"
    );
}

#[test]
fn test_event_binding_rendering() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // on:click={state.increment} -> az-on="click call increment"
    assert!(
        output.contains("az-on=\"click call increment\""),
        "Event binding for increment missing"
    );
}

#[test]
fn test_data_bind_attribute() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    assert!(
        output.contains("data-bind=\"count\""),
        "data-bind attribute missing"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// LiveStateMetadata::predictions() Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_predictions_metadata_count() {
    let predictions = <CounterState as azumi::LiveStateMetadata>::predictions();
    assert_eq!(
        predictions.len(),
        3,
        "Expected 3 predictions, found {}: {:?}",
        predictions.len(),
        predictions
    );
}

#[test]
fn test_predictions_metadata_increment() {
    let predictions = <CounterState as azumi::LiveStateMetadata>::predictions();
    let pred_map: std::collections::HashMap<_, _> = predictions.iter().copied().collect();
    
    assert!(
        pred_map.contains_key("increment"),
        "increment prediction missing: {:?}",
        predictions
    );
    assert_eq!(
        pred_map.get("increment"),
        Some(&"count = count + 1"),
        "increment prediction DSL mismatch"
    );
}

#[test]
fn test_predictions_metadata_toggle() {
    let predictions = <CounterState as azumi::LiveStateMetadata>::predictions();
    let pred_map: std::collections::HashMap<_, _> = predictions.iter().copied().collect();
    
    assert!(
        pred_map.contains_key("toggle"),
        "toggle prediction missing: {:?}",
        predictions
    );
    assert_eq!(
        pred_map.get("toggle"),
        Some(&"active = !active"),
        "toggle prediction DSL mismatch"
    );
}

#[test]
fn test_predictions_metadata_reset() {
    let predictions = <CounterState as azumi::LiveStateMetadata>::predictions();
    let pred_map: std::collections::HashMap<_, _> = predictions.iter().copied().collect();
    
    assert!(
        pred_map.contains_key("reset"),
        "reset prediction missing: {:?}",
        predictions
    );
    assert_eq!(
        pred_map.get("reset"),
        Some(&"count = 0"),
        "reset prediction DSL mismatch"
    );
}

#[test]
fn test_predictions_metadata_struct_name() {
    assert_eq!(
        <CounterState as azumi::LiveStateMetadata>::struct_name(),
        "CounterState",
        "struct_name mismatch"
    );
}

#[test]
fn test_predictions_metadata_empty_for_no_methods() {
    let predictions = <NestedState as azumi::LiveStateMetadata>::predictions();
    assert!(
        predictions.is_empty(),
        "Expected no predictions for NestedState (do_nothing has no mutations), found {:?}",
        predictions
    );
}

#[test]
fn test_az_predictions_attribute_on_scope() {
    let state = CounterState::default();
    let comp = html! { @counter_view(state = &state) };
    let output = test::render(&comp);

    // The scope div should have az-predictions with the prediction data
    assert!(
        output.contains("az-predictions="),
        "az-predictions attribute should be present on scope div"
    );
}

#[test]
fn test_no_az_predictions_when_empty() {
    let state = NestedState::default();
    let comp = html! { @nested_view(state = &state) };
    let output = test::render(&comp);
    
    eprintln!("DEBUG OUTPUT: {}", output);

    // NestedState has no predictions, so az-predictions should not be present
    assert!(
        !output.contains("az-predictions="),
        "az-predictions should NOT be present when there are no predictions"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Complex Live State (Nested/Multiple)
// ════════════════════════════════════════════════════════════════════════════

mod nested {
    use super::*;

    #[azumi::live]
    #[derive(Default)]
    pub struct NestedState {
        pub child: CounterState,
    }

    #[azumi::live_impl(component = "nested_view")]
    impl NestedState {
        pub fn do_nothing(&mut self) {}
    }

    #[azumi::component]
    pub fn nested_view<'a>(state: &'a NestedState) -> impl Component + 'a {
        html! {
            <div>
                @counter_view(state = &state.child)
                <button on:click={state.do_nothing}>"Action"</button>
            </div>
        }
    }
}

use nested::*;

#[test]
fn test_nested_live_scopes() {
    let state = NestedState::default();
    let comp = html! { @nested_view(state = &state) };
    let output = test::render(&comp);

    // Should have two az-scope attributes (one for NestedState, one for CounterState)
    let scope_count = output.matches("az-scope=\"").count();
    assert!(
        scope_count >= 2,
        "Expected at least 2 az-scope attributes, found {}",
        scope_count
    );
    assert!(output.contains("az-struct=\"NestedState\""));
    assert!(output.contains("az-struct=\"CounterState\""));
}
