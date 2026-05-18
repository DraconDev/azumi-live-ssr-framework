use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

/// Lesson 1: CSS Scoping & Validation Fundamentals
///
/// Automatic CSS scoping demonstration
#[azumi::component]
pub fn scoped_component() -> impl Component {
    html! {
        <div class={container}>
            <div class={icon}>"🔒"</div>
            <div>
                <h1 class={title}>"Automatically Scoped CSS"</h1>
                <p class={desc}>"This CSS is safely confined to this component."</p>
            </div>
        </div>
        <style>
            .container {
                padding: "2rem";
                border: "1px solid rgba(56, 189, 248, 0.2)";
                background: "linear-gradient(135deg, rgba(56, 189, 248, 0.1), rgba(56, 189, 248, 0.05))";
                border-radius: "12px";
                display: "flex";
                align-items: "center";
                gap: "1.5rem";
                box-shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1)";
            }
            .icon { font-size: "2rem"; }
            .title {
                color: "#38bdf8";
                margin-bottom: "0.25rem";
                font-size: "1.25rem";
                font-weight: "700";
            }
            .desc { color: "#bae6fd"; margin: "0"; }
        </style>
    }
}

/// Example: Multiple components with same class names
#[azumi::component]
pub fn multiple_scoped_components() -> impl Component {
    html! {
        <div class={grid}>
            <div class={card}>
                <div class={badge_purple}>"Component A"</div>
                <h3 class={card_title}>"First Component"</h3>
                <p class={card_text}>"Uses class 'card'"</p>
            </div>
            <div class={card_alt}>
                <div class={badge_blue}>"Component B"</div>
                <h3 class={card_title}>"Second Component"</h3>
                <p class={card_text}>"Also uses class 'card'"</p>
            </div>
        </div>
        <style>
            .grid { display: "grid"; grid-template-columns: "1fr 1fr"; gap: "1.5rem"; }
            .card {
                padding: "1.5rem";
                border: "1px solid rgba(167, 139, 250, 0.2)";
                background: "rgba(167, 139, 250, 0.05)";
                border-radius: "12px";
            }
            .card_alt {
                padding: "1.5rem";
                border: "1px solid rgba(56, 189, 248, 0.2)";
                background: "rgba(56, 189, 248, 0.05)";
                border-radius: "12px";
            }
            .card_title { font-weight: "bold"; color: "#f1f5f9"; margin: "0.5rem 0"; font-size: "1.1rem"; }
            .card_text { color: "#94a3b8"; font-size: "0.9rem"; }

            .badge_purple {
                display: "inline-block"; font-size: "0.75rem"; font-weight: "bold";
                color: "#a78bfa"; background: "rgba(167, 139, 250, 0.1)";
                padding: "0.2rem 0.6rem"; border-radius: "9999px";
            }
            .badge_blue {
                display: "inline-block"; font-size: "0.75rem"; font-weight: "bold";
                color: "#38bdf8"; background: "rgba(56, 189, 248, 0.1)";
                padding: "0.2rem 0.6rem"; border-radius: "9999px";
            }
        </style>
    }
}

/// Example: CSS validation - valid styles
#[azumi::component]
pub fn valid_css_example() -> impl Component {
    html! {
        <div class={valid_container}>
            <div class={header}>
                <h2 class={valid_title}>"Valid CSS Rules"</h2>
                <div class={check_icon}>"✓"</div>
            </div>
            <p class={valid_text}>"This component passes compile-time validation."</p>
            <ul class={valid_list}>
                <li class={item}><span class={bullet}>"•"</span>"Proper property values"</li>
                <li class={item}><span class={bullet}>"•"</span>"Valid color formats"</li>
                <li class={item}><span class={bullet}>"•"</span>"Correct unit usage"</li>
            </ul>
        </div>
        <style>
            .valid_container {
                padding: "2rem";
                background: "linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(6, 95, 70, 0.1))";
                border: "1px solid rgba(16, 185, 129, 0.2)";
                border-radius: "12px";
            }
            .header { display: "flex"; justify-content: "space-between"; align-items: "center"; margin-bottom: "1rem"; }
            .check_icon {
                background: "rgba(16, 185, 129, 0.2)"; color: "#34d399";
                width: "32px"; height: "32px"; border-radius: "50%";
                display: "flex"; align-items: "center"; justify-content: "center"; font-weight: "bold";
            }
            .valid_title { color: "#34d399"; font-size: "1.25rem"; margin: "0"; font-weight: "700"; }
            .valid_text { color: "#d1fae5"; margin-bottom: "1.5rem"; }
            .valid_list { list-style: "none"; padding: "0"; margin: "0"; display: "grid"; gap: "0.5rem"; }
            .item { color: "#e2e8f0"; display: "flex"; align-items: "center"; gap: "0.5rem"; }
            .bullet { color: "#34d399"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::page(route = "/lesson-1")]
#[azumi::component]
pub fn page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 1: Scoping & Validation"</h1>
                    <p class={subtitle}>"Automatic CSS scoping and validation rules"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ CSS is automatically scoped to each component"</li>
                        <li class={point}>"✅ No manual CSS management needed"</li>
                        <li class={point}>"✅ Prevents CSS conflicts between components"</li>
                        <li class={point}>"✅ Azumi validates CSS syntax at compile time"</li>
                        <li class={point}>"✅ Only valid CSS properties and values allowed"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                         <h3 class={card_label}>"Example 1: Scoped Styles"</h3>
                        @scoped_component()
                    </div>
                    <div class={example_card}>
                         <h3 class={card_label}>"Example 2: Conflict Prevention"</h3>
                        @multiple_scoped_components()
                    </div>
                    <div class={example_card}>
                         <h3 class={card_label}>"Example 3: Validation"</h3>
                        @valid_css_example()
                    </div>
                </section>
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #34d399, #38bdf8)";
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
pub async fn lesson1_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
