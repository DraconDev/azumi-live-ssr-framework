use azumi::prelude::*;

/// Previous / Next navigation between lessons
#[azumi::component]
#[allow(non_snake_case)]
pub fn LessonNav<'a>(
    prev_num: Option<i32>,
    next_num: Option<i32>,
    prev_title: &'a str,
    next_title: &'a str,
) -> impl Component + 'a {
    html! {
        <nav class={nav_container}>
            <div class={nav_row}>
                @if let Some(n) = prev_num {
                    <a href={lesson_route(n)} class={nav_link_prev}>
                        <span class={nav_arrow}>"←"</span>
                        <div class={nav_text_block}>
                            <span class={nav_label}>"Previous"</span>
                            <span class={nav_title_text}>{prev_title}</span>
                        </div>
                    </a>
                } else {
                    <a href="/" class={nav_link_prev}>
                        <span class={nav_arrow}>"←"</span>
                        <div class={nav_text_block}>
                            <span class={nav_label}>"Back"</span>
                            <span class={nav_title_text}>"Home"</span>
                        </div>
                    </a>
                }

                @if let Some(n) = next_num {
                    <a href={lesson_route(n)} class={nav_link_next}>
                        <div class={nav_text_block_right}>
                            <span class={nav_label}>"Next"</span>
                            <span class={nav_title_text}>{next_title}</span>
                        </div>
                        <span class={nav_arrow}>"→"</span>
                    </a>
                } else {
                    <a href="/" class={nav_link_next}>
                        <div class={nav_text_block_right}>
                            <span class={nav_label}>"Done"</span>
                            <span class={nav_title_text}>"Home"</span>
                        </div>
                        <span class={nav_arrow}>"→"</span>
                    </a>
                }
            </div>
        </nav>
        <style>
            .nav_container {
                margin-top: "3rem";
                padding-top: "2rem";
                border-top: "1px solid rgba(255,255,255,0.05)";
            }
            .nav_row {
                display: "flex";
                justify-content: "space-between";
                gap: "1rem";
            }
            .nav_link_prev, .nav_link_next {
                display: "flex";
                align-items: "center";
                gap: "0.75rem";
                padding: "1rem 1.5rem";
                background: "rgba(30, 41, 59, 0.4)";
                border: "1px solid rgba(255,255,255,0.05)";
                border-radius: "12px";
                text-decoration: "none";
                transition: "all 0.2s ease";
                flex: "1";
                max-width: "50%";
            }
            .nav_link_prev:hover, .nav_link_next:hover {
                background: "rgba(30, 41, 59, 0.8)";
                border-color: "rgba(255,255,255,0.1)";
                transform: "translateY(-2px)";
            }
            .nav_arrow {
                font-size: "1.25rem";
                color: "#818cf8";
                font-weight: "bold";
            }
            .nav_text_block { display: "flex"; flex-direction: "column"; }
            .nav_text_block_right { display: "flex"; flex-direction: "column"; text-align: "right"; }
            .nav_label {
                font-size: "0.75rem";
                color: "#64748b";
                text-transform: "uppercase";
                letter-spacing: "0.05em";
                font-weight: "600";
            }
            .nav_title_text {
                font-size: "0.9rem";
                color: "#e2e8f0";
                font-weight: "500";
            }
        </style>
    }
}

/// Map lesson number to route path
fn lesson_route(n: i32) -> String {
    match n {
        0 => crate::examples::lessons::pages::lesson0::page_ROUTE.to_string(),
        1 => crate::examples::lessons::pages::lesson1::page_ROUTE.to_string(),
        2 => crate::examples::lessons::pages::lesson2::page_ROUTE.to_string(),
        3 => crate::examples::lessons::pages::lesson3::page_ROUTE.to_string(),
        4 => crate::examples::lessons::pages::lesson4::page_ROUTE.to_string(),
        5 => crate::examples::lessons::pages::lesson5::page_ROUTE.to_string(),
        6 => crate::examples::lessons::pages::lesson6::page_ROUTE.to_string(),
        7 => crate::examples::lessons::pages::lesson7::page_ROUTE.to_string(),
        8 => crate::examples::lessons::pages::lesson8::page_ROUTE.to_string(),
        9 => crate::examples::lessons::pages::lesson9::page_ROUTE.to_string(),
        10 => crate::examples::lessons::pages::lesson10::render_page_ROUTE.to_string(),
        11 => crate::examples::lessons::pages::lesson11::page_ROUTE.to_string(),
        12 => crate::examples::lessons::pages::lesson12::render_page_ROUTE.to_string(),
        13 => "/lesson-13".to_string(),
        14 => crate::examples::lessons::pages::lesson14::render_page_ROUTE.to_string(),
        15 => crate::examples::lessons::pages::lesson15::render_page_ROUTE.to_string(),
        16 => "/lesson-16".to_string(),
        17 => crate::examples::lessons::pages::lesson17_testing::render_page_ROUTE.to_string(),
        18 => crate::examples::lessons::pages::lesson18_security::render_page_ROUTE.to_string(),
        19 => "/lesson-19".to_string(),
        20 => "/lesson-20".to_string(),
        _ => "/".to_string(),
    }
}

/// Lesson titles for navigation
pub const LESSON_TITLES: &[&str] = &[
    "Introduction",            // 0
    "CSS Scoping",             // 1
    "Global vs Component CSS", // 2
    "Composition",             // 3
    "Children & Layouts",      // 4
    "@let Pattern",            // 5
    "Control Flow",            // 6
    "Form Handling",           // 7
    "Action System",           // 8
    "Azumi Live",              // 9
    "Client-Side UI State",    // 10
    "Async Patterns",          // 11
    "Images & Media",          // 12
    "Live Forms",              // 13
    "Composing Live",          // 14
    "Full Application",        // 15
    "Async Database",          // 16
    "Testing",                 // 17
    "Security",                // 18
    "Authentication",          // 19
    "Custom Inputs",           // 20
];

/// Get prev/next lesson numbers and titles
pub fn lesson_prev_next(current: i32) -> (Option<i32>, Option<i32>, &'static str, &'static str) {
    let prev_num = if current > 0 { Some(current - 1) } else { None };
    let next_num = if current < 20 { Some(current + 1) } else { None };
    let prev_title = prev_num.map(|n| LESSON_TITLES[n as usize]).unwrap_or("Home");
    let next_title = next_num.map(|n| LESSON_TITLES[n as usize]).unwrap_or("Home");
    (prev_num, next_num, prev_title, next_title)
}
