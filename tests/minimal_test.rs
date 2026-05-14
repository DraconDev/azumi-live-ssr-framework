use azumi::{html, Component};

#[azumi::component]
fn MyComp() -> impl Component {
    html! {
        <div>"Hello"</div>
    }
}

#[test]
fn test_html_macro() {
    let _comp = html! { @MyComp() };
    // Force a type error to see what the compiler thinks the type is
    let _: () = _comp;
}
