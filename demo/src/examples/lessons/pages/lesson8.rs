use azumi::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct CounterState {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ManagementState {
    pub status: String,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CompositionState {
    pub message: String,
    pub step: i32,
}

use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;

/// Lesson 8: Action System Deep Dive
///
/// Server-side interactivity patterns
pub fn counter_display(state: CounterState) -> impl Component {
    html! {
        <div id={counter_box} class={counter} az-scope={serde_json::to_string(&state).unwrap_or_default()}>
            <div class={count_display}>{state.count}</div>
            <button class={counter_button} az-on={click call increment_counter -> #counter_box}>
                "Increment"
            </button>
            <div class={timestamp}>"Last updated: 12:00:00"</div>
        </div>
        <style>
            .counter {
                padding: "2rem";
                text-align: "center";
                border: "1px solid rgba(255,255,255,0.05)";
                background: "rgba(15, 23, 42, 0.4)";
                border-radius: "12px";
                color: "white";
            }
            .count_display { font-size: "3rem"; margin: "1rem 0"; color: "#38bdf8"; font-weight: "bold"; }
            .counter_button {
                padding: "1rem 2rem";
                background: "linear-gradient(to right, #3b82f6, #2563eb)";
                color: "white";
                border: "none";
                cursor: "pointer";
                border-radius: "8px";
                font-weight: "600";
                transition: "opacity 0.2s";
            }
            .timestamp { font-size: "0.8rem"; color: "#94a3b8"; margin-top: "1rem"; }
            #counter_box { display: "block"; }
        </style>
    }
}

#[azumi::action]
pub async fn increment_counter(state: CounterState) -> impl Component {
    let new_state = CounterState {
        count: state.count + 1,
    };
    counter_display(new_state)
}

/// Example: Action with state management
pub fn state_management_example(state: ManagementState) -> impl Component {
    html! {

        <div id={state_box} class={state_container} az-scope={serde_json::to_string(&state).unwrap_or_default()}>
            <h3 class={title}>"State Management"</h3>

            <div class={state_info}>
                <p>"Current State: " <span class={highlight}>{state.status}</span></p>
                <p>"Counter: " <span class={highlight}>{state.count}</span></p>
            </div>

            <button class={action_button} az-on={click call update_state -> #state_box}>
                "Update State"
            </button>
        </div>
        <style>
            .state_container {
                padding: "1.5rem";
                background: "rgba(20, 184, 166, 0.1)";
                border-radius: "12px";
                border: "1px solid rgba(20, 184, 166, 0.2)";
                color: "#e2e8f0";
            }
            .title { color: "#2dd4bf"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .state_info {
                margin: "1rem 0";
                padding: "1rem";
                background: "rgba(0,0,0,0.2)";
                border-radius: "8px";
                color: "#cbd5e1";
            }
            .highlight { color: "#5eead4"; font-weight: "bold"; }
            .action_button {
                padding: "0.75rem 1.5rem";
                background: "linear-gradient(to right, #0d9488, #0f766e)";
                color: "white";
                border: "none";
                cursor: "pointer";
                border-radius: "8px";
                font-weight: "600";
            }
            #state_box { display: "block"; }
        </style>
    }
}

#[azumi::action]
pub async fn update_state(state: ManagementState) -> impl Component {
    let new_count = state.count + 1;
    let new_status = if new_count % 2 == 0 {
        "Active"
    } else {
        "Processing"
    };
    let new_state = ManagementState {
        status: new_status.to_string(),
        count: new_count,
    };
    state_management_example(new_state)
}

/// Example: Action composition
pub fn action_composition_example(state: CompositionState) -> impl Component {
    html! {

        <div id={composition_box} class={composition_container} az-scope={serde_json::to_string(&state).unwrap_or_default()}>
            <h3 class={title}>"Action Composition"</h3>

            <div class={action_card}>
                <p>"Message: " <span class={msg_text}>{state.message}</span></p>
                <p>"Step: " <span class={step_text}>{state.step}</span></p>
            </div>

            <button class={compose_button} az-on={click call compose_actions -> #composition_box}>
                "Compose Actions"
            </button>
        </div>
        <style>
            .composition_container { padding: "1.5rem"; color: "#e2e8f0"; }
            .title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .action_card {
                margin: "1rem 0";
                padding: "1rem";
                background: "rgba(30, 41, 59, 0.6)";
                border-radius: "8px";
                border: "1px solid rgba(255,255,255,0.05)";
                color: "#cbd5e1";
            }
            .msg_text { color: "#f472b6"; }
            .step_text { color: "#fb7185"; font-weight: "bold"; }
            .compose_button {
                padding: "0.75rem 1.5rem";
                background: "linear-gradient(to right, #db2777, #be185d)";
                color: "white";
                border: "none";
                cursor: "pointer";
                border-radius: "8px";
                font-weight: "600";
            }
            #composition_box { display: "block"; }
        </style>
    }
}

#[azumi::action]
pub async fn compose_actions(state: CompositionState) -> impl Component {
    let new_step = state.step + 1;
    let new_message = format!("Action composed at step {}", new_step);
    let new_state = CompositionState {
        message: new_message,
        step: new_step,
    };
    action_composition_example(new_state)
}

/// Main lesson demonstration component
#[azumi::page(route = "/lesson-8")]
#[azumi::component]
pub fn page() -> impl Component {
    let counter_state = CounterState { count: 0 };
    let management_state = ManagementState {
        status: "Active".to_string(),
        count: 0,
    };
    let composition_state = CompositionState {
        message: "Initial State".to_string(),
        step: 0,
    };

    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 8: Action System Deep Dive"</h1>
                    <p class={subtitle}>"Server-side interactivity patterns"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ Server-side actions for interactivity"</li>
                        <li class={point}>"✅ State management patterns"</li>
                        <li class={point}>"✅ Action composition"</li>
                        <li class={point}>"✅ Type-safe action parameters"</li>
                        <li class={point}>"✅ Compile-time action validation"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        { counter_display(counter_state) }
                    </div>
                    <div class={example_card}>
                        { state_management_example(management_state.clone()) }
                    </div>
                    <div class={example_card}>
                        { action_composition_example(composition_state.clone()) }
                    </div>
                </section>
                @LessonNav(
                    prev_num=Some(7),
                    next_num=Some(9),
                    prev_title="Form Handling",
                    next_title="Azumi Live",
                )
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #f472b6, #fb7185)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .subtitle { font-size: "1.25rem"; color: "#94a3b8"; }

                .key_points {
                    background: "rgba(30, 41, 59, 0.5)";
                    padding: "2rem";
                    border-radius: "16px";
                    margin-bottom: "3rem";
                    border: "1px solid rgba(255,255,255,0.05)";
                    backdrop-filter: "blur(10px)";
                }
                .section_title {
                    font-size: "1.5rem";
                    color: "#f1f5f9";
                    margin-bottom: "1.5rem";
                    border-bottom: "1px solid rgba(255,255,255,0.1)";
                    padding-bottom: "0.5rem";
                }
                .points_list { list-style: "none"; padding: "0"; display: "grid"; gap: "1rem"; }
                .point {
                    color: "#e2e8f0";
                    padding: "0.75rem";
                    background: "rgba(255,255,255,0.03)";
                    border-radius: "8px";
                    font-size: "1.1rem";
                }

                .examples { display: "grid"; gap: "2rem"; }
                .example_card {
                    border: "1px solid rgba(255,255,255,0.1)";
                    padding: "2rem";
                    border-radius: "16px";
                    background: "rgba(15, 23, 42, 0.6)";
                }
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson8_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
