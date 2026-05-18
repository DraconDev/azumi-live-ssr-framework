use axum::response::{Html, IntoResponse};
use azumi::prelude::*;

/// Component to be tested
#[azumi::component]
fn SimpleCard(_title: String, _content: String) -> impl Component {
    html! {
        <div class={card}>
            <h2 class={card_title}>{_title}</h2>
            <p class={card_content}>{_content}</p>
        </div>
        <style>
            .card { padding: "1rem"; border: "1px solid #ddd"; }
            .card_title { font-size: "1.25rem"; font-weight: "bold"; }
            .card_content { color: "#666"; }
        </style>
    }
}

/// Live component to be tested
#[azumi::live]
pub struct TestCounter {
    pub count: i32,
}

#[azumi::live_impl(component = "counter_view")]
impl TestCounter {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}

#[azumi::component]
fn counter_view<'a>(state: &'a TestCounter) -> impl Component + 'a {
    html! {
        <div>
            <span class={count_class}>{state.count}</span>
            <button on:click={state.increment}>"Inc"</button>
        </div>
        <style>
            .count_class { font-size: "1.5rem"; font-weight: "bold"; }
        </style>
    }
}

// -----------------------------------------------------------------------------
// UNIT TESTS - This is what we are verifying!
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use azumi::test;

    #[test]
    fn test_simple_card_render() {
        let card = SimpleCard::Props::builder()
            ._title("Test Title".to_string())
            ._content("Test Content".to_string())
            .build()
            .unwrap();

        let html = test::render(&SimpleCard::render(card));

        // Assert text content
        test::assert_selector(&html, ".card_title", Some("Test Title"));
        test::assert_selector(&html, ".card_content", Some("Test Content"));
    }

    #[test]
    fn test_counter_logic() {
        let mut simulator = test::simulate(TestCounter { count: 0 });

        // Initial state
        assert_eq!(simulator.state.count, 0);

        // Perform action
        simulator.act(TestCounter::increment);

        // Verify state change
        assert_eq!(simulator.state.count, 1);
    }
}

// -----------------------------------------------------------------------------
// Lesson Page (Documentation)
// -----------------------------------------------------------------------------

#[azumi::page(route = "/lesson-17")]
#[azumi::component]
pub fn render_page() -> impl Component {
    html! {
        @crate::examples::lessons::components::layout::DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 17: Testing"</h1>
                    <p class={subtitle}>"Verify components and logic without a browser using the test harness."</p>
                </header>

                <div class={section}>
                    <h2 class={section_title}>"1. Testing Rendering"</h2>
                    <div class={code_wrapper}>
                        <pre class={code}>
    "#[test]\n"
    "fn test_render() {\n"
    "    let html = test::render(&MyComponent { ... });\n"
    "    test::assert_selector(&html, \".title\", Some(\"Hello\"));\n"
    "}"
                        </pre>
                    </div>
                </div>

                <div class={section}>
                    <h2 class={section_title}>"2. Testing Live Logic"</h2>
                    <div class={code_wrapper}>
                        <pre class={code}>
    "#[test]\n"
    "fn test_logic() {\n"
    "    let mut sim = test::simulate(State { ... });\n"
    "    sim.act(State::increment);\n"
    "    assert_eq!(sim.state.count, 1);\n"
    "}"
                        </pre>
                    </div>
                </div>

                <div class={success_box}>
                    <div class={success_icon}>"✓"</div>
                    <p>"If you are seeing this page, the unit tests in this file passed!"</p>
                </div>
            </div>
            <style>
                .container { max-width: "800px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #22c55e, #16a34a)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .subtitle { color: "#94a3b8"; font-size: "1.2rem"; }

                .section { margin-bottom: "2rem"; }
                .section_title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.5rem"; border-bottom: "1px solid rgba(255,255,255,0.1)"; padding-bottom: "0.5rem"; }

                .code_wrapper {
                    background: "rgba(0, 0, 0, 0.3)";
                    border-radius: "12px";
                    border: "1px solid rgba(255,255,255,0.05)";
                    overflow: "hidden";
                }
                .code {
                    padding: "1.5rem";
                    margin: "0";
                    color: "#cbd5e1";
                    font-family: "monospace";
                    font-size: "0.9rem";
                    line-height: "1.5";
                    overflow-x: "auto";
                }

                .success_box {
                    background: "rgba(34, 197, 94, 0.1)";
                    border: "1px solid rgba(34, 197, 94, 0.2)";
                    border-radius: "12px";
                    padding: "1.5rem";
                    display: "flex";
                    align-items: "center";
                    gap: "1rem";
                    margin-top: "3rem";
                }
                .success_icon {
                    width: "32px"; height: "32px";
                    background: "#22c55e"; color: "white";
                    border-radius: "50%";
                    display: "flex"; align-items: "center"; justify-content: "center";
                    font-weight: "bold"; font-size: "1.2rem";
                }
            </style>
        }
    }
}

pub async fn lesson17_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&render_page()))
}
