use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// Nested Macro Stress Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_nested_macro_expansion() {
    let title = "Nested Title";
    let desc = "Nested Desc";
    let comp = html! {
        <div>
            {
                azumi::head! {
                    title: title,
                    description: desc,
                    image: "/nested.jpg",
                    url: "https://nested.com",
                    type: "article"
                }
            }
        </div>
    };
    let output = test::render(&comp);
    assert!(output.contains("<title>Nested Title"));
    assert!(output.contains("content=\"Nested Desc\""));
}

#[azumi::component]
fn Wrapper(children: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div class:external="wrapper">
            {children}
        </div>
    }
}

#[test]
fn test_head_inside_component() {
    let comp = html! {
        @Wrapper {
            {
                azumi::head! {
                    title: "Wrapped",
                    description: "Wrapped Desc",
                    image: "/wrapped.jpg",
                    url: "https://wrapped.com",
                    type: "website"
                }
            }
        }
    };
    let output = test::render(&comp);
    assert!(output.contains("wrapper"));
    assert!(output.contains("<title>Wrapped"));
}
