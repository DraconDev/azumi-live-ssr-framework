use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use azumi::prelude::*;

/// Lesson 5: @let Pattern for Local Variables
///
/// Using @let for local variable declarations
#[azumi::component]
pub fn let_pattern_example() -> impl Component {
    html! {
        <div class={let_demo}>
            <h2 class={title}>"@let Pattern Examples"</h2>

            // Basic variable declaration
            @let name = "Azumi";
            <p>"Hello, " <span class={highlight}>{name}</span> "!"</p>

            // Calculated values
            @let items = ["Item 1", "Item 2", "Item 3"];
            @let item_count = items.len();
            <p>"items.len() = " <span class={highlight}>{item_count}</span></p>

            // Derived values from calculations
            @let base_price = 100.0;
            @let tax_rate = 0.08;
            @let _total_price = base_price * (1.0 + tax_rate);

            <div class={receipt}>
                <div class={receipt_header}>"Receipt"</div>
                <div class={line_item}><span>"Base Price"</span> <span>"$" {base_price} ".00"</span></div>
                <div class={line_item}><span>"Tax Rate"</span> <span>{tax_rate * 100.0} "%"</span></div>
                <div class={line_total}><span>"Total"</span> <span>"$" {format!("{:.2}", base_price * (1.0 + tax_rate))}</span></div>
            </div>

            // Complex data transformations
            @let users = [
                ("Alice", 25),
                ("Bob", 30),
                ("Charlie", 35)
            ];
            @let user_names = users.iter().map(|(name, _)| *name).collect::<Vec<&str>>();
            <div class={user_list}>
                <h3 class={list_title}>"Active Users:"</h3>
                <div class={badges}>
                    @for name in user_names {
                        <span class={user_badge}>{name}</span>
                    }
                </div>
            </div>
        </div>
        <style>
            .let_demo { padding: "1.5rem"; font-family: "'Inter', sans-serif"; }
            .title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; font-weight: "700"; }
            .highlight { font-weight: "bold"; color: "#38bdf8"; background: "rgba(56, 189, 248, 0.1)"; padding: "0.1rem 0.3rem"; border-radius: "4px"; }

            .receipt {
                background: "#f8fafc";
                padding: "1.5rem";
                border-radius: "8px";
                margin-top: "1.5rem";
                color: "#334155";
                font-family: "monospace";
                box-shadow: "0 4px 6px -1px rgba(0,0,0,0.1)";
                max-width: "300px";
                transform: "rotate(-1deg)";
            }
            .receipt_header { text-align: "center"; font-weight: "bold"; text-transform: "uppercase"; border-bottom: "1px dashed #cbd5e1"; padding-bottom: "0.5rem"; margin-bottom: "0.5rem"; }
            .line_item { display: "flex"; justify-content: "space-between"; margin-bottom: "0.25rem"; font-size: "0.9rem"; }
            .line_total { display: "flex"; justify-content: "space-between"; margin-top: "0.5rem"; border-top: "1px dashed #cbd5e1"; padding-top: "0.5rem"; font-weight: "bold"; font-size: "1.1rem"; }

            .user_list { margin-top: "2rem"; }
            .list_title { font-size: "1rem"; color: "#94a3b8"; margin-bottom: "0.5rem"; text-transform: "uppercase"; letter-spacing: "0.05em"; }
            .badges { display: "flex"; gap: "0.5rem"; flex-wrap: "wrap"; }
            .user_badge {
                background: "rgba(167, 139, 250, 0.2)";
                color: "#c084fc";
                padding: "0.25rem 0.75rem";
                border-radius: "9999px";
                font-size: "0.875rem";
                font-weight: "600";
            }
        </style>
    }
}

/// Example: @let with conditional logic
#[azumi::component]
pub fn let_with_conditions() -> impl Component {
    html! {
        <div class={conditions_demo}>
            <h3 class={title}>"@let with Conditions"</h3>

            @let score = 85;
            @let grade = if score >= 90 {
                "A"
            } else if score >= 80 {
                "B"
            } else if score >= 70 {
                "C"
            } else {
                "F"
            };

            <div class={grade_card}>
                <div class={score_circle}>
                    <span class={score_val}>{score}</span>
                    <span class={score_label}>"SCORE"</span>
                </div>
                <div class={grade_val}>{grade}</div>
            </div>
        </div>
        <style>
            .conditions_demo {
                padding: "1.5rem";
                background: "rgba(20, 184, 166, 0.1)";
                border-radius: "12px";
                border: "1px solid rgba(20, 184, 166, 0.2)";
                color: "#e2e8f0";
            }
            .title { color: "#2dd4bf"; margin-bottom: "1.5rem"; font-size: "1.25rem"; font-weight: "700"; }

            .grade_card { display: "flex"; align-items: "center"; gap: "2rem"; }
            .score_circle {
                width: "80px"; height: "80px"; border-radius: "50%";
                border: "4px solid #2dd4bf";
                display: "flex"; flex-direction: "column"; align-items: "center"; justify-content: "center";
            }
            .score_val { font-size: "1.5rem"; font-weight: "800"; color: "#2dd4bf"; line-height: "1"; }
            .score_label { font-size: "0.6rem"; color: "#5eead4"; font-weight: "bold"; }

            .grade_val { font-size: "4rem"; font-weight: "900"; color: "#f0fdfa"; text-shadow: "0 0 20px rgba(45, 212, 191, 0.5)"; }
        </style>
    }
}

/// Example: @let for component composition
#[azumi::component]
pub fn let_composition_example() -> impl Component {
    html! {
        <div class={composition_demo}>
            <h3 class={section_title}>"@let for Composition"</h3>

            @let card_title = "Dynamic Component";
            @let card_content = "This component uses @let variables";

            <div class={component_container}>
                <div class={icon}>"✨"</div>
                <div>
                    <h4 class={comp_title}>{card_title}</h4>
                    <p class={comp_text}>{card_content}</p>
                </div>
            </div>
        </div>
        <style>
            .composition_demo { padding: "1.5rem"; }
            .section_title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .component_container {
                margin: "1rem 0";
                padding: "1.5rem";
                background: "linear-gradient(to right, rgba(30, 41, 59, 0.8), rgba(30, 41, 59, 0.4))";
                border-radius: "12px";
                border: "1px solid rgba(255,255,255,0.05)";
                display: "flex";
                gap: "1rem";
                align-items: "flex-start";
            }
            .icon { font-size: "2rem"; }
            .comp_title { color: "#a5f3fc"; margin-bottom: "0.25rem"; font-size: "1.1rem"; font-weight: "bold"; }
            .comp_text { color: "#cbd5e1"; margin: "0"; }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::page(route = "/lesson-5")]
#[azumi::component]
pub fn page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 5: @let Pattern for Local Variables"</h1>
                    <p class={subtitle}>"Using @let for local variable declarations"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ @let for local variable declarations"</li>
                        <li class={point}>"✅ Works within html! macro"</li>
                        <li class={point}>"✅ Can be used for calculations"</li>
                        <li class={point}>"✅ Supports complex expressions"</li>
                        <li class={point}>"✅ Enables cleaner component logic"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        @let_pattern_example()
                    </div>
                    <div class={example_card}>
                        @let_with_conditions()
                    </div>
                    <div class={example_card}>
                        @let_composition_example()
                    </div>
                </section>
                @LessonNav(
                    prev_num=Some(4),
                    next_num=Some(6),
                    prev_title="Children & Layouts",
                    next_title="Control Flow",
                )
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #2dd4bf, #38bdf8)";
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
pub async fn lesson5_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
