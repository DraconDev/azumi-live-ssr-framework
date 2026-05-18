use azumi::prelude::*;

/// Lesson 13: Form Handling
///
/// Building forms with Azumi Live

#[azumi::live]
pub struct ContactForm {
    pub submitted: bool,
}

#[azumi::live_impl(component = "contact_form_view")]
impl ContactForm {
    pub fn submit(&mut self) {
        self.submitted = true;
    }

    pub fn reset(&mut self) {
        self.submitted = false;
    }
}

/// Contact form component
#[azumi::component]
pub fn contact_form_view<'a>(state: &'a ContactForm) -> impl Component + 'a {
    html! {
        <div class={form_wrapper}>
            <div class={card_glow}></div>
            <div class={form_container}>
                <h2 class={form_title}>
                    <span class={icon}>"📬"</span>
                    "Contact Form"
                </h2>

                @if state.submitted {
                    <div class={success_box}>
                        <div class={success_icon}>"✨"</div>
                        <h3 class={success_title}>"Message Sent!"</h3>
                        <p class={success_text}>"We've received your inquiry and will get back to you shortly."</p>
                        <button class={btn btn_secondary} on:click={state.reset}>
                            "Send Another Message"
                        </button>
                    </div>
                }

                @if !state.submitted {
                    <div class={field_group}>
                        <div class={field}>
                            <label class={label}>"Name"</label>
                            <input class={input} type="text" name="name" placeholder="Ex: Alice Smith" />
                        </div>
                        <div class={field}>
                            <label class={label}>"Email"</label>
                            <input class={input} type="email" name="email" placeholder="alice@example.com" />
                        </div>
                    </div>
                    <div class={field}>
                        <label class={label}>"Message"</label>
                        <textarea class={textarea} name="message" placeholder="How can we help you today?"></textarea>
                    </div>
                    <div class={actions}>
                        <button class={btn btn_primary} type="button" on:click={state.submit}>
                            "Send Message"
                        </button>
                    </div>
                }
            </div>
        </div>
        <style>
            .form_wrapper {
                position: "relative";
                padding: "2px";
                border-radius: "20px";
                background: "linear-gradient(135deg, rgba(255,255,255,0.1), rgba(255,255,255,0.05))";
            }
            .card_glow {
                position: "absolute";
                top: "0";
                left: "0";
                right: "0";
                bottom: "0";
                background: "radial-gradient(circle at top right, rgba(99, 102, 241, 0.15), transparent 70%)";
                pointer-events: "none"; border-radius: "20px";
            }

            .form_container {
                max-width: "480px";
                padding: "2.5rem";
                background: "var(--azumi-bg-card, rgba(15, 23, 42, 0.8))";
                backdrop-filter: "blur(12px)";
                border-radius: "18px";
                border: "1px solid var(--azumi-border, rgba(255,255,255,0.1))";
                box-shadow: "0 25px 50px -12px rgba(0, 0, 0, 0.25)";
            }

            .form_title {
                margin-bottom: "2rem";
                color: "var(--azumi-text, #f8fafc)";
                font-size: "1.75rem";
                font-weight: "700";
                display: "flex"; align-items: "center"; gap: "0.75rem";
            }
            .icon { font-size: "1.5rem"; }

            .field_group {
                display: "grid";
                grid-template-columns: "repeat(auto-fit, minmax(200px, 1fr))";
                gap: "1.5rem";
                margin-bottom: "1.5rem";
            }

            .field {
                display: "flex";
                flex-direction: "column";
                gap: "0.5rem";
                margin-bottom: "1px"; /* Collapsing margins fix */
                width: "100%";
            }

            .label {
                font-size: "0.875rem";
                font-weight: "600";
                color: "var(--azumi-text-dim, #94a3b8)";
                text-transform: "uppercase";
                letter-spacing: "0.05em";
            }

            .input, .textarea {
                width: "100%";
                box-sizing: "border-box";
                padding: "0.875rem 1rem";
                background: "rgba(0, 0, 0, 0.2)";
                border: "1px solid var(--azumi-border, rgba(255,255,255,0.1))";
                border-radius: "10px";
                color: "var(--azumi-text, white)";
                font-size: "1rem";
                transition: "all 0.2s ease";
                font-family: "inherit";
            }
            .input:focus, .textarea:focus {
                outline: "none";
                border-color: "var(--azumi-primary, #6366f1)";
                background: "rgba(99, 102, 241, 0.05)";
                box-shadow: "0 0 0 4px rgba(99, 102, 241, 0.1)";
            }

            .textarea {
                min-height: "140px";
                resize: "vertical";
                line-height: "1.6";
            }

            .actions { margin-top: "1rem"; }

            .btn {
                padding: "0.875rem 2rem";
                border: "none";
                border-radius: "10px";
                font-size: "1rem";
                font-weight: "600";
                cursor: "pointer";
                width: "100%";
                transition: "all 0.2s cubic-bezier(0.4, 0, 0.2, 1)";
                display: "flex"; justify-content: "center"; align-items: "center";
            }
            .btn:active { transform: "scale(0.98)"; }

            .btn_primary {
                background: "linear-gradient(135deg, var(--azumi-primary, #6366f1), var(--azumi-accent, #8b5cf6))";
                color: "white";
                box-shadow: "0 10px 15px -3px rgba(99, 102, 241, 0.3)";
                text-shadow: "0 1px 2px rgba(0,0,0,0.2)";
            }
            .btn_primary:hover {
                filter: "brightness(1.1)";
                box-shadow: "0 20px 25px -5px rgba(99, 102, 241, 0.4)";
                transform: "translateY(-2px)";
            }

            .btn_secondary {
                background: "rgba(255,255,255,0.05)";
                color: "var(--azumi-text, white)";
                border: "1px solid rgba(255,255,255,0.1)";
            }
            .btn_secondary:hover {
                background: "rgba(255,255,255,0.1)";
            }

            .success_box {
                padding: "3rem 2rem";
                text-align: "center";
                display: "flex"; flex-direction: "column"; align-items: "center";
                animation: "scaleIn 0.4s cubic-bezier(0.16, 1, 0.3, 1)";
            }
            .success_icon {
                font-size: "4rem";
                margin-bottom: "1rem";
                animation: "pop 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) 0.2s backwards";
            }
            .success_title {
                color: "var(--azumi-text, white)";
                margin-bottom: "0.5rem";
                font-size: "1.5rem";
            }
            .success_text {
                color: "var(--azumi-text-dim, #94a3b8)";
                margin-bottom: "2rem";
                line-height: "1.5";
            }

            @keyframes scaleIn {
                from { opacity: "0"; transform: "scale(0.9)"; }
                to { opacity: "1"; transform: "scale(1)"; }
            }
            @keyframes pop {
                from { opacity: "0"; transform: "scale(0.5)"; }
                to { opacity: "1"; transform: "scale(1)"; }
            }
        </style>
    }
}

use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;

/// Full page component ensuring script injection
#[azumi::component]
pub fn lesson13_page<'a>(state: &'a ContactForm) -> impl Component + 'a {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 13: Form Handling"</h1>
                    <p class={subtitle}>"Building forms with Azumi Live"</p>
                </header>

                <div class={explanation}>
                    <h3 class={exp_title}>"📝 Form Patterns"</h3>
                    <ul class={exp_list}>
                        <li class={exp_item}><strong>"Submit action"</strong>" - Toggles submitted state"</li>
                        <li class={exp_item}><strong>"Reset action"</strong>" - Clears form state"</li>
                        <li class={exp_item}><strong>"Conditional rendering"</strong>" - Shows form or success message"</li>
                    </ul>
                </div>

                <div class={demo_area}>
                    @contact_form_view(state = state)
                </div>

                @LessonNav(
                    prev_num=Some(12),
                    next_num=Some(14),
                    prev_title="Images & Media",
                    next_title="Composing Live",
                )
            </div>
        }
        <style>
            .container { max-width: "800px"; margin: "0 auto"; }
            .header { text-align: "center"; margin-bottom: "3rem"; }
            .main_title {
                font-size: "3rem";
                font-weight: "800";
                color: "#e2e8f0";
                background: "linear-gradient(to right, #fbbf24, #f59e0b)";
                -webkit-background-clip: "text";
                -webkit-text-fill-color: "transparent";
                margin-bottom: "1rem";
            }
            .subtitle { font-size: "1.25rem"; color: "#94a3b8"; }

            .explanation {
                background: "rgba(30, 41, 59, 0.4)";
                padding: "2rem";
                border-radius: "16px";
                margin: "0 auto 3rem";
                border: "1px solid rgba(255,255,255,0.05)";
                max-width: "600px";
            }
            .exp_title { color: "#f59e0b"; font-size: "1.25rem"; margin-bottom: "1rem"; }
            .exp_list { color: "#cbd5e1"; padding-left: "1.5rem"; display: "flex"; flex-direction: "column"; gap: "0.5rem"; }
            .exp_item { line-height: "1.6"; }

            .demo_area { display: "flex"; justify-content: "center"; margin: "2rem 0"; }
        </style>
    }
}

// Handler for Axum
pub async fn lesson13_handler() -> impl axum::response::IntoResponse {
    let form_state = ContactForm { submitted: false };
    use lesson13_page::Props;
    let page = lesson13_page::render(Props::builder().state(&form_state).build().expect("props"));
    axum::response::Html(azumi::render_to_string(&page))
}
