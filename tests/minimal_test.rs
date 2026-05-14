use azumi::{html, Component, from_fn_once, FnComponent};
use std::fmt;

#[azumi::component]
fn ChildComp() -> impl Component {
    html! {
        <div>"Child"</div>
    }
}

// Test: helper function approach
fn render_component<C: Component>(comp: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    comp.render(f)
}

#[test]
fn test_helper_function() {
    let comp = from_fn_once(move |f: &mut fmt::Formatter<'_>| {
        render_component(&ChildComp::render(
            ChildComp::Props::builder().build().expect("missing"),
        ), f)?;
        Ok(())
    });
    let output = azumi::test::render(&comp);
    assert!(output.contains("Child"));
}
