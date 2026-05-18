use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

/// Lesson 10: Client-Side UI State with `az-ui` and `set`
///
/// Learn when to use az-ui (client-side) vs az-scope (server-side) state.
#[azumi::page(route = "/lesson-10")]
#[azumi::component]
pub fn render_page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={modern_h1}>"Lesson 10: Client-Side UI State"</h1>
                    <p class={explanation}>
                        "Azumi is server-first, but sometimes you need pure client-side interactivity for "
                        "ephemeral UI state like tabs, accordions, and toggles. "
                        "For this, we use the "<span class={code}>"az-ui"</span>" attribute with the "<span class={code}>"set"</span>" command."
                    </p>
                </header>

                // ==========================================
                // Example 1: Tabs
                // ==========================================
                <div class={modern_card} az-ui="{ \"active_tab\": \"rust\" }">
                    <h2 class={modern_h2}>"Example 1: Tabs"</h2>
                    <p class={text_dim_mb}>"State is local to the browser. Refreshing resets it."</p>

                    <div class={tabs}>
                        <button
                            class={tab_btn}
                            // Bind class 'active' if active_tab == 'rust'
                            az-bind:class:active="active_tab == 'rust'"
                            // On click, set active_tab locally
                            az-on="click set active_tab = 'rust'"
                        >
                            "Rust"
                        </button>
                        <button
                            class={tab_btn}
                            az-bind:class:active="active_tab == 'python'"
                            az-on="click set active_tab = 'python'"
                        >
                            "Python"
                        </button>
                        <button
                            class={tab_btn}
                            az-bind:class:active="active_tab == 'js'"
                            az-on="click set active_tab = 'js'"
                        >
                            "JavaScript"
                        </button>
                    </div>

                    <div class={tab_content} az-bind:class:active="active_tab == 'rust'">
                        <h3 class={content_title}>"Rust"</h3>
                        <p>"Rust is blazingly fast and memory-efficient with no garbage collector."</p>
                    </div>
                    <div class={tab_content} az-bind:class:active="active_tab == 'python'">
                        <h3 class={content_title}>"Python"</h3>
                        <p>"Python is great for data science, AI, and scripting."</p>
                    </div>
                    <div class={tab_content} az-bind:class:active="active_tab == 'js'">
                        <h3 class={content_title}>"JavaScript"</h3>
                        <p>"JavaScript powers the web... but Azumi helps you write less of it!"</p>
                    </div>
                </div>

                // ==========================================
                // Example 2: Accordion
                // ==========================================
                <div class={modern_card} az-ui="{ \"acc1\": false, \"acc2\": false }">
                    <h2 class={modern_h2}>"Example 2: Accordion"</h2>

                    <div class={accordion_item}>
                        <div
                            class={accordion_header}
                            az-on="click set acc1 = !acc1"
                        >
                            "Section 1: Why Azumi?"
                            <span az-bind:text="acc1 ? '−' : '+'" class={toggle_icon}>"+"</span>
                        </div>
                        // Show body if acc1 is true
                        <div class={accordion_body} az-bind:class:open="acc1">
                            <p>"Because it brings compile-time safety to your frontend code!"</p>
                        </div>
                    </div>

                    <div class={accordion_item}>
                        <div
                            class={accordion_header}
                            az-on="click set acc2 = !acc2"
                        >
                            "Section 2: How does it work?"
                            <span az-bind:text="acc2 ? '−' : '+'" class={toggle_icon}>"+"</span>
                        </div>
                        <div class={accordion_body} az-bind:class:open="acc2">
                            <p>"It uses Rust macros to analyze your code and generate optimized HTML and minimal JS."</p>
                        </div>
                    </div>
                </div>

                <div class={modern_card}>
                    <h2 class={modern_h2}>"When to use what?"</h2>
                    <ul class={info_list}>
                        <li class={info_item}><strong class={strong}>"az-ui + set:"</strong>" UI state (tabs, modals, toggles). Ephemeral — lost on refresh."</li>
                        <li class={info_item}><strong class={strong}>"az-scope + call:"</strong>" Business data (user profile, shopping cart, database records). Persisted on server."</li>
                    </ul>
                </div>
            </div>
            <style>
                .container { max-width: "800px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }

                .modern_h1 {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #facc15, #f59e0b)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }

                .modern_h2 {
                    margin-top: "0";
                    color: "var(--azumi-text)";
                    margin-bottom: "0.5rem";
                    font-size: "1.8rem";
                    font-weight: "600";
                }

                .explanation {
                    color: "var(--azumi-text-dim)";
                    line-height: "1.6";
                    margin-bottom: "2rem";
                    font-size: "1.1rem";
                    max-width: "600px";
                    margin-left: "auto";
                    margin-right: "auto";
                }

                .text_dim_mb {
                    color: "var(--azumi-text-dim)";
                    margin-bottom: "1.5rem";
                }

                .modern_card {
                    border: "1px solid rgba(255,255,255,0.05)";
                    border-radius: "16px";
                    padding: "2rem";
                    margin-bottom: "2rem";
                    background: "rgba(30, 41, 59, 0.6)";
                    backdrop-filter: "blur(10px)";
                    color: "#cbd5e1";
                }

                .code { background: "rgba(255,255,255,0.1)"; padding: "0.2rem 0.4rem"; border-radius: "4px"; font-family: "monospace"; color: "var(--azumi-primary-hover)"; }

                /* Tabs Styling */
                .tabs { display: "flex"; border-bottom: "1px solid var(--azumi-border)"; margin-bottom: "1.5rem"; gap: "0.5rem"; }
                .tab_btn {
                    padding: "0.75rem 1.5rem";
                    border: "none";
                    background: "transparent";
                    cursor: "pointer";
                    font-weight: "600";
                    color: "var(--azumi-text-dim)";
                    border-bottom: "2px solid transparent";
                    margin-bottom: "-1px";
                    transition: "all 0.2s";
                }
                .tab_btn:hover { color: "var(--azumi-text)"; }
                .tab_btn.active { color: "var(--azumi-primary)"; border-bottom-color: "var(--azumi-primary)"; }
                .tab_content { display: "none"; padding: "1rem 0"; animation: "fadeIn 0.3s ease-out"; }
                .tab_content.active { display: "block"; }
                .content_title { color: "var(--azumi-primary)"; margin-bottom: "0.5rem"; font-size: "1.25rem"; }

                @keyframes fadeIn { from { opacity: "0"; transform: "translateY(5px)"; } to { opacity: "1"; transform: "translateY(0)"; } }

                /* Accordion Styling */
                .accordion_item {
                    border: "1px solid var(--azumi-border)";
                    border-radius: "var(--radius-md)";
                    margin-bottom: "1rem";
                    overflow: "hidden";
                    background: "rgba(15, 23, 42, 0.3)";
                }
                .accordion_header {
                    padding: "1rem";
                    background: "rgba(255,255,255,0.02)";
                    cursor: "pointer";
                    font-weight: "600";
                    display: "flex";
                    justify-content: "space-between";
                    align-items: "center";
                    color: "var(--azumi-text)";
                    transition: "background 0.2s";
                    user-select: "none";
                }
                .accordion_header:hover { background: "rgba(255,255,255,0.05)"; }
                .toggle_icon { color: "var(--azumi-primary)"; font-weight: "bold"; font-size: "1.2rem"; }
                .accordion_body { display: "none"; padding: "1.5rem"; border-top: "1px solid var(--azumi-border)"; color: "var(--azumi-text-dim)"; }
                .accordion_body.open { display: "block"; }

                .info_list { list-style: "none"; padding: "0"; display: "grid"; gap: "1rem"; }
                .info_item { padding: "1rem"; background: "rgba(0,0,0,0.2)"; border-radius: "8px"; color: "var(--azumi-text)"; border: "1px solid var(--azumi-border)"; }
                .strong { color: "var(--azumi-primary)"; margin-right: "0.5rem"; }
            </style>
        }
    }
}

pub async fn lesson10_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&render_page()))
}
