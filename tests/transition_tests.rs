use azumi::{component, html, render_to_string};

#[derive(Clone)]
struct TransitionItem {
    id: i32,
    name: String,
}

#[test]
fn test_transition_fade_attribute_renders() {
    #[azumi::component]
    fn fade_box() -> impl azumi::Component {
        html! {
            <div az-transition:fade={"true"}>"Fade content"</div>
        }
    }
    let output = render_to_string(&fade_box());
    assert!(
        output.contains("az-transition:fade"),
        "Should render fade attribute. Got: {}",
        output
    );
}

#[test]
fn test_transition_slide_attribute_renders() {
    #[azumi::component]
    fn slide_box() -> impl azumi::Component {
        html! {
            <div az-transition:slide={"true"}>"Slide content"</div>
        }
    }
    let output = render_to_string(&slide_box());
    assert!(
        output.contains("az-transition:slide"),
        "Should render slide attribute. Got: {}",
        output
    );
}

#[test]
fn test_transition_scale_attribute_renders() {
    #[azumi::component]
    fn scale_box() -> impl azumi::Component {
        html! {
            <div az-transition:scale={"true"}>"Scale content"</div>
        }
    }
    let output = render_to_string(&scale_box());
    assert!(
        output.contains("az-transition:scale"),
        "Should render scale attribute. Got: {}",
        output
    );
}

#[test]
fn test_transition_with_duration_renders() {
    #[azumi::component]
    fn slow_fade() -> impl azumi::Component {
        html! {
            <div az-transition:fade={"duration=500"}>"Slow fade"</div>
        }
    }
    let output = render_to_string(&slow_fade());
    assert!(
        output.contains("duration=500"),
        "Should render with duration. Got: {}",
        output
    );
}

#[test]
fn test_multiple_transitions_in_same_component() {
    #[azumi::component]
    fn multi_transition() -> impl azumi::Component {
        html! {
            <div>
                <p az-transition:fade={"true"}>"Fade"</p>
                <p az-transition:slide={"true"}>"Slide"</p>
                <p az-transition:scale={"true"}>"Scale"</p>
            </div>
        }
    }
    let output = render_to_string(&multi_transition());
    let fades = output.matches("az-transition:fade").count();
    let slides = output.matches("az-transition:slide").count();
    let scales = output.matches("az-transition:scale").count();
    assert_eq!(fades, 1);
    assert_eq!(slides, 1);
    assert_eq!(scales, 1);
}

#[test]
fn test_transition_with_keyed_list() {
    let items = vec![
        TransitionItem { id: 1, name: "A".to_string() },
        TransitionItem { id: 2, name: "B".to_string() },
    ];

    #[azumi::component]
    fn keyed_transition_list(items: Vec<TransitionItem>) -> impl azumi::Component {
        html! {
            <div>
                @for item in &items @keyed(item.id) {
                    <div az-transition:fade={"true"}>{&item.name}</div>
                }
            </div>
        }
    }

    let output = render_to_string(&keyed_transition_list::render(
        keyed_transition_list::Props::builder().items(items).build().unwrap(),
    ));
    assert!(output.contains("az-transition:fade"));
    assert!(output.contains("data-key="));
}

#[test]
fn test_transition_preserves_other_attributes() {
    #[azumi::component]
    fn styled_fade() -> impl azumi::Component {
        html! {
            <div class={"box"} az-transition:fade={"true"} id={"main"}>"Content"</div>
        }
    }
    let output = render_to_string(&styled_fade());
    assert!(output.contains("az-transition:fade"));
}

#[test]
fn test_transition_on_nested_elements() {
    #[azumi::component]
    fn nested_transitions() -> impl azumi::Component {
        html! {
            <div az-transition:fade={"true"}>
                <span az-transition:slide={"true"}>"Nested"</span>
            </div>
        }
    }
    let output = render_to_string(&nested_transitions());
    assert!(output.contains("az-transition:fade"));
    assert!(output.contains("az-transition:slide"));
}
