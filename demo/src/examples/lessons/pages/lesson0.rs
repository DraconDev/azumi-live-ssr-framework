use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use azumi::prelude::*;

/// Lesson 0: Introduction to Azumi Components
///
/// Basic component structure with proper syntax
#[azumi::component]
pub fn hello_world() -> impl Component {
    html! {
        <div class={greeting}>"Hello, Azumi!"</div>
        <style>
            .greeting {
                color: "#60a5fa";
                font-size: "1.5rem";
                font-weight: "bold";
                text-align: "center";
                padding: "2rem";
                background: "rgba(30, 41, 59, 0.4)";
                border-radius: "8px";
                border: "1px solid rgba(96, 165, 250, 0.2)";
            }
        </style>
    }
}

/// Example: Component with styling and structure
#[azumi::component]
pub fn basic_component() -> impl Component {
    html! {
        <div class={container}>
            <h1 class={title}>"Basic Azumi Component"</h1>
            <p class={desc}>"This demonstrates the basic component structure"</p>
            <p class={highlight}>"All CSS is automatically scoped to this component"</p>
        </div>
        <style>
            .container {
                padding: "1.5rem";
                border: "1px solid rgba(255,255,255,0.1)";
                background: "rgba(15, 23, 42, 0.5)";
                border-radius: "12px";
            }
            .title { color: "#38bdf8"; margin-bottom: "0.5rem"; font-size: "1.25rem"; }
            .desc { color: "#94a3b8"; }
            .highlight { color: "#a5f3fc"; font-style: "italic"; }
        </style>
    }
}

/// Example: Component with multiple elements
#[azumi::component]
pub fn multi_element_component() -> impl Component {
    html! {
        <div class={card}>
            <h2 class={card_title}>"Multi-Element Component"</h2>
            <p class={card_content}>"Components can contain multiple elements with proper styling"</p>
            <p class={card_content}>"All CSS is automatically scoped to this component"</p>
        </div>
        <style>
            .card {
                padding: "1.5rem";
                margin: "1rem 0";
                border: "1px solid rgba(255,255,255,0.1)";
                border-radius: "12px";
                background: "rgba(30, 41, 59, 0.3)";
            }
            .card_title { font-size: "1.2rem"; color: "#818cf8"; margin-bottom: "1rem"; }
            .card_content { color: "#cbd5e1"; margin-bottom: "0.5rem"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::page(route = "/lesson-0")]
#[azumi::component]
pub fn page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 0: Quick Intro"</h1>
                    <p class={subtitle}>"Understanding the component structure."</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Unique Features"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ Components use #[azumi::component] macro"</li>
                        <li class={point}>"✅ CSS is automatically scoped to each component"</li>
                        <li class={point}>"✅ Components return impl Component"</li>
                        <li class={point}>"✅ HTML structure uses html! macro"</li>
                        <li class={point}>"✅ All text content must be quoted"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        <div class={card_label}>"Example 1: Hello World"</div>
                        @hello_world()
                    </div>
                    <div class={example_card}>
                         <div class={card_label}>"Example 2: Basic Component"</div>
                        @basic_component()
                    </div>
                    <div class={example_card}>
                         <div class={card_label}>"Example 3: Multi-Element"</div>
                        @multi_element_component()
                    </div>
                </section>

                @LessonNav(
                    prev_num=None,
                    next_num=Some(1),
                    prev_title="Home",
                    next_title="CSS Scoping",
                )
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #60a5fa, #a78bfa)";
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
                .card_label {
                    font-size: "0.875rem";
                    color: "#64748b";
                    text-transform: "uppercase";
                    letter-spacing: "0.05em";
                    margin-bottom: "1.5rem";
                    font-weight: "600";
                }
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson0_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
