//! Example: Animated Spinner component.
//! Copy this into your project's components/ directory.
//! Usage: html! { <button>{Spinner()} " Loading..."</button> }

use azumi::prelude::*;

/// A CSS-only animated spinner. Zero JavaScript.
/// Uses the `.az-spinner` class — style with `.az-spinner { width: 1em; }` for sizing.
#[azumi::component]
pub fn Spinner() -> impl Component {
    html! {
        <span class:external="az-spinner" role="status" aria-label="Loading">
            <style>
                .az-spinner {
                    display: "inline-block";
                    width: "1em";
                    height: "1em";
                    border: "2px solid currentColor";
                    border-right-color: "transparent";
                    border-radius: "50%";
                    animation: "az-spin 0.6s linear infinite";
                }
                @keyframes az-spin {
                    to { transform: "rotate(360deg)"; }
                }
            </style>
        </span>
    }
}
