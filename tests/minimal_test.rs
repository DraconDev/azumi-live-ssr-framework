use azumi::{html, Component};

#[azumi::component]
fn MyComp() -> impl Component {
    html! {
        <div>"Hello"</div>
    }
}

#[test]
fn test_html_macro() {
    let comp = html! { @MyComp() };
    compile_error!(stringify!(comp));
}
