use azumi::{html, Component};

#[azumi::component]
fn ChildComp() -> impl Component {
    html! {
        <div>"Child"</div>
    }
}

#[azumi::component]
fn ParentComp() -> impl Component {
    html! {
        <div>
            @ChildComp()
        </div>
    }
}

#[test]
fn test_parent_renders() {
    let comp = ParentComp::render(ParentComp::Props::builder().build().expect("missing"));
    let output = azumi::test::render(&comp);
    assert!(output.contains("Child"));
}

#[test]
fn test_component_call_top_level() {
    let comp = html! { @ChildComp() };
    let output = azumi::test::render(&comp);
    assert!(output.contains("Child"));
}

#[test]
fn test_component_call_with_props() {
    #[azumi::component]
    fn Greet(name: String) -> impl Component {
        html! {
            <div>"Hello "{&name}</div>
        }
    }
    let comp = html! { @Greet(name = "World".to_string()) };
    let output = azumi::test::render(&comp);
    assert!(output.contains("Hello"));
}
