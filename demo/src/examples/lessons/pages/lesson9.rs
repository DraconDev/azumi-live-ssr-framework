use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

/// Lesson 9: Introducing Azumi Live
///
/// Compiler-driven optimistic UI - write Rust, get instant updates!

#[azumi::live]
pub struct Counter {
    pub count: i32,
    pub active: bool,
}

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }
}

/// Live component view
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {

        <div class={counter_box}>
            <h2 class={counter_title}>"🚀 Azumi Live Counter"</h2>

            <div class={value} data-bind="count">{state.count}</div>
            <div class={status}>
                "Status: "
                <span data-bind="active" class={status_text}>{if state.active { "Active ✓" } else { "Inactive ✗" }}</span>
            </div>

            <div class={btn_row}>
                <button class={btn_primary} on:click={state.increment}>
                    "+ Increment"
                </button>
                <button class={btn_secondary} on:click={state.decrement}>
                    "- Decrement"
                </button>
                <button class={btn_danger} on:click={state.toggle}>
                    "Toggle Status"
                </button>
            </div>
        </div>

        <style>
            .counter_box {
                padding: "2rem";
                border: "1px solid rgba(255,255,255,0.1)";
                border-radius: "16px";
                background: "rgba(30, 41, 59, 0.6)";
                backdrop-filter: "blur(10px)";
                color: "white";
                text-align: "center";
                max-width: "450px";
                margin: "0 auto";
                box-shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)";
            }
            .counter_title {
                font-size: "1.5rem";
                margin-bottom: "1.5rem";
                color: "#e2e8f0";
            }
            .value {
                font-size: "5rem";
                font-weight: "800";
                margin: "1rem 0";
                background: "linear-gradient(to right, #818cf8, #c084fc)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
            }
            .status {
                font-size: "1.1rem";
                color: "#cbd5e1";
                margin-bottom: "2rem";
            }
            .status_text {
                font-weight: "bold";
                color: "#a5f3fc";
            }
            .btn_row {
                display: "flex";
                gap: "0.75rem";
                justify-content: "center";
                flex-wrap: "wrap";
            }
            .btn_primary {
                padding: "0.75rem 1.5rem";
                background: "linear-gradient(to right, #3b82f6, #2563eb)";
                color: "white";
                border: "none";
                border-radius: "8px";
                cursor: "pointer";
                font-weight: "600";
                transition: "opacity 0.2s";
            }
            .btn_secondary {
                padding: "0.75rem 1.5rem";
                background: "rgba(255, 255, 255, 0.1)";
                color: "white";
                border: "1px solid rgba(255, 255, 255, 0.1)";
                border-radius: "8px";
                cursor: "pointer";
                font-weight: "600";
            }
            .btn_danger {
                padding: "0.75rem 1.5rem";
                background: "linear-gradient(to right, #ef4444, #dc2626)";
                color: "white";
                border: "none";
                border-radius: "8px";
                cursor: "pointer";
                font-weight: "600";
            }
        </style>
    }
}

/// Main lesson demonstration component
/// This uses #[azumi::page] so SEO is automatic!
#[azumi::page(route = "/lesson-9")]
#[allow(non_upper_case_globals)]
pub fn page() -> impl Component {
    let state = Counter {
        count: 0,
        active: true,
    };

    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 9: Introducing Azumi Live"</h1>
                    <p class={subtitle}>"Compiler-driven optimistic UI"</p>
                </header>

                <div class={live_demo_section}>
                    @counter_view(state=&state)
                </div>

                <div class={concepts}>
                    <h3 class={concept_title}>"🎯 Key Concepts"</h3>
                    <ul class={concept_list}>
                        <li class={concept_item}><strong>"#[azumi::live]"</strong> " - Marks struct as reactive state"</li>
                        <li class={concept_item}><strong>"#[azumi::live_impl]"</strong> " - Analyzes mutations at compile time"</li>
                        <li class={concept_item}><strong>"on:click={state.method}"</strong> " - Declarative event binding"</li>
                        <li class={concept_item}><strong>"Zero JS Required"</strong> " - Compiler generates predictions"</li>
                    </ul>
                </div>
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #818cf8, #c084fc)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .subtitle { font-size: "1.25rem"; color: "#94a3b8"; }

                .live_demo_section { margin-bottom: "4rem"; }

                .concepts {
                    background: "rgba(30, 41, 59, 0.5)";
                    padding: "2rem";
                    border-radius: "16px";
                    margin-top: "2rem";
                    border: "1px solid rgba(255,255,255,0.05)";
                    backdrop-filter: "blur(10px)";
                }
                .concept_title { color: "#f1f5f9"; margin-bottom: "1.5rem"; font-size: "1.5rem"; }
                .concept_list { list-style: "none"; padding: "0"; display: "grid"; gap: "1rem"; }
                .concept_item {
                    padding: "1rem";
                    background: "rgba(255,255,255,0.03)";
                    border-radius: "8px";
                    color: "#e2e8f0";
                    font-size: "1.1rem";
                    border-left: "4px solid #818cf8";
                }
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson9_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
