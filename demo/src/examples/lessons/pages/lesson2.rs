use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use azumi::prelude::*;

/// Lesson 2: Global vs Component CSS
///
/// Understanding style scoping options
#[azumi::component]
pub fn global_css_example() -> impl Component {
    html! {
        <div class={wrapper}>
            <div class={component_card}>
                <div class={badge}>"Scoped"</div>
                <h2 class={local_class}>"Component Level"</h2>
                <p class={text_content}>"Styles defined here stay here."</p>
            </div>
            <div class={global_card}>
                <div class={badge_global}>"Global"</div>
                <h2 class={global_demo}>"Global Level"</h2>
                <p class={text_content}>"Styles apply everywhere."</p>
            </div>
        </div>
        // Global styles - not scoped to component
        <style global>
            body { font-family: "'Inter', system-ui, sans-serif"; }
        </style>

        // Component-scoped styles - automatically scoped
        <style>
            .wrapper { display: "grid"; grid-template-columns: "1fr 1fr"; gap: "1.5rem"; }
            .component_card {
                background: "rgba(56, 189, 248, 0.1)";
                padding: "1.5rem";
                border-radius: "12px";
                border: "1px solid rgba(56, 189, 248, 0.2)";
            }
            .global_card {
                background: "rgba(167, 139, 250, 0.1)";
                padding: "1.5rem";
                border-radius: "12px";
                border: "1px solid rgba(167, 139, 250, 0.2)";
            }
            .local_class { color: "#38bdf8"; font-weight: "700"; margin: "0.5rem 0"; font-size: "1.25rem"; }
            .global_demo { color: "#a78bfa"; font-weight: "700"; margin: "0.5rem 0"; font-size: "1.25rem"; }
            .text_content { color: "#cbd5e1"; font-size: "0.95rem"; }

            .badge {
                display: "inline-block"; font-size: "0.75rem"; font-weight: "bold"; text-transform: "uppercase";
                color: "#38bdf8"; background: "rgba(56, 189, 248, 0.2)"; padding: "0.2rem 0.6rem"; border-radius: "4px";
            }
            .badge_global {
                display: "inline-block"; font-size: "0.75rem"; font-weight: "bold"; text-transform: "uppercase";
                color: "#a78bfa"; background: "rgba(167, 139, 250, 0.2)"; padding: "0.2rem 0.6rem"; border-radius: "4px";
            }
        </style>
    }
}

/// Example: Multiple components with different scoping
#[azumi::component]
pub fn mixed_scoping_example() -> impl Component {
    html! {
        <div class={container}>
            <div class={header}>
                <h3 class={scoped_title}>"Visualizing Scope"</h3>
            </div>
            <div class={visual_list}>
                <div class={item}>
                    <div class={dot_global}></div>
                    <div>
                        <span class={label_global}>"Global Scope"</span>
                        <p class={desc}>"Affects 100% of pages"</p>
                    </div>
                </div>
                <div class={item}>
                    <div class={dot_scoped}></div>
                    <div>
                        <span class={label_scoped}>"Component Scope"</span>
                        <p class={desc}>"Affects only this instance"</p>
                    </div>
                </div>
            </div>
        </div>
        <style global>
            /* This would affect the entire app */
            /* body { font-family: "Arial, sans-serif"; } */
        </style>

        <style>
            .container {
                padding: "2rem";
                border: "1px solid rgba(255,255,255,0.05)";
                background: "rgba(30, 41, 59, 0.4)";
                border-radius: "16px";
            }
            .header { border-bottom: "1px solid rgba(255,255,255,0.05)"; padding-bottom: "1rem"; margin-bottom: "1.5rem"; }
            .scoped_title { color: "#e2e8f0"; font-size: "1.25rem"; font-weight: "700"; margin: "0"; }

            .visual_list { display: "grid"; gap: "1.5rem"; }
            .item { display: "flex"; gap: "1rem"; align-items: "flex-start"; }

            .dot_global { width: "12px"; height: "12px"; border-radius: "50%"; background: "#f472b6"; margin-top: "0.4rem"; box-shadow: "0 0 10px rgba(244, 114, 182, 0.5)"; }
            .dot_scoped { width: "12px"; height: "12px"; border-radius: "50%"; background: "#34d399"; margin-top: "0.4rem"; box-shadow: "0 0 10px rgba(52, 211, 153, 0.5)"; }

            .label_global { color: "#f472b6"; font-weight: "bold"; font-size: "1.1rem"; }
            .label_scoped { color: "#34d399"; font-weight: "bold"; font-size: "1.1rem"; }
            .desc { color: "#94a3b8"; margin: "0.25rem 0 0 0"; }
        </style>
    }
}

/// Example: CSS scoping best practices
#[azumi::component]
pub fn scoping_best_practices() -> impl Component {
    html! {
        <div class={best_practices}>
            <h3 class={bp_title}>"CSS Scoping Best Practices"</h3>

            <div class={practice_item}>
                <span class={do_class}>"DO:"</span> " Use component-scoped styles for most cases"
            </div>

            <div class={practice_item}>
                <span class={do_class}>"DO:"</span> " Use global styles only for truly global elements"
            </div>

            <div class={practice_item}>
                <span class={dont_class}>"DON'T:"</span> " Overuse global styles - they can cause conflicts"
            </div>

            <div class={practice_item}>
                <span class={do_class}>"DO:"</span> " Let Azumi handle scoping automatically"
            </div>
        </div>
        <style>
            .best_practices {
                padding: "1.5rem";
                background: "rgba(30, 41, 59, 0.3)";
                border-radius: "12px";
                border: "1px solid rgba(255,255,255,0.05)";
            }
            .bp_title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .practice_item {
                margin: "0.75rem 0";
                padding: "0.75rem";
                background: "rgba(15, 23, 42, 0.6)";
                border-radius: "8px";
                color: "#cbd5e1";
                border: "1px solid rgba(255,255,255,0.05)";
            }
            .do_class { color: "#4ade80"; font-weight: "bold"; margin-right: "0.5rem"; }
            .dont_class { color: "#f87171"; font-weight: "bold"; margin-right: "0.5rem"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::page(route = "/lesson-2")]
#[azumi::component]
pub fn page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 2: Global vs Component CSS"</h1>
                    <p class={subtitle}>"Understanding style scoping options"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ Global styles use <style global> tag"</li>
                        <li class={point}>"✅ Component styles use <style> tag (automatically scoped)"</li>
                        <li class={point}>"✅ Global styles affect the entire application"</li>
                        <li class={point}>"✅ Component styles are scoped to prevent conflicts"</li>
                        <li class={point}>"✅ Azumi handles scoping automatically for component styles"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        <div class={card_label}>"Example 1: Scoped Styles"</div>
                        @global_css_example()
                    </div>
                    <div class={example_card}>
                        <div class={card_label}>"Example 2: Mixed Scoping"</div>
                        @mixed_scoping_example()
                    </div>
                    <div class={example_card}>
                        <div class={card_label}>"Example 3: Best Practices"</div>
                        @scoping_best_practices()
                    </div>
                </section>
                @LessonNav(
                    prev_num=Some(1),
                    next_num=Some(3),
                    prev_title="CSS Scoping",
                    next_title="Composition",
                )
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #a78bfa, #818cf8)";
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
pub async fn lesson2_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
