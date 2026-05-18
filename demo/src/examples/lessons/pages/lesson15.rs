use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use azumi::prelude::*;

/// Lesson 15: Full Application Pattern
///
/// Building a complete interactive todo app

#[azumi::live]
pub struct TodoApp {
    pub show_completed: bool,
    pub item_count: i32,
}

#[azumi::live_impl(component = "todo_app_view")]
impl TodoApp {
    pub fn toggle_filter(&mut self) {
        self.show_completed = !self.show_completed;
    }

    pub fn add_item(&mut self) {
        self.item_count += 1;
    }

    pub fn clear(&mut self) {
        self.item_count = 0;
    }
}

/// Todo app component
#[azumi::component]
pub fn todo_app_view<'a>(state: &'a TodoApp) -> impl Component + 'a {
    html! {
        <div class={todo_app}>
            <header class={app_header}>
                <h1 class={app_title}>"📝 Azumi Todos"</h1>
                <p class={app_subtitle}>"Built with Azumi Live"</p>
            </header>

            <div class={input_section}>
                <input class={todo_input} placeholder="What needs to be done?" />
                <button class={add_btn} on:click={state.add_item}>"Add"</button>
            </div>

            <div class={filter_section}>
                <div class={filter_group}>
                    <button
                        class={if !state.show_completed { format!("{} {}", filter_btn, filter_active) } else { filter_btn.to_string() }}
                        on:click={state.toggle_filter}>
                        "Active"
                    </button>
                    <button
                        class={if state.show_completed { format!("{} {}", filter_btn, filter_active) } else { filter_btn.to_string() }}
                        on:click={state.toggle_filter}>
                        "Completed"
                    </button>
                </div>
                <div class={status_group}>
                     <span class={item_count} data-bind="item_count">{state.item_count}</span>
                     <button class={clear_btn} on:click={state.clear}>"Clear"</button>
                </div>
            </div>

            <div class={todo_list}>
                @if state.item_count == 0 {
                    <div class={empty_state}>
                        "🎉 No todos! Add one above."
                    </div>
                }
                @if state.item_count > 0 {
                    <div class={list_content}>
                        <p class={list_summary}>"You have " <strong class={count_highlight}>{state.item_count}</strong> " item(s) in your list."</p>
                        <div class={dsl_box} style={ --bg_color: "rgba(59, 130, 246, 0.1)"; --padding: "1rem" }>
                             "This box is styled with the new Style DSL!"
                        </div>
                    </div>
                }
            </div>
        </div>
        <style>
            .todo_app {
                max-width: "500px";
                background: "rgba(30, 41, 59, 0.6)";
                backdrop-filter: "blur(12px)";
                border-radius: "16px";
                border: "1px solid rgba(255,255,255,0.1)";
                overflow: "hidden";
                box-shadow: "0 20px 25px -5px rgba(0, 0, 0, 0.2), 0 10px 10px -5px rgba(0, 0, 0, 0.1)";
            }
            .app_header {
                background: "linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%)";
                color: "white";
                padding: "2rem";
                text-align: "center";
            }
            .app_title {
                margin: "0";
                font-size: "2rem";
                font-weight: "800";
            }
            .app_subtitle {
                opacity: "0.8";
                margin-top: "0.5rem";
                font-size: "0.9rem";
            }
            .input_section {
                padding: "1.5rem";
                display: "flex";
                gap: "0.75rem";
                border-bottom: "1px solid rgba(255,255,255,0.05)";
            }
            .todo_input {
                flex: "1";
                padding: "0.75rem 1rem";
                background: "rgba(0, 0, 0, 0.2)";
                border: "1px solid rgba(255,255,255,0.1)";
                border-radius: "8px";
                font-size: "1rem";
                color: "white";
                transition: "border 0.2s";
            }
            .todo_input:focus {
                outline: "none";
                border-color: "#818cf8";
            }
            .add_btn {
                padding: "0.75rem 1.5rem";
                background: "#10b981";
                color: "white";
                border: "none";
                border-radius: "8px";
                cursor: "pointer";
                font-size: "1rem";
                font-weight: "600";
                transition: "opacity 0.2s";
            }
            .add_btn:hover { opacity: "0.9"; }

            .filter_section {
                padding: "1rem 1.5rem";
                display: "flex";
                justify-content: "space-between";
                align-items: "center";
                background: "rgba(0,0,0,0.1)";
                flex-wrap: "wrap";
                gap: "1rem";
            }
            .filter_group { display: "flex"; gap: "0.5rem"; }
            .status_group { display: "flex"; align-items: "center"; gap: "1rem"; }

            .filter_btn {
                padding: "0.4rem 0.8rem";
                border: "1px solid rgba(255,255,255,0.1)";
                border-radius: "6px";
                background: "transparent";
                color: "#94a3b8";
                cursor: "pointer";
                font-size: "0.9rem";
                transition: "all 0.2s";
            }
            .filter_btn:hover { background: "rgba(255,255,255,0.05)"; color: "white"; }
            .filter_active {
                background: "#6366f1";
                color: "white";
                border-color: "#6366f1";
            }
            .item_count {
                font-size: "1.2rem";
                font-weight: "bold";
                color: "#e2e8f0";
            }
            .clear_btn {
                padding: "0.4rem 0.8rem";
                background: "rgba(239, 68, 68, 0.2)";
                color: "#fca5a5";
                border: "1px solid rgba(239, 68, 68, 0.3)";
                border-radius: "6px";
                cursor: "pointer";
                font-size: "0.8rem";
                transition: "all 0.2s";
            }
            .clear_btn:hover { background: "rgba(239, 68, 68, 0.3)"; color: "#fecaca"; }

            .todo_list { padding: "1.5rem"; min-height: "150px"; }
            .empty_state { text-align: "center"; padding: "2rem"; color: "#64748b"; font-style: "italic"; }

            .list_content { display: "grid"; gap: "1rem"; text-align: "center"; }
            .list_summary { color: "#cbd5e1"; }
            .count_highlight { color: "#38bdf8"; }

            .dsl_box {
                background-color: "var(--bg_color)";
                padding: "var(--padding)";
                border-radius: "8px";
                color: "#93c5fd";
                border: "1px dashed rgba(59, 130, 246, 0.3)";
            }
        </style>
    }
}

/// Full page component for Lesson 15
#[azumi::page(route = "/lesson-15")]
#[azumi::component]
pub fn render_page() -> impl Component {
    let app_state = TodoApp {
        show_completed: false,
        item_count: 0,
    };
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 15: Full Application"</h1>
                    <p class={subtitle}>"Building a complete interactive todo app"</p>
                </header>

                <div class={explanation}>
                    <h3 class={exp_title}>"🚀 Putting It All Together"</h3>
                    <ul class={exp_list}>
                        <li class={exp_item}><strong class={strong}>"Multiple actions"</strong>" - add, toggle filter, clear"</li>
                        <li class={exp_item}><strong class={strong}>"Conditional rendering"</strong>" - empty state vs items"</li>
                        <li class={exp_item}><strong class={strong}>"Optimistic updates"</strong>" - instant count changes"</li>
                    </ul>
                </div>

                <div class={demo_area}>
                    @todo_app_view(state = &app_state)
                </div>
                @LessonNav(
                    prev_num=Some(14),
                    next_num=Some(16),
                    prev_title="Composing Live",
                    next_title="Async Database",
                )
            </div>
            <style>
                .container { max-width: "800px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #a855f7, #ec4899)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .subtitle { color: "#94a3b8"; font-size: "1.2rem"; }

                .explanation {
                    background: "rgba(168, 85, 247, 0.1)";
                    padding: "1.5rem";
                    border-radius: "12px";
                    margin: "2rem 0";
                    border: "1px solid rgba(168, 85, 247, 0.2)";
                }
                .exp_title { color: "#d8b4fe"; margin-bottom: "1rem"; font-size: "1.2rem"; }
                .exp_list { padding-left: "1.5rem"; display: "grid"; gap: "0.5rem"; }
                .exp_item { color: "#cbd5e1"; }
                .strong { color: "#f0abfc"; }

                .demo_area { display: "flex"; justify-content: "center"; margin: "2rem 0"; }
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson15_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&render_page()))
}
