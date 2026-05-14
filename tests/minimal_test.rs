use azumi::{html, Component};

#[azumi::component]
fn MyComp() -> impl Component {
    html! {
        <div>"Hello"</div>
    }
}

#[test]
fn test_my_comp() {
    let _comp = html! { @MyComp() };
}
