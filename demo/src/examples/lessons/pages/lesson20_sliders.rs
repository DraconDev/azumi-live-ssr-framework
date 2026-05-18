use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::prelude::*;

#[azumi::live]
pub struct SliderState {
    pub value: i32,
}

#[azumi::live_impl(component = "slider_view")]
impl SliderState {
    pub fn update_value(&mut self) {
        // For now, we'll just increment to show it's alive
        self.value = (self.value + 1) % 101;
    }
}

#[azumi::component]
pub fn slider_view<'a>(state: &'a SliderState) -> impl Component + 'a {
    html! {
        <div class={slider_container}>
            <div class={card}>
                <h2 class={title}>"Lesson 20: Custom Inputs"</h2>
                <p class={description}>"Interactive sliders with Azumi Live state."</p>

                <div class={range_wrapper}>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        value={state.value}
                        az-on:input="update_value"
                        class={range_input}
                    />
                    <div class={range_track}>
                        <div class={range_fill} style={format!("width: {}%", state.value)}></div>
                    </div>
                </div>

                <div class={value_display}>
                    <span class={value_number}>{state.value}</span>
                    <span class={value_label}>"/ 100"</span>
                </div>

                <button az-on:click="update_value" class={btn}>"Increment"</button>
            </div>
        </div>
        <style>
            .slider_container { padding: "2rem"; max-width: "600px"; margin: "0 auto"; }
            .card {
                background: "rgba(30, 41, 59, 0.5)";
                border: "1px solid rgba(255,255,255,0.1)";
                border-radius: "16px";
                padding: "2rem";
            }
            .title { color: "#f1f5f9"; font-size: "1.5rem"; margin-bottom: "0.5rem"; font-weight: "700"; }
            .description { color: "#94a3b8"; margin-bottom: "2rem"; font-size: "0.95rem"; line-height: "1.6"; }

            .range_wrapper { position: "relative"; margin-bottom: "1.5rem"; }
            .range_input {
                -webkit-appearance: "none";
                appearance: "none";
                width: "100%";
                height: "8px";
                background: "transparent";
                z-index: "2";
                margin: "0";
            }
            .range_track {
                position: "absolute";
                top: "50%";
                left: "0";
                right: "0";
                height: "8px";
                background: "rgba(255,255,255,0.1)";
                border-radius: "4px";
                transform: "translateY(-50%)";
                z-index: "1";
                overflow: "hidden";
            }
            .range_fill {
                height: "100%";
                background: "linear-gradient(to right, #6366f1, #8b5cf6)";
                border-radius: "4px";
                transition: "width 0.15s ease";
            }

            .value_display {
                display: "flex";
                align-items: "baseline";
                gap: "0.5rem";
                margin-bottom: "1.5rem";
            }
            .value_number { font-size: "3rem"; font-weight: "800"; color: "#a78bfa"; line-height: "1"; }
            .value_label { font-size: "1rem"; color: "#64748b"; }

            .btn {
                background: "#6366f1";
                color: "white";
                padding: "0.75rem 1.5rem";
                border-radius: "8px";
                font-weight: "600";
                cursor: "pointer";
                transition: "all 0.2s";
            }
            .btn:hover {
                background: "#4f46e5";
                transform: "translateY(-1px)";
            }
        </style>
    }
}

pub async fn lesson20_handler() -> impl axum::response::IntoResponse {
    let state = SliderState { value: 50 };
    use slider_view::Props;
    let slider = slider_view::render(Props::builder().state(&state).build().expect("props"));

    let full_page = html! {
        @DarkModernLayout() {
            {slider}
        }
    };

    axum::response::Html(azumi::render_to_string(&full_page))
}
