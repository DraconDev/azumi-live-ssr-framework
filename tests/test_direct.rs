use azumi::{html, Component, RenderWrapper, from_fn_once};

#[azumi::component]
fn ChildComp() -> impl Component {
    html! { <div>"Child"</div> }
}

// Test: manually write what the macro should expand to
#[test]
fn test_manual_expansion() {
    let comp = azumi::from_fn_once(move |f| {
        azumi::RenderWrapper(&ChildComp::render(
            ChildComp::Props::builder().build().unwrap_or_else(|| panic!("Failed to build props for component {}", "ChildComp"))
        )).render_azumi(f)?;
        Ok(())
    });
}
