use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

/// Lesson 12: Image Optimization
///
/// Demonstrates the `@Image` component for performance.
#[azumi::page(route = "/lesson-12")]
#[azumi::component]
pub fn render_page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 12: Image Optimization"</h1>
                    <p class={explanation}>
                        "Azumi automatically optimizes standard " <strong class={strong}>"<img>"</strong> " tags with lazy loading and async decoding."
                    </p>
                </header>

                <div class={section}>
                    <h2 class={section_title}>"1. Basic Optimized Image"</h2>
                    <div class={code}>
                        "<img src=\"/static/photo.jpg\" width=\"800\" height=\"600\" alt=\"Photo\" />"
                    </div>
                    <div class={card}>
                        <div class={label}>"Output HTML"</div>
                        <div class={code_comment}>
                            "&lt;img src=\"...\" loading=\"lazy\" decoding=\"async\" width=\"800\" ...&gt;"
                        </div>

                        // Usage of the component
                        <div class={image_wrapper}>
                            <img
                                src="/static/photo.jpg"
                                width="800"
                                height="600"
                                alt="Photo"
                                class={demo_img}
                            />
                        </div>
                    </div>
                </div>

                <div class={section}>
                    <h2 class={section_title}>"2. Eager Loading (Above the Fold)"</h2>
                    <p class={subtext}>"For hero images at the top of the page, use eager loading."</p>
                    <div class={code}>
                        "<img ... loading=\"eager\" />"
                    </div>
                    <div class={card}>
                        <div class={image_wrapper}>
                            <img
                                src="/static/hero.jpg"
                                alt="Hero mountain"
                                width="800"
                                height="600"
                                loading="eager"
                                class={demo_img}
                            />
                        </div>
                    </div>
                </div>

                <div class={section}>
                    <h2 class={section_title}>"3. Responsive Images (srcset)"</h2>
                    <p class={subtext}>"Automatically serve the right size for the device."</p>
                    <div class={code}>
                        "<img srcset=\"...\" sizes=\"...\" />"
                    </div>
                    <div class={card}>
                        <div class={label}>"Try resizing window"</div>

                        // We use a placeholder service that supports width parameter
                        // In real app, you'd have photo-400.jpg, photo-800.jpg on disk
                        <div class={image_wrapper}>
                            <img
                                src="/static/nature.jpg"
                                srcset="/static/nature.jpg 600w,
                                        /static/nature.jpg 1200w"
                                sizes="(max-width: 600px) 100vw, 800px"
                                alt="Responsive nature"
                                class={responsive_img}
                            />
                        </div>
                    </div>
                </div>
            </div>
            <style>
                .container { max-width: "800px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #14b8a6, #0d9488)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .explanation { color: "#94a3b8"; line-height: "1.6"; margin-bottom: "2rem"; font-size: "1.1rem"; }
                .strong { color: "#2dd4bf"; }

                .section { margin-bottom: "3rem"; }
                .section_title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.5rem"; border-bottom: "1px solid rgba(255,255,255,0.1)"; padding-bottom: "0.5rem"; }
                .subtext { color: "#94a3b8"; margin-bottom: "1rem"; }

                .code { background: "rgba(0,0,0,0.3)"; padding: "1rem"; border-radius: "8px"; font-family: "monospace"; color: "#cbd5e1"; margin-bottom: "1rem"; border: "1px solid rgba(255,255,255,0.05)"; }

                .card {
                    border: "1px solid rgba(255,255,255,0.1)";
                    border-radius: "16px";
                    padding: "2rem";
                    background: "rgba(30, 41, 59, 0.6)";
                    backdrop-filter: "blur(10px)";
                }

                .label {
                    display: "inline-block"; background: "rgba(20, 184, 166, 0.2)"; color: "#5eead4";
                    padding: "0.25rem 0.5rem"; border-radius: "4px"; font-size: "0.8rem";
                    margin-bottom: "0.5rem"; font-weight: "600";
                }
                .code_comment { color: "#64748b"; font-family: "monospace"; font-size: "0.9rem"; margin-bottom: "1rem"; }

                .image_wrapper {
                    border-radius: "8px";
                    overflow: "hidden";
                    border: "1px solid rgba(255,255,255,0.1)";
                    box-shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.2)";
                }
                .demo_img { max-width: "100%"; height: "auto"; display: "block"; }
                .responsive_img { width: "100%"; height: "auto"; display: "block"; }
            </style>
        }
    }
}

pub async fn lesson12_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&render_page()))
}
