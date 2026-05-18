use crate::examples::lessons::components::lesson_nav::LessonNav;
use crate::examples::lessons::components::lesson_nav::lesson_prev_next;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use azumi::prelude::*;

use super::super::components::auth_infra::CurrentUser;

// -----------------------------------------------------------------------------
// 1. LIVE STATE
// -----------------------------------------------------------------------------
#[azumi::live]
pub struct AuthState {
    pub username: Option<String>,
}

#[azumi::live_impl(component = "lesson19_page")]
impl AuthState {
    pub fn logout(&mut self) {
        self.username = None;
    }
}

// -----------------------------------------------------------------------------
// 2. COMPONENT
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// 2. COMPONENT
// -----------------------------------------------------------------------------
#[azumi::component]
pub fn lesson19_page<'a>(state: &'a AuthState) -> impl Component + 'a {
    html! {
        @crate::examples::lessons::components::layout::DarkModernLayout() {
            <div class={container}>
                <div class={card}>
                    <div class={header}>
                        <h1 class={title}>"Lesson 19: Authentication"</h1>
                        <p class={subtitle}>"Simplified with Reusable Extractors"</p>
                    </div>

                    <div class={status_box}>
                        @if let Some(user) = &state.username {
                            <div class={user_state}>
                                <h3 class={welcome_msg}>"Welcome back, " <span class={username}>{user}</span> "!"</h3>
                                <p class={status_desc}>"Authenticated via shared middleware."</p>
                                <button class={logout_btn} on:click={state.logout}>"Mock Logout"</button>
                            </div>
                        } else {
                            <div class={guest_state}>
                                <h3 class={guest_msg}>"You are Guest"</h3>
                                <p class={status_desc}>"No session found."</p>
                                <a href="/lesson-19-login" class={login_btn}>"Simulate Login"</a>
                            </div>
                        }
                    </div>

                    <div class={explanation}>
                        <h4 class={exp_title}>"How it works:"</h4>
                        <ol class={exp_list}>
                            <li class={exp_item}>"Middleware validates cookies and sets"<code class={code}>"User"</code>"extension."</li>
                            <li class={exp_item}>"Handler uses"<code class={code}>"CurrentUser"</code>"extractor (zero boilerplate)."</li>
                            <li class={exp_item}>"State is initialized with user data."</li>
                        </ol>
                    </div>
                </div>
                @LessonNav(
                    prev_num=Some(18),
                    next_num=Some(20),
                    prev_title="Security",
                    next_title="Custom Inputs",
                )
            </div>
            <style>
                 .container { max-width: "600px"; margin: "0 auto"; }
                 .card {
                    background: "rgba(30, 41, 59, 0.6)";
                    backdrop-filter: "blur(12px)";
                    border-radius: "16px";
                    box-shadow: "0 10px 25px -5px rgba(0,0,0,0.3)";
                    padding: "2.5rem";
                    border: "1px solid rgba(255,255,255,0.05)";
                    margin-bottom: "2rem";
                }
                 .header { text-align: "center"; margin-bottom: "2.5rem"; }
                 .title {
                    font-size: "2.25rem";
                    margin: "0 0 0.5rem 0";
                    background: "linear-gradient(to right, #60a5fa, #3b82f6)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    font-weight: "800";
                }
                 .subtitle { color: "#94a3b8"; font-size: "1.1rem"; }

                 .status_box {
                    background: "rgba(15, 23, 42, 0.5)";
                    padding: "2rem";
                    border-radius: "12px";
                    text-align: "center";
                    border: "1px solid rgba(255,255,255,0.05)";
                    margin-bottom: "2rem";
                }
                 .user_state { animation: "fadeIn 0.5s ease"; }
                 .welcome_msg { color: "#e2e8f0"; font-size: "1.5rem"; margin-bottom: "0.5rem"; }
                 .username { color: "#60a5fa"; font-weight: "bold"; }
                 .status_desc { color: "#94a3b8"; margin-bottom: "1.5rem"; }

                 .logout_btn {
                    background: "rgba(239, 68, 68, 0.2)";
                    color: "#fca5a5";
                    border: "1px solid rgba(239, 68, 68, 0.3)";
                    padding: "0.75rem 1.5rem";
                    border-radius: "8px";
                    font-weight: "600";
                    cursor: "pointer";
                    font-size: "1rem";
                    transition: "all 0.2s";
                }
                 .logout_btn:hover { background: "rgba(239, 68, 68, 0.3)"; color: "#fecaca"; transform: "translateY(-1px)"; }

                 .guest_state { animation: "fadeIn 0.5s ease"; }
                 .guest_msg { color: "#cbd5e1"; font-size: "1.5rem"; margin-bottom: "0.5rem"; }

                 .login_btn {
                    background: "linear-gradient(to right, #3b82f6, #2563eb)";
                    color: "white";
                    border: "none";
                    padding: "0.75rem 2rem";
                    border-radius: "8px";
                    font-weight: "600";
                    cursor: "pointer";
                    text-decoration: "none";
                    display: "inline-block";
                    box-shadow: "0 4px 6px -1px rgba(37, 99, 235, 0.3)";
                    transition: "all 0.2s";
                }
                 .login_btn:hover { transform: "translateY(-2px)"; box-shadow: "0 10px 15px -3px rgba(37, 99, 235, 0.4)"; }

                 .explanation {
                    margin-top: "2rem";
                    padding-top: "2rem";
                    border-top: "1px solid rgba(255,255,255,0.05)";
                }
                 .exp_title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.1rem"; }
                 .exp_list { color: "#94a3b8"; padding-left: "1.2rem"; line-height: "1.6"; }
                 .exp_item { margin-bottom: "0.5rem"; }
                 .code {
                    font-family: "monospace";
                    color: "#60a5fa";
                    background: "rgba(0,0,0,0.3)";
                    padding: "0.1rem 0.3rem";
                    border-radius: "4px";
                    font-size: "0.9em";
                    margin: "0 0.3rem";
                }
                 @keyframes fadeIn { from { opacity: "0"; transform: "translateY(5px)"; } to { opacity: "1"; transform: "translateY(0)"; } }
            </style>
        }
    }
}

// -----------------------------------------------------------------------------
// 3. HANDLER
// -----------------------------------------------------------------------------

// Look how clean this is! No traits, no complex imports.
// We just ask for `CurrentUser` from our infrastructure.
pub async fn lesson19_handler(CurrentUser(user): CurrentUser) -> impl IntoResponse {
    let state = AuthState {
        username: user.map(|u| u.username),
    };

    use lesson19_page::*;
    let component = render(Props::builder().state(&state).build().unwrap());
    axum::response::Html(azumi::render_to_string(&component))
}

pub async fn login_handler(jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build(("azumi_user", "Dracon"))
        .path("/")
        .http_only(true)
        .build();
    (jar.add(cookie), Redirect::to("/lesson-19-auth"))
}
