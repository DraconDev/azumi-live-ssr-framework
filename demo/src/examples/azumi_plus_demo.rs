use crate::actions::{like_button, LikeState};
use azumi::prelude::*;

pub fn azumi_plus_demo() -> impl Component {
    html! {
        {azumi_script()}

        <div class={demo_container}>
            <h1>"Azumi+ Demo"</h1>

            // Client-Side UI State (az-ui)
            <div class={demo_container} az-ui="{ \"count\": 0 }">
                <h2>"Client-Side Counter"</h2>
                <p>
                    "Count: " <span az-bind:text="count">"0"</span>
                </p>
                <button class={btn} az-on="click set count = count + 1">
                    "Increment"
                </button>
            </div>

            <hr />

            // Server Action - pure fragment, styles defined above
            {like_button(LikeState { liked: false, count: 10 })}
        </div>
        <style>
        .demo_container {
            padding: "2rem";
            border: "1px solid #ccc";
            border-radius: "8px";
            max-width: "600px";
            margin: "0 auto";
        }
        .btn {
            padding: "0.5rem 1rem";
            border: "none";
            border-radius: "4px";
            cursor: "pointer";
            background: "#eee";
        }
        .btn.liked {
            background: "#ff4081";
            color: "white";
        }
        #likebox {
            margin-top: "2rem";
        }
        </style>
    }
}

pub async fn azumi_plus_demo_handler() -> axum::response::Html<String> {
    axum::response::Html(azumi::render_to_string(&azumi_plus_demo()))
}
