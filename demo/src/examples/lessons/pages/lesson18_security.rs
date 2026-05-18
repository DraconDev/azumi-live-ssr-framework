use axum::response::{Html, IntoResponse};
use azumi::prelude::*;

/// Live component with secure state
#[azumi::live]
pub struct SecureCounter {
    pub count: i32,
    pub is_admin: bool,
}

#[azumi::live_impl(component = "secure_view")]
impl SecureCounter {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}

// -----------------------------------------------------------------------------
// VERIFICATION TEST
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_implicit_security_rejection() {
        let state = SecureCounter {
            count: 10,
            is_admin: false,
        };
        let signed_scope = state.to_scope();

        // 1. Verify normal request works (Implicitly signed)
        let response = __azumi_live_handlers_securecounter::increment_handler(signed_scope.clone()).await;
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Valid signed state should be accepted"
        );

        // 2. Simulate Attacker trying to modify state client-side
        // They try to change is_admin to true without updating the signature
        let tampered_scope = signed_scope.replace("false", "true");

        println!("Tampered Scope: {}", tampered_scope);

        // 3. Verify the handler REJECTS it automatically
        // We didn't write any verification code in SecureCounter, but the macro provided it.
        let response = __azumi_live_handlers_securecounter::increment_handler(tampered_scope).await;
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "Tampered state should be rejected automatically"
        );
    }
}

#[azumi::page(route = "/lesson-18")]
#[azumi::component]
pub fn render_page() -> impl Component {
    let initial_state = SecureCounter {
        count: 0,
        is_admin: false,
    };

    html! {
        @crate::examples::lessons::components::layout::DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 18: Security"</h1>
                    <p class={subtitle}>"Signed State & Anti-Tampering"</p>
                </header>

                <div class={info_card}>
                    <div class={info_content}>
                        <div class={info_icon}>"🔒"</div>
                        <p class={info_text}>
                            "Azumi automatically signs all component state with HMAC-SHA256. "
                            "If a malicious user tries to modify the JSON in "
                            <span class={code_snippet}>"az-scope"</span>
                            ", the server will reject the action."
                        </p>
                    </div>
                </div>

                <div class={demo_wrapper}>
                    @secure_view(state=&initial_state)
                </div>

                <div class={verify_steps}>
                    <h3 class={verify_title}>"How to Verify:"</h3>
                    <ol class={step_list}>
                        <li class={step_item}>"Inspect the button below in DevTools."</li>
                        <li class={step_item}>"Find the parent div with the "<span class={code_snippet}>"az-scope"</span>" attribute."</li>
                        <li class={step_item}>"The attribute contains formatted: "<span class={code_snippet}>"JSON|SIGNATURE"</span>"."</li>
                        <li class={step_item}>"Try changing "<span class={code_snippet}>"\"is_admin\":false"</span>" to "<span class={code_snippet}>"true"</span>"."</li>
                        <li class={step_item}>"Click 'Secure Increment' and watch the network request fail (400 Bad Request)."</li>
                    </ol>
                </div>
            </div>
            <style>
                .container { max-width: "700px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    margin-bottom: "1rem";
                    background: "linear-gradient(to right, #f87171, #ef4444)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                }
                .subtitle { color: "#94a3b8"; font-size: "1.2rem"; }

                .info_card {
                    background: "rgba(239, 68, 68, 0.1)";
                    border: "1px solid rgba(239, 68, 68, 0.2)";
                    border-radius: "12px";
                    padding: "1.5rem";
                    margin-bottom: "2rem";
                }
                .info_content { display: "flex"; gap: "1rem"; align-items: "flex-start"; }
                .info_icon { font-size: "1.5rem"; }
                .info_text { color: "#fca5a5"; margin: "0"; line-height: "1.6"; }

                .code_snippet {
                    font-family: "monospace";
                    background: "rgba(0,0,0,0.3)";
                    color: "#fca5a5";
                    padding: "0.2rem 0.4rem";
                    border-radius: "4px";
                    font-size: "0.9em";
                }

                .demo_wrapper { margin-bottom: "2rem"; }

                .verify_steps {
                    background: "rgba(30, 41, 59, 0.6)";
                    backdrop-filter: "blur(10px)";
                    padding: "1.5rem";
                    border-radius: "12px";
                    border: "1px solid rgba(255,255,255,0.05)";
                }
                .verify_title { margin: "0 0 1rem 0"; font-size: "1.1rem"; color: "#cbd5e1"; }
                .step_list { margin: "0"; padding-left: "1.2rem"; display: "grid"; gap: "0.5rem"; }
                .step_item { color: "#94a3b8"; }
            </style>
        }
    }
}

// Update secure_view with scoped styles
#[azumi::component]
pub fn secure_view<'a>(state: &'a SecureCounter) -> impl Component + 'a {
    html! {
        <div class={card}>
            <div class={counter_display}>
                <div class={count_val}>{state.count}</div>
                // Combine base badge class with conditional specific class
                <span class={if state.is_admin { format!("{} {}", status_badge, status_admin) } else { format!("{} {}", status_badge, status_user) }}>
                    {if state.is_admin { "Admin Access Unlocked" } else { "Standard User Access" }}
                </span>
            </div>

            <button class={btn} on:click={state.increment}>
                "Secure Increment"
            </button>
        </div>
        <style>
             .card {
                background: "rgba(30, 41, 59, 0.4)";
                border-radius: "16px";
                padding: "2rem";
                border: "1px solid rgba(255,255,255,0.05)";
                display: "flex";
                flex-direction: "column";
                align-items: "center";
            }
            .counter_display { text-align: "center"; margin-bottom: "2rem"; }
            .count_val {
                font-size: "5rem";
                font-weight: "800";
                color: "#f8fafc";
                line-height: "1";
                margin-bottom: "1rem";
                text-shadow: "0 4px 12px rgba(0,0,0,0.5)";
            }

            .status_badge {
                display: "inline-block";
                padding: "0.5rem 1rem";
                border-radius: "9999px";
                font-size: "0.875rem";
                font-weight: "600";
                letter-spacing: "0.05em";
                text-transform: "uppercase";
            }
            .status_user { background: "rgba(148, 163, 184, 0.2)"; color: "#cbd5e1"; border: "1px solid rgba(148, 163, 184, 0.2)"; }
            .status_admin { background: "rgba(34, 197, 94, 0.2)"; color: "#86efac"; border: "1px solid rgba(34, 197, 94, 0.3)"; box-shadow: "0 0 10px rgba(34, 197, 94, 0.2)"; }

            .btn {
                display: "inline-flex";
                align-items: "center";
                justify-content: "center";
                padding: "1rem 2rem";
                background: "linear-gradient(to right, #ef4444, #dc2626)";
                color: "white";
                font-weight: "700";
                border-radius: "8px";
                border: "none";
                cursor: "pointer";
                transition: "all 0.2s";
                font-size: "1.1rem";
                box-shadow: "0 4px 6px -1px rgba(220, 38, 38, 0.3)";
            }
            .btn:hover { transform: "translateY(-2px)"; box-shadow: "0 10px 15px -3px rgba(220, 38, 38, 0.4)"; }
            .btn:active { transform: "translateY(0)"; }
        </style>
    }
}

pub async fn lesson18_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&render_page()))
}
