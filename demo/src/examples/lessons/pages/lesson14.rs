use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use azumi::prelude::*;

/// Lesson 14: Composition with Live Components
///
/// Building complex UIs by composing live components

#[azumi::live]
pub struct TabState {
    pub active_index: i32,
}

#[azumi::live_impl(component = "tabs_view")]
impl TabState {
    pub fn select_0(&mut self) {
        self.active_index = 0;
    }
    pub fn select_1(&mut self) {
        self.active_index = 1;
    }
    pub fn select_2(&mut self) {
        self.active_index = 2;
    }
}

/// Tabs component
#[azumi::component]
pub fn tabs_view<'a>(state: &'a TabState) -> impl Component + 'a {
    html! {
        <div class={tabs_container}>
            <div class={tab_buttons}>
                <button
                    class={if state.active_index == 0 { format!("{} {}", tab_btn, tab_btn_active) } else { tab_btn.to_string() }}
                    on:click={state.select_0}>
                    "🏠 Overview"
                </button>
                <button
                    class={if state.active_index == 1 { format!("{} {}", tab_btn, tab_btn_active) } else { tab_btn.to_string() }}
                    on:click={state.select_1}>
                    "📊 Features"
                </button>
                <button
                    class={if state.active_index == 2 { format!("{} {}", tab_btn, tab_btn_active) } else { tab_btn.to_string() }}
                    on:click={state.select_2}>
                    "💡 Examples"
                </button>
            </div>
            <div class={tab_content}>
                @if state.active_index == 0 {
                    <div class={tab_panel}>
                        <h3 class={panel_title}>"Overview"</h3>
                        <p>"Azumi Live allows you to build interactive components with zero JavaScript. The compiler analyzes your Rust code and generates optimistic predictions."</p>
                    </div>
                }
                @if state.active_index == 1 {
                    <div class={tab_panel}>
                        <h3 class={panel_title}>"Features"</h3>
                        <ul class={feature_list}>
                            <li class={feature_item}>"Compiler-driven optimistic UI"</li>
                            <li class={feature_item}>"Type-safe state management"</li>
                            <li class={feature_item}>"Zero client-side JavaScript needed"</li>
                            <li class={feature_item}>"Automatic DOM reconciliation"</li>
                        </ul>
                    </div>
                }
                @if state.active_index == 2 {
                    <div class={tab_panel}>
                        <h3 class={panel_title}>"Examples"</h3>
                        <p>"Counters, Like buttons, Forms, Tabs, Accordions - all built with the same pattern!"</p>
                    </div>
                }
            </div>
        </div>
        <style>
            .tabs_container {
                background: "rgba(30, 41, 59, 0.6)";
                backdrop-filter: "blur(10px)";
                border-radius: "16px";
                border: "1px solid rgba(255,255,255,0.05)";
                overflow: "hidden";
                box-shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1)";
            }
            .tab_buttons {
                display: "flex";
                background: "rgba(0, 0, 0, 0.2)";
                border-bottom: "1px solid rgba(255,255,255,0.05)";
            }
            .tab_btn {
                flex: "1";
                padding: "1rem";
                border: "none";
                background: "transparent";
                cursor: "pointer";
                font-size: "1rem";
                color: "#94a3b8";
                transition: "all 0.2s";
                font-weight: "500";
            }
            .tab_btn:hover {
                color: "#e2e8f0";
                background: "rgba(255,255,255,0.02)";
            }
            .tab_btn_active {
                background: "rgba(255, 255, 255, 0.05)";
                color: "#38bdf8";
                font-weight: "bold";
                border-bottom: "2px solid #38bdf8";
            }
            .tab_content {
                padding: "2rem";
                min-height: "200px";
                color: "#cbd5e1";
            }
            .tab_panel {
                animation: "fadeIn 0.3s ease";
            }
            .panel_title {
                color: "#e2e8f0";
                margin-bottom: "1rem";
                font-size: "1.5rem";
            }
            .feature_list {
                padding-left: "1.5rem";
                display: "grid";
                gap: "0.5rem";
            }
            .feature_item {
                color: "#cbd5e1";
            }
            @keyframes fadeIn { from { opacity: "0"; transform: "translateY(5px)"; } to { opacity: "1"; transform: "translateY(0)"; } }
        </style>
    }
}

/// Full page component for Lesson 14
#[azumi::page(route = "/lesson-14")]
#[azumi::component]
pub fn render_page() -> impl Component {
    let tab_state = TabState { active_index: 0 };
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 14: Component Composition"</h1>
                    <p class={subtitle}>"Building complex UIs with live components"</p>
                </header>

                <div class={explanation}>
                    <h3 class={exp_title}>"🧩 Composition Pattern"</h3>
                    <p class={exp_text}>"Each tab switch is a separate action. The compiler generates predictions for each:"</p>
                    <div class={code_block}>
                        <span class={code}>"select_0"</span> " → " <span class={code}>"active_index = 0"</span>
                    </div>
                </div>

                <div class={demo_area}>
                    @tabs_view(state=&tab_state)
                </div>

                @LessonNav(
                    prev_num=Some(13),
                    next_num=Some(15),
                    prev_title="Live Forms",
                    next_title="Full Application",
                )
            </div>
             <style>
                .container { max-width: "800px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #38bdf8, #818cf8)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .subtitle { font-size: "1.25rem"; color: "#94a3b8"; }

                .explanation {
                    background: "rgba(6, 182, 212, 0.1)";
                    padding: "1.5rem";
                    border-radius: "12px";
                    margin: "2rem 0";
                    border: "1px solid rgba(6, 182, 212, 0.2)";
                }
                .exp_title { color: "#22d3ee"; margin-bottom: "0.5rem"; font-size: "1.2rem"; }
                .exp_text { color: "#cbd5e1"; margin-bottom: "1rem"; }

                .code_block {
                    background: "rgba(0,0,0,0.3)";
                    padding: "0.75rem";
                    border-radius: "6px";
                    display: "inline-block";
                    color: "#94a3b8";
                }
                .code { color: "#67e8f9"; font-family: "monospace"; }

                .demo_area { margin: "2rem 0"; }
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson14_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&render_page()))
}
