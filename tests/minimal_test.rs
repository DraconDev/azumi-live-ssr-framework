use azumi::{html, Component, from_fn_once};
use std::fmt;

#[azumi::component]
fn MyComp() -> impl Component {
    html! {
        <div>"Hello"</div>
    }
}

#[test]
fn test_component_call_workaround() {
    // This is what html! { @MyComp() } should expand to
    let comp = from_fn_once(|f: &mut fmt::Formatter<'_>| -> fmt::Result {
        MyComp::render(
            MyComp::Props::builder().build().expect("Missing required props in wrapper call"),
        )
        .render(f)?;
        Ok(())
    });
    let output = azumi::test::render(&comp);
    assert!(output.contains("Hello"));
}
