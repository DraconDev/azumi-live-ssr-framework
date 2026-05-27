//! Example: Accessible Confirm Dialog component.
//! Copy into your project. Use with az-confirm or standalone.
//!
//! Usage: html! { Confirm::render(Confirm::Props::builder()
//!   .message("Delete this item?").build().unwrap()) }

use azumi::prelude::*;

#[azumi::component]
pub fn Confirm(
    message: &str,
    #[prop(default = "\"Cancel\"")] cancel_label: &str,
    #[prop(default = "\"Confirm\"")] confirm_label: &str,
) -> impl Component {
    html! {
        <div class:external="az-confirm-overlay" role="alertdialog" aria-modal="true" aria-labelledby="az-confirm-msg">
            <div class:external="az-confirm-dialog">
                <p id="az-confirm-msg">{message}</p>
                <div class:external="az-confirm-actions">
                    <button class:external="az-confirm-cancel" autofocus>{cancel_label}</button>
                    <button class:external="az-confirm-confirm">{confirm_label}</button>
                </div>
            </div>
        </div>
        <style>
            .az-confirm-overlay {
                position: "fixed";
                inset: "0";
                background: "rgba(0, 0, 0, 0.5)";
                display: "flex";
                align-items: "center";
                justify-content: "center";
                z-index: "9999";
            }
            .az-confirm-dialog {
                background: "var(--bg-primary, white)";
                border-radius: "8px";
                padding: "1.5rem";
                max-width: "24rem";
                width: "90%";
                box-shadow: "0 20px 60px rgba(0, 0, 0, 0.3)";
            }
            .az-confirm-dialog p {
                margin: "0 0 1rem";
                font-size: "1rem";
            }
            .az-confirm-actions {
                display: "flex";
                gap: "0.75rem";
                justify-content: "flex-end";
            }
            .az-confirm-cancel {
                padding: "0.5rem 1rem";
                border: "1px solid var(--border, #e5e7eb)";
                border-radius: "4px";
                background: "transparent";
                cursor: "pointer";
            }
            .az-confirm-confirm {
                padding: "0.5rem 1rem";
                border: "none";
                border-radius: "4px";
                background: "var(--danger, #ef4444)";
                color: "white";
                cursor: "pointer";
            }
        </style>
    }
}
