use crate::examples::lessons::components::layout::DarkModernLayout;
use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use azumi::prelude::*;

/// Lesson 11: Async Loading Patterns
///
/// Demonstrates how to handle loading and error states for async operations.
/// Key concept: Use `loading: bool` and `error: Option<String>` in your state.

#[azumi::live]
pub struct UserLoader {
    pub loading: bool,
    pub error: Option<String>,
    pub users: Vec<String>,
}

#[azumi::live_impl(component = "user_loader_view")]
impl UserLoader {
    // Optimistic Update: Set loading=true instantly on the client
    // Optimistic Update: Set preview=true instantly. Azumi predicts this!
    pub async fn load_users(&mut self) {
        // 1. Optimistic Prediction (Azumi detects this assignment)
        self.loading = true;

        // 2. Server-side delay (simulating DB)
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // 3. Update State (Server Response)
        self.users = vec![
            "Alice Chen".to_string(),
            "Bob Smith".to_string(),
            "Charlie Kim".to_string(),
        ];
        self.loading = false;
    }

    // Optimistic Update: Set loading=true instantly
    pub async fn load_fail(&mut self) {
        self.loading = true;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        self.loading = false;
        self.error = Some("Network timeout: Could not reach user database.".to_string());
        self.users.clear();
    }

    // Optimistic Update: Clear everything instantly
    pub fn reset(&mut self) {
        self.loading = false;
        self.error = None;
        self.users.clear();
    }
}

#[azumi::component]
pub fn user_loader_view<'a>(state: &'a UserLoader) -> impl Component + 'a {
    html! {
        <div class={container}>
            <div class={card}>
                <div class={header}>
                    <h1 class={title}>"Async Data Loading"</h1>
                    <p class={subtitle}>"Click to see optimistic state updates."</p>
                </div>

                // ===============================================
                // The Pattern: Logic-less View Switching
                // ===============================================

                <div class={content_area}>
                    @if state.loading {
                         <div class={loading_state}>
                            <h3 style={ --color: "var(--azumi-text)"; --margin-bottom: "1rem" }>"Optimistic Loading..."</h3>
                            // Skeleton UI Pattern
                            <div class={skeleton_row}></div>
                            <div class={skeleton_row}></div>
                            <div class={skeleton_row}></div>
                        </div>
                    } else {
                        @if state.error.is_some() {
                            <div class={error_state}>
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <line x1="12" y1="8" x2="12" y2="12"></line>
                                    <line x1="12" y1="16" x2="12.01" y2="16"></line>
                                </svg>
                                <div>
                                    <strong>"Error Occurred"</strong>
                                    <p>{state.error.as_ref().unwrap()}</p>
                                </div>
                            </div>
                        } else {
                            @if state.users.is_empty() {
                                <div class={empty_state}>
                                    "No users loaded. Ready to fetch."
                                </div>
                            } else {
                                <ul class={user_list}>
                                    @for user in &state.users {
                                        <li class={user_item}>
                                            <div class={avatar}>{&user[0..1]}</div>
                                            {user}
                                        </li>
                                    }
                                </ul>
                            }
                        }
                    }
                </div>

                <div class={controls}>
                    // Use on:click syntax which auto-generates the correct az-on attribute
                    <button class={modern_btn} on:click={state.load_users}>
                        "Load Users (Success)"
                    </button>
                    <button
                        class={modern_btn}
                        style={ --azumi-primary: "#ef4444"; --azumi-primary-hover: "#dc2626" }
                        on:click={state.load_fail}
                    >
                        "Load Users (Fail)"
                    </button>
                    <button
                        class={modern_btn}
                        style={
                            --azumi-primary: "transparent";
                            --azumi-primary-hover: "var(--azumi-bg-subtle)";
                            --border-color: "var(--azumi-border)"
                        }
                        on:click={state.reset}
                    >
                        "Reset"
                    </button>
                </div>
            </div>

                @LessonNav(
                    prev_num=Some(10),
                    next_num=Some(12),
                    prev_title="Client-Side UI State",
                    next_title="Images & Media",
                )
        </div>
        <style>
            .container { max-width: "700px"; margin: "0 auto"; }
            .card {
                /* Use Theme Variables */
                border: "1px solid var(--azumi-border)";
                border-radius: "var(--radius-lg)";
                padding: "var(--spacing-xl)";
                background: "var(--azumi-bg-card)";
                backdrop-filter: "blur(12px)";
                color: "var(--azumi-text)";
                box-shadow: "0 10px 15px -3px rgba(0, 0, 0, 0.1)";
            }
            .header { text-align: "center"; margin-bottom: "var(--spacing-xl)"; }
            .title { color: "var(--azumi-text)"; margin-bottom: "var(--spacing-xs)"; font-size: "2rem"; }
            .subtitle { color: "var(--azumi-text-dim)"; font-size: "1rem"; }

            .content_area {
                min-height: "240px";
                display: "flex";
                flex-direction: "column";
                justify-content: "center";
                background: "rgba(0, 0, 0, 0.2)";
                border-radius: "var(--radius-md)";
                margin-bottom: "var(--spacing-lg)";
                border: "1px solid var(--azumi-border)";
            }

            /* Loading State */
            .loading_state { text-align: "center"; padding: "2rem"; color: "var(--azumi-text-dim)"; }
            .spinner {
                display: "inline-block"; width: "40px"; height: "40px";
                border: "3px solid var(--azumi-border)";
                border-top-color: "var(--azumi-primary)";
                border-radius: "50%";
                animation: "spin 1s linear infinite";
                margin-bottom: "var(--spacing-md)";
            }
            @keyframes spin { to { transform: "rotate(360deg)"; } }

            /* Error State */
            .error_state {
                background: "rgba(220, 38, 38, 0.1)"; color: "#fca5a5";
                padding: "var(--spacing-lg)";
                border-radius: "var(--radius-md)";
                border: "1px solid rgba(220, 38, 38, 0.2)";
                display: "flex"; align_items: "center"; gap: "var(--spacing-md)";
                margin: "var(--spacing-lg)";
            }

            /* Data State */
            .user_list { list-style: "none"; padding: "var(--spacing-lg)"; display: "grid"; gap: "var(--spacing-sm)"; margin: "0"; }
            .user_item {
                display: "flex"; align_items: "center"; gap: "var(--spacing-md)";
                padding: "var(--spacing-md)";
                border-bottom: "1px solid var(--azumi-border)";
                background: "rgba(255,255,255,0.02)";
                border-radius: "var(--radius-md)";
                transition: "background 0.2s";
            }
            .user_item:hover { background: "rgba(255,255,255,0.05)"; }
            .user_item:last-child { border-bottom: "none"; }

            .avatar {
                width: "40px"; height: "40px";
                background: "linear-gradient(135deg, var(--azumi-primary), var(--azumi-accent))";
                color: "white"; border-radius: "50%";
                display: "flex"; align-items: "center"; justify-content: "center";
                font-weight: "bold"; font-size: "1.2rem";
                box-shadow: "0 2px 4px rgba(0,0,0,0.2)";
            }

            /* Controls */
            .controls {
                display: "flex"; gap: "var(--spacing-md)"; justify-content: "center"; flex-wrap: "wrap";
                padding-top: "var(--spacing-lg)";
                border-top: "1px solid var(--azumi-border)";
            }

            .modern_btn {
                padding: "0.85rem 1.75rem";
                border-radius: "12px";
                font-weight: "600";
                border: "var(--border-color, 1px solid rgba(255,255,255,0.1))";
                cursor: "pointer";
                transition: "all 0.2s cubic-bezier(0.4, 0, 0.2, 1)";
                background: "linear-gradient(135deg, var(--azumi-primary), var(--azumi-primary-hover))";
                color: "white";
                box-shadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)";
                text-shadow: "0 1px 2px rgba(0,0,0,0.2)";
                letter-spacing: "0.025em";
            }
            .modern_btn:hover {
                transform: "translateY(-2px)";
                box-shadow: "0 10px 15px -3px rgba(0, 0, 0, 0.2), 0 4px 6px -2px rgba(0, 0, 0, 0.1)";
                filter: "brightness(1.1)";
            }
            .modern_btn:active {
                transform: "translateY(0)";
            }

            .skeleton_row {
                height: "40px";
                margin-bottom: "10px";
                background: "linear-gradient(90deg, rgba(255,255,255,0.05) 25%, rgba(255,255,255,0.1) 50%, rgba(255,255,255,0.05) 75%)";
                background-size: "200% 100%";
                border-radius: "8px";
                animation: "shimmer 1.5s infinite";
            }
            @keyframes shimmer {
                "0%" { background-position: "200% 0"; }
                "100%" { background-position: "-200% 0"; }
            }

            .empty_state { text-align: "center"; color: "var(--azumi-text-dim)"; padding: "2rem"; font-style: "italic"; }
        </style>
    }
}

#[azumi::page(route = "/lesson-11")]
#[azumi::component]
pub fn page() -> impl Component {
    let state = UserLoader {
        loading: false,
        error: None,
        users: Vec::new(),
    };

    html! {
        @DarkModernLayout() {
            @user_loader_view(state=&state)
        }
    }
}

pub async fn lesson11_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
