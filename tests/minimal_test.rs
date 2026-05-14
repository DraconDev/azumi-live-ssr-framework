use azumi::{html, Component, from_fn_once};
use std::fmt;

#[test]
fn test_same_quote_context() {
    // Test if 'f' from a manually written closure works
    // This mimics what the macro should produce
    let comp = from_fn_once(move |f: &mut fmt::Formatter<'_>| {
        write!(f, "<div>Hello</div>")?;
        Ok(())
    });
    let output = azumi::test::render(&comp);
    assert!(output.contains("Hello"));
}

#[test]
fn test_html_no_component() {
    let comp = html! {
        <div>"Hello"</div>
    };
    let output = azumi::test::render(&comp);
    assert!(output.contains("Hello"));
}

#[test]
fn test_html_with_expression_component() {
    fn my_comp() -> impl Component {
        from_fn_once(|f: &mut fmt::Formatter<'_>| {
            write!(f, "<div>Hello</div>")?;
            Ok(())
        })
    }
    let comp = html! {
        <div>
            @{my_comp()}
        </div>
    };
    let output = azumi::test::render(&comp);
    assert!(output.contains("Hello"));
}
