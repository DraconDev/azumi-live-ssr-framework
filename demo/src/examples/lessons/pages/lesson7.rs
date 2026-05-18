use azumi::prelude::*;

use crate::examples::lessons::components::layout::DarkModernLayout;

/// Lesson 7: Form Handling with Validation
///
/// Form validation with compile-time checks
#[derive(Debug)]
#[allow(dead_code)]
struct UserForm {
    name: String,
    email: String,
    age: i32,
}

#[azumi::component]
pub fn user_form_component() -> impl Component {
    html! {

        <form class={form}>
            <div class={form_field}>
                <label class={form_label} for="name">"Name"</label>
                <input class={form_input} type="text" name="name" required />
            </div>
            <div class={form_field}>
                <label class={form_label} for="email">"Email"</label>
                <input class={form_input} type="email" name="email" required />
            </div>
            <div class={form_field}>
                <label class={form_label} for="age">"Age"</label>
                <input class={form_input} type="number" name="age" min="18" max="120" />
            </div>
            <button class={form_button} type="submit">"Submit"</button>
        </form>
        <style>
            .form { display: "grid"; gap: "1.5rem"; max-width: "400px"; }
            .form_field { display: "grid"; gap: "0.5rem"; }
            .form_label { font-weight: "600"; color: "#cbd5e1"; font-size: "0.9rem"; }
            .form_input {
                padding: "0.75rem";
                border: "1px solid rgba(255,255,255,0.1)";
                border-radius: "8px";
                background: "rgba(15, 23, 42, 0.6)";
                color: "white";
                font-family: "inherit";
            }
            .form_button {
                padding: "0.75rem";
                background: "linear-gradient(to right, #3b82f6, #2563eb)";
                color: "white";
                border: "none";
                cursor: "pointer";
                border-radius: "8px";
                font-weight: "600";
                transition: "opacity 0.2s";
            }
        </style>
    }
}

/// Example: Validation feedback
#[azumi::component]
pub fn validation_example() -> impl Component {
    html! {

        <div class={validation_container}>
            <h3 class={title}>"Form Validation"</h3>

            <p>"Azumi provides compile-time validation for:"</p>
            <ul class={list}>
                <li>"Required fields"</li>
                <li>"Proper input types"</li>
                <li>"Valid attribute values"</li>
                <li>"Accessible form structure"</li>
            </ul>

            <div class={status_container}>
                <p class={success_message}>"✅ Valid form structure"</p>
                <p class={error_message}>"❌ Invalid patterns caught at compile time"</p>
            </div>
        </div>
        <style>
            .validation_container { padding: "1.5rem"; color: "#cbd5e1"; }
            .title { color: "#e2e8f0"; margin-bottom: "1rem"; font-size: "1.25rem"; }
            .list { margin-left: "1.5rem"; margin-bottom: "1.5rem"; color: "#94a3b8"; }
            .status_container { background: "rgba(0,0,0,0.2)"; padding: "1rem"; border-radius: "8px"; }
            .error_message { color: "#f87171"; font-size: "0.9rem"; margin-top: "0.5rem"; font-weight: "500"; }
            .success_message { color: "#4ade80"; font-size: "0.9rem"; margin-top: "0.5rem"; font-weight: "500"; }
        </style>
    }
}

/// Example: Complex form with multiple fields
#[azumi::component]
pub fn complex_form_example() -> impl Component {
    html! {

        <form class={complex_form}>
            <div class={form_section}>
                <h3 class={section_title}>"Personal Information"</h3>
                <div class={form_grid}>
                    <label class={label} for="fullname">"Full Name"</label>
                    <input class={input} type="text" name="fullname" required />

                    <label class={label} for="birthdate">"Birth Date"</label>
                    <input class={input} type="date" name="birthdate" />

                    <label class={label} for="country">"Country"</label>
                    <select class={input} name="country">
                        <option value="us">"United States"</option>
                        <option value="uk">"United Kingdom"</option>
                        <option value="ca">"Canada"</option>
                    </select>
                </div>
            </div>

            <div class={form_section}>
                <h3 class={section_title}>"Preferences"</h3>
                <div class={form_grid}>
                    <label class={checkbox_label}>
                        <input type="checkbox" name="newsletter" />
                        " Subscribe to newsletter"
                    </label>

                    <label class={checkbox_label}>
                        <input type="checkbox" name="notifications" />
                        " Enable notifications"
                    </label>
                </div>
            </div>

            <button class={save_button} type="submit">"Save Preferences"</button>
        </form>
        <style>
            .complex_form { display: "grid"; gap: "1.5rem"; max-width: "500px"; }
            .form_section {
                border: "1px solid rgba(255,255,255,0.05)";
                padding: "1.5rem";
                border-radius: "12px";
                background: "rgba(15, 23, 42, 0.4)";
            }
            .section_title { font-weight: "600"; margin-bottom: "1rem"; color: "#e2e8f0"; font-size: "1.1rem"; }
            .form_grid { display: "grid"; gap: "0.75rem"; }
            .label { color: "#cbd5e1"; font-size: "0.9rem"; font-weight: "500"; }
            .input {
                padding: "0.6rem";
                background: "rgba(0, 0, 0, 0.2)";
                border: "1px solid rgba(255,255,255,0.1)";
                border-radius: "6px";
                color: "white";
                width: "100%";
            }
            .checkbox_label { color: "#cbd5e1"; display: "flex"; gap: "0.5rem"; align-items: "center"; cursor: "pointer"; }
            .save_button {
                padding: "0.75rem";
                background: "linear-gradient(to right, #10b981, #059669)";
                color: "white";
                border: "none";
                border-radius: "8px";
                font-weight: "600";
                cursor: "pointer";
            }
        </style>
    }
}

/// Main lesson demonstration component
#[azumi::page(route = "/lesson-7")]
#[azumi::component]
pub fn page() -> impl Component {
    html! {
        @DarkModernLayout() {
            <div class={container}>
                <header class={header}>
                    <h1 class={main_title}>"Lesson 7: Form Handling with Validation"</h1>
                    <p class={subtitle}>"Form validation with compile-time checks"</p>
                </header>

                <section class={key_points}>
                    <h2 class={section_title}>"Key Concepts"</h2>
                    <ul class={points_list}>
                        <li class={point}>"✅ Form validation at compile time"</li>
                        <li class={point}>"✅ Required field validation"</li>
                        <li class={point}>"✅ Input type validation"</li>
                        <li class={point}>"✅ Accessible form structure"</li>
                        <li class={point}>"✅ Type-safe form handling"</li>
                    </ul>
                </section>

                <section class={examples}>
                    <div class={example_card}>
                        @user_form_component()
                    </div>
                    <div class={example_card}>
                        @validation_example()
                    </div>
                    <div class={example_card}>
                        @complex_form_example()
                    </div>
                </section>
            </div>
            <style>
                .container { max-width: "900px"; margin: "0 auto"; }
                .header { text-align: "center"; margin-bottom: "3rem"; }
                .main_title {
                    font-size: "3rem";
                    font-weight: "800";
                    background: "linear-gradient(to right, #3b82f6, #10b981)";
                    -webkit-background-clip: "text";
                    -webkit-text-fill-color: "transparent";
                    margin-bottom: "1rem";
                }
                .subtitle { font-size: "1.25rem"; color: "#94a3b8"; }

                .key_points {
                    background: "rgba(30, 41, 59, 0.5)";
                    padding: "2rem";
                    border-radius: "16px";
                    margin-bottom: "3rem";
                    border: "1px solid rgba(255,255,255,0.05)";
                    backdrop-filter: "blur(10px)";
                }
                .section_title {
                    font-size: "1.5rem";
                    color: "#f1f5f9";
                    margin-bottom: "1.5rem";
                    border-bottom: "1px solid rgba(255,255,255,0.1)";
                    padding-bottom: "0.5rem";
                }
                .points_list { list-style: "none"; padding: "0"; display: "grid"; gap: "1rem"; }
                .point {
                    color: "#e2e8f0";
                    padding: "0.75rem";
                    background: "rgba(255,255,255,0.03)";
                    border-radius: "8px";
                    font-size: "1.1rem";
                }

                .examples { display: "grid"; gap: "2rem"; }
                .example_card {
                    border: "1px solid rgba(255,255,255,0.1)";
                    padding: "2rem";
                    border-radius: "16px";
                    background: "rgba(15, 23, 42, 0.6)";
                }
            </style>
        }
    }
}

// Handler for Axum
pub async fn lesson7_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&page()))
}
