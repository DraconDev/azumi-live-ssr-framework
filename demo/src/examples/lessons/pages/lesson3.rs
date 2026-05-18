use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use azumi::prelude::*;

/// Lesson 3: Component Composition
///
/// Building complex UIs from simple components
#[azumi::component]
pub fn card<'a>(title: &'a str, content: &'a str) -> impl Component + 'a {
    html! {
        <div class={card}>
            <h3 class={card_title}>{title}</h3>
            <p class={card_content}>{content}</p>
        </div>
        <style>
            .card {
                border: "1px solid rgba(255,255,255,0.08)";
                padding: "1.5rem";
                margin: "0.5rem";
                background: "rgba(30, 41, 59, 0.4)";
                backdrop-filter: "blur(12px)";
                border-radius: "12px";
                box-shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)";
                transition: "transform 0.2s ease";
            }
            .card:hover { transform: "translateY(-2px)"; }
            .card_title {
                font-weight: "700";
                margin-bottom: "0.75rem";
                color: "#e2e8f0";
                font-size: "1.1rem";
                letter-spacing: "-0.01em";
            }
            .card_content { color: "#94a3b8"; line-height: "1.6"; }
        </style>
    }
}

/// Example: Dashboard composed of multiple cards
#[azumi::component]
pub fn dashboard() -> impl Component {
    html! {
        <div>
            <div class={dash_header}>
                <h2 class={dashboard_title}>"Component Composition Dashboard"</h2>
                <span class={status_badge}>"Live"</span>
            </div>
            <div class={dashboard_container}>
                @card(title="Welcome", content="Welcome to Azumi Component Composition")
                @card(title="Features", content="Type-safe components that compose beautifully")
                @card(title="Performance", content="Compile-time optimized rendering")
            </div>
        </div>
        <style>
            .dashboard_container { display: "grid"; gap: "1rem"; grid-template-columns: "repeat(auto-fit, minmax(250px, 1fr))"; }
            .dash_header { display: "flex"; align-items: "center"; justify-content: "space-between"; margin-bottom: "1.5rem"; }
            .dashboard_title {
                font-size: "1.5rem";
                color: "#818cf8";
                font-weight: "700";
                margin: "0";
            }
            .status_badge {
                background: "rgba(16, 185, 129, 0.2)";
                color: "#34d399";
                font-size: "0.75rem";
                font-weight: "bold";
                padding: "0.25rem 0.75rem";
                border-radius: "9999px";
                text-transform: "uppercase";
            }
        </style>
    }
}

/// Example: Complex layout with nested composition
#[azumi::component]
pub fn complex_layout() -> impl Component {
    html! {
        <div class={layout_container}>
            <div class={header_section}>
                <h2 class={header_title}>"Complex Layout Composition"</h2>
                <p class={header_desc}>"Multiple components working together"</p>
            </div>
            <div class={main_section}>
                <div class={content_area}>
                    @card(title="Main Content", content="This is the primary content area")
                    @card(title="Additional Info", content="More information here")
                </div>
                <div class={sidebar}>
                    <h3 class={sidebar_title}>"Sidebar"</h3>
                    <div class={link_list}>
                        @card(title="Quick Links", content="Navigation and tools")
                        @card(title="Resources", content="Docs and API")
                    </div>
                </div>
            </div>
        </div>
        <style>
            .layout_container { display: "grid"; gap: "1.5rem"; }
            .header_section {
                background: "linear-gradient(135deg, rgba(99, 102, 241, 0.1), rgba(79, 70, 229, 0.1))";
                padding: "2rem";
                border-radius: "12px";
                border: "1px solid rgba(99, 102, 241, 0.2)";
                text-align: "center";
            }
            .header_title { color: "#818cf8"; margin-bottom: "0.5rem"; font-size: "1.75rem"; font-weight: "700"; }
            .header_desc { color: "#c7d2fe"; font-size: "1.1rem"; }

            .main_section {
                display: "grid";
                grid-template-columns: "2fr 1fr";
                gap: "1.5rem";
            }
            .content_area { display: "grid"; gap: "1rem"; }
            .sidebar {
                background: "rgba(15, 23, 42, 0.3)";
                padding: "1.5rem";
                border-radius: "12px";
                border: "1px solid rgba(255,255,255,0.05)";
                height: "fit-content";
            }
            .sidebar_title {
                color: "#94a3b8";
                margin-bottom: "1rem";
                font-size: "0.875rem";
                text-transform: "uppercase";
                letter-spacing: "0.05em";
                font-weight: "600";
            }
            .link_list { display: "grid"; gap: "0.5rem"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::page(route = "/lesson-3")]
#[azumi::component]
pub fn page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 3: Component Composition"</h1>
                    <p class={subtitle}>"Building complex UIs from simple components"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ Create simple, focused components"</li>
                        <li class={point}>"✅ Compose them together to build complex UIs"</li>
                        <li class={point}>"✅ Reuse components across your application"</li>
                        <li class={point}>"✅ Pass props to customize component behavior"</li>
                        <li class={point}>"✅ Maintain clean separation of concerns"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        <div class={card_label}>"Example 1: Dashboard"</div>
                        @dashboard()
                    </div>
                    <div class={example_card}>
                        <div class={card_label}>"Example 2: Complex Layout"</div>
                        @complex_layout()
                    </div>
                </section>
                @LessonNav(
                    prev_num=Some(2),
                    next_num=Some(4),
                    prev_title="Global vs Component CSS",
                    next_title="Children & Layouts",
                )
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #818cf8, #60a5fa)";
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
pub async fn lesson3_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
