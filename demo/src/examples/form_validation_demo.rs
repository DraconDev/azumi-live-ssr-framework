//! Demo: Signup form with bind:value + data-validate
//! Shows live two-way binding and client-side validation together.
//! Run with: cargo run --example form_validation_demo

use azumi::prelude::*;

/// Signup form with live binding and validation
#[azumi::component]
fn signup_form() -> impl Component {
    html! {
        <div class:external="signup-container">
            <h2>"Create Account"</h2>
            <p class:external="subtitle">"All fields required."</p>

            <form az-action={"signup"} az-target={"#signup-result"} class:external="signup-form">
                <div class:external="form-group">
                    <label class:external="form-label" for={"name"}>"Full Name"</label>
                    <input
                        type={"text"}
                        id={"name"}
                        name={"name"}
                        class:external="form-input"
                        placeholder={"Jane Smith"}
                        bind:value={""}
                        data-validate={"name:required,min-length:2"}
                    />
                    <p id={"name_error"} class:external="form-error" style={"display: none"}></p>
                </div>

                <div class:external="form-group">
                    <label class:external="form-label" for={"email"}>"Email address"</label>
                    <input
                        type={"email"}
                        id={"email"}
                        name={"email"}
                        class:external="form-input"
                        placeholder={"jane@example.com"}
                        bind:value={""}
                        data-validate={"email:required,email"}
                    />
                    <p id={"email_error"} class:external="form-error" style={"display: none"}></p>
                </div>

                <div class:external="form-group">
                    <label class:external="form-label" for={"password"}>"Password"</label>
                    <input
                        type={"password"}
                        id={"password"}
                        name={"password"}
                        class:external="form-input"
                        placeholder={"At least 8 characters"}
                        bind:value={""}
                        data-validate={"password:required,min-length:8"}
                    />
                    <p id={"password_error"} class:external="form-error" style={"display: none"}></p>
                </div>

                <div class:external="form-group form-group-checkbox">
                    <label class:external="form-label-checkbox">
                        <input
                            type={"checkbox"}
                            name={"agree"}
                            bind:checked={false}
                            data-validate={"agree:required"}
                        />
                        <span>"I agree to the Terms of Service"</span>
                    </label>
                    <p id={"agree_error"} class:external="form-error" style={"display: none"}></p>
                </div>

                <button type={"submit"} class:external="submit-btn">"Create Account"</button>
            </form>

            <div id={"signup-result"}></div>
        </div>

        <style>
            .signup-container {
                max-width: "28rem";
                margin: "2rem auto";
                padding: "2rem";
                background: "var(--bg-secondary, #f9fafb)";
                border-radius: "8px";
                border: "1px solid var(--border, #e5e7eb)";
            }
            .signup-container h2 {
                margin: "0 0 0.25rem";
                font-size: "1.5rem";
            }
            .subtitle {
                color: "var(--text-muted, #6b7280)";
                margin: "0 0 1.5rem";
                font-size: "0.875rem";
            }
            .signup-form {
                display: "flex";
                flex-direction: "column";
                gap: "1rem";
            }
            .form-group {
                display: "flex";
                flex-direction: "column";
                gap: "0.25rem";
            }
            .form-group-checkbox {
                flex-direction: "row";
                align-items: "center";
            }
            .form-label {
                font-size: "0.85rem";
                font-weight: "600";
                color: "var(--text-secondary, #374151)";
            }
            .form-label-checkbox {
                display: "flex";
                align-items: "center";
                gap: "0.5rem";
                font-size: "0.85rem";
                cursor: "pointer";
            }
            .form-input {
                padding: "0.75rem 1rem";
                border: "1px solid var(--border, #e5e7eb)";
                border-radius: "4px";
                font-size: "1rem";
                outline: "none";
                transition: "border-color 0.15s";
            }
            .form-input:focus {
                border-color: "var(--accent, #3b82f6)";
            }
            .form-error {
                color: "var(--danger, #ef4444)";
                font-size: "0.8rem";
                margin: "0";
            }
            .submit-btn {
                padding: "0.75rem";
                background: "var(--accent, #3b82f6)";
                color: "white";
                border: "none";
                border-radius: "4px";
                font-size: "1rem";
                font-weight: "600";
                cursor: "pointer";
                transition: "background 0.15s";
            }
            .submit-btn:hover {
                background: "var(--accent-hover, #2563eb)";
            }
            [aria-invalid="true"] {
                border-color: "var(--danger, #ef4444)";
            }
        </style>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azumi::render_to_string;

    #[test]
    fn signup_form_has_bind_value() {
        let html = render_to_string(&signup_form());
        assert!(html.contains("data-bind-value="), "Form should have bind:value attributes");
    }

    #[test]
    fn signup_form_has_data_validate() {
        let html = render_to_string(&signup_form());
        assert!(html.contains("data-validate="), "Form should have data-validate attributes");
    }

    #[test]
    fn signup_form_has_three_inputs_with_validation() {
        let html = render_to_string(&signup_form());
        let count = html.matches("data-validate=").count();
        assert_eq!(count, 4, "Should have 4 data-validate attributes (name, email, password, agree)");
    }

    #[test]
    fn signup_form_has_all_validation_rules() {
        let html = render_to_string(&signup_form());
        assert!(html.contains("required,min-length:2"), "Name should require min-length:2");
        assert!(html.contains("required,email"), "Email should require email format");
        assert!(html.contains("required,min-length:8"), "Password should require min-length:8");
        assert!(html.contains("required"), "Agreement checkbox should be required");
    }
}
