use azumi::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct LikeState {
    pub liked: bool,
    pub count: i32,
}

/// Dynamic fragment - this is what gets swapped by actions
pub fn like_button(state: LikeState) -> impl Component {
    html! {

        <div id={like_box} az-scope={serde_json::to_string(&state).unwrap_or_default()}>
            <h2>"Server-Side Action"</h2>
            <p>
                "Likes: " <span az-bind:text="count">{state.count}</span>
            </p>

            <button
                class={btn}
                az-bind:class.liked="liked"
                az-on={click call toggle_like -> #like_box}
            >
                <span az-bind:text="liked ? 'Unlike' : 'Like'">
                    {if state.liked { "Unlike" } else { "Like" }}
                </span>
            </button>
        </div>

        <style>
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
            #like_box {}
        </style>
    }
}

/// Server-side action - returns ONLY the fragment
#[azumi::action]
pub async fn toggle_like(state: LikeState) -> impl Component {
    let new_state = LikeState {
        liked: !state.liked,
        count: if !state.liked {
            state.count + 1
        } else {
            state.count - 1
        },
    };

    // Return the same fragment, styles are already on the page!
    like_button(new_state)
}
