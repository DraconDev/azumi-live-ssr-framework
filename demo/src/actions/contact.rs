use azumi::prelude::*;
use azumi::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ContactResponse {
    pub success: bool,
    pub message: String,
}

#[azumi::component]
pub fn contact_submit(state: &mut Option<azumi::form::FormValidator<ContactForm>>) -> impl Component {
    let submitted = state.as_ref().map(|s| s.submitted()).unwrap_or(false);
    let success = submitted && state.as_ref().map(|s| s.is_valid()).unwrap_or(false);

    html! {
        @if success {
            <div class={form_success}>
                {"Thanks for reaching out! We will get back to you soon."}
            </div>
        }

        <form
            method="post"
            action="/blog/contact/submit"
            az-on="submit preventDefault action /blog/contact/submit"
        >
            <div class={form_group}>
                <label class={form_label} for="name">"Name"</label>
                <input
                    type="text"
                    id="name"
                    name="name"
                    placeholder="Your name"
                />
                @if let Some(err) = state.as_ref().and_then(|s| s.field_error("name")) {
                    <p class={form_error}>{err}</p>
                }
            </div>

            <div class={form_group}>
                <label class={form_label} for="email">"Email"</label>
                <input
                    type="email"
                    id="email"
                    name="email"
                    placeholder="your@email.com"
                />
                @if let Some(err) = state.as_ref().and_then(|s| s.field_error("email")) {
                    <p class={form_error}>{err}</p>
                }
            </div>

            <div class={form_group}>
                <label class={form_label} for="message">"Message"</label>
                <textarea
                    id="message"
                    name="message"
                    placeholder="Your message..."
                ></textarea>
                @if let Some(err) = state.as_ref().and_then(|s| s.field_error("message")) {
                    <p class={form_error}>{err}</p>
                }
            </div>

            <button type="submit">"Send Message"</button>
        </form>
    }
}

// Client-side validation rules (mirrors server-side)
pub fn client_validate(data: &ContactForm) -> Vec<(&str, &str)> {
    let mut errors = Vec::new();

    if data.name.trim().is_empty() {
        errors.push(("name", "Name is required"));
    }
    if data.email.trim().is_empty() {
        errors.push(("email", "Email is required"));
    } else if !data.email.contains('@') {
        errors.push(("email", "Please enter a valid email address"));
    }
    if data.message.trim().is_empty() {
        errors.push(("message", "Message is required"));
    } else if data.message.trim().len() < 10 {
        errors.push(("message", "Message must be at least 10 characters"));
    }

    errors
}