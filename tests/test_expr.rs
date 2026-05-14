use azumi::{html, Component};

#[azumi::component]
fn ChildComp() -> impl Component {
    html! { <div>"Child"</div> }
}

#[test]
fn test_expr_interpolation() {
    let child = ChildComp::render(ChildComp::Props::builder().build().unwrap());
    let comp = html! { @{child} };
}

#[test]
fn test_plain_text() {
    let comp = html! { <div>"Hello"</div> };
}
