use azumi::{html, Component};

#[azumi::component]
fn ChildComp() -> impl Component {
    html! { <div>"Child"</div> }
}

#[test]
fn test_component_call() {
    let comp = html! { @ChildComp() };
}
