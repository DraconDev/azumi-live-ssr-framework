use azumi::prelude::*;

use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;

/// Lesson 6: Control Flow Patterns
///
/// @if, @else, @for, @match patterns
#[azumi::component]
pub fn control_flow_example() -> impl Component {
    html! {

        <div class={content}>
            <h2 class={title}>"Control Flow Demo"</h2>

            @if true {
                <h3 class={subtitle}>"Detailed View"</h3>
                <div class={item}>"Item 1"</div>
                <div class={item}>"Item 2"</div>
                <div class={item}>"Item 3"</div>
            }

            @if false {
                <h3>"Summary View"</h3>
                <p>"Total items: 3"</p>
            }

            @match "active" {
                "active" => {
                    <p class={status_active}>"Status: Active"</p>
                }
                "inactive" => {
                    <p class={status_inactive}>"Status: Inactive"</p>
                }
                _ => {
                    <p>"Status: Unknown"</p>
                }
            }
        </div>
        <style>
            .content { padding: "1.5rem"; }
            .title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .subtitle { color: "#cbd5e1"; margin-bottom: "0.5rem"; font-size: "1rem"; }
            .item {
                margin: "0.5rem 0";
                padding: "0.75rem";
                background: "rgba(15, 23, 42, 0.4)";
                border-radius: "8px";
                border: "1px solid rgba(255,255,255,0.05)";
                color: "#cbd5e1";
            }
            .status_active { color: "#4ade80"; font-weight: "bold"; margin-top: "1rem"; }
            .status_inactive { color: "#f87171"; font-weight: "bold"; margin-top: "1rem"; }
        </style>
    }
}

/// Example: Complex conditional logic
#[azumi::component]
pub fn complex_conditions() -> impl Component {
    html! {

        <div class={conditions_container}>
            <h3 class={title}>"Complex Conditions"</h3>

            @let user_role = "admin";
            @if user_role == "admin" {
                <p class={permission_granted}>"Full access granted"</p>
                <ul class={list}>
                    <li>"Item 1"</li>
                    <li>"Item 2"</li>
                    <li>"Item 3"</li>
                </ul>
            }

            @if "user" != "admin" {
                <p class={permission_denied}>"Limited access only"</p>
            }
        </div>
        <style>
            .conditions_container {
                padding: "1.5rem";
                background: "rgba(20, 184, 166, 0.1)";
                border-radius: "12px";
                border: "1px solid rgba(20, 184, 166, 0.2)";
                color: "#e2e8f0";
            }
            .title { color: "#2dd4bf"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .permission_granted { color: "#5eead4"; font-weight: "bold"; margin-bottom: "0.5rem"; }
            .permission_denied { color: "#f87171"; font-weight: "bold"; }
            .list { margin-left: "1.5rem"; color: "#cbd5e1"; }
        </style>
    }
}

/// Example: Pattern matching with enums
#[azumi::component]
pub fn pattern_matching_example() -> impl Component {
    html! {

        <div class={pattern_container}>
            <h3 class={title}>"Pattern Matching"</h3>

            @match "loading" {
                "loading" => {
                    <p class={state_loading}>"Loading data..."</p>
                }
                "success" => {
                    <p class={state_success}>"Data loaded successfully!"</p>
                }
                "error" => {
                    <p class={state_error}>"Error loading data"</p>
                }
                _ => {
                    <p>"Unknown state"</p>
                }
            }
        </div>
        <style>
            .pattern_container { padding: "1.5rem"; }
            .title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .state_loading { color: "#60a5fa"; font-weight: "bold"; }
            .state_success { color: "#4ade80"; font-weight: "bold"; }
            .state_error { color: "#f87171"; font-weight: "bold"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::page(route = "/lesson-6")]
#[azumi::component]
pub fn page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 6: Control Flow Patterns"</h1>
                    <p class={subtitle}>"@if, @else, @for, @match patterns"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ @if for conditional rendering"</li>
                        <li class={point}>"✅ @for for iteration over collections"</li>
                        <li class={point}>"✅ @match for pattern matching"</li>
                        <li class={point}>"✅ All control flow works at compile time"</li>
                        <li class={point}>"✅ Type-safe expressions and patterns"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        @control_flow_example()
                    </div>
                    <div class={example_card}>
                        @complex_conditions()
                    </div>
                    <div class={example_card}>
                        @pattern_matching_example()
                    </div>
                </section>
                @LessonNav(
                    prev_num=Some(5),
                    next_num=Some(7),
                    prev_title="@let Pattern",
                    next_title="Form Handling",
                )
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #2dd4bf, #818cf8)";
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
pub async fn lesson6_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
