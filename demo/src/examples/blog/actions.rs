use crate::examples::blog::data::{get_posts, increment_likes};
use axum::extract::Form;
use azumi::prelude::*;

/// Contact form data
#[derive(serde::Deserialize)]
pub struct ContactForm {
    name: String,
    email: String,
    message: String,
}

/// Handles blog/contact form submissions
#[azumi::action]
pub async fn contact_action(Form(form): Form<ContactForm>) -> ActionResult {
    let mut errors = Vec::<(&str, &str)>::new();

    if form.name.trim().is_empty() {
        errors.push(("name", "Name is required"));
    }
    if form.email.trim().is_empty() {
        errors.push(("email", "Email is required"));
    } else if !form.email.contains('@') {
        errors.push(("email", "Please enter a valid email address"));
    }
    if form.message.trim().is_empty() {
        errors.push(("message", "Message is required"));
    } else if form.message.trim().len() < 10 {
        errors.push(("message", "Message must be at least 10 characters"));
    }

    if !errors.is_empty() {
        let error_html = errors.iter().map(|(field, msg)| {
            format!(r#"<p data-error="{}" style="color: #d32f2f; font-size: 0.875rem; margin-top: 0.25rem;">{}</p>"#, field, msg)
        }).collect::<Vec<_>>().join("");
        return ActionResult::err(error_html);
    }

    // In production: send email, save to DB, etc.
    eprintln!(
        "Contact form: {} <{}> said: {}",
        form.name,
        form.email,
        form.message
    );

    let success_style = "background: #e8f5e9; color: #2e7d32; padding: 1rem; border-radius: 4px; margin-bottom: 1rem;";
    let component = html! {
        <div style={success_style}>
            <strong>{format!("Thanks, {}!", form.name)}</strong>" Your message has been sent."
        </div>
    };

    ActionResult::ok(&component)
}

/// Handles blog/post like increments
#[azumi::action]
pub async fn like_post(slug: String) -> ActionResult {
    let posts = get_posts();
    let post = posts.iter().find(|p| p.slug == slug);

    match post {
        Some(p) => {
            let new_count = increment_likes(&p.slug);
            let like_style = "color: #888; font-size: 0.875rem;";
            let component = html! {
                <span style={like_style}>{new_count} " likes"</span>
            };
            ActionResult::ok(&component)
        }
        None => {
            ActionResult::err("Post not found".to_string())
        }
    }
}