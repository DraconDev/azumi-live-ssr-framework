use azumi::{html, Component, from_fn_once, RenderWrapper};
use std::fmt;

#[azumi::component]
fn MyComp() -> impl Component {
    html! {
        <div>"Hello"</div>
    }
}

// Test using RenderWrapper instead of direct .render()
#[test]
fn test_workaround_render_wrapper() {
    let comp = from_fn_once(move |f: &mut fmt::Formatter<'_>| {
        RenderWrapper(&MyComp::render(
            MyComp::Props::builder().build().expect("Missing required props"),
        ))
        .render_azumi(f)?;
        Ok(())
    });
    let output = azumi::test::render(&comp);
    assert!(output.contains("Hello"));
}
