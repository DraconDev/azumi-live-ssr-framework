use crate::examples::blog::data::{get_posts, increment_likes, BlogPost};
use azumi::prelude::*;

/// Handles blog/contact form submissions
#[azumi::action]
pub async fn contact_action(
    name: String,
    email: String,
    message: String,
) -> ActionResult {
    let mut errors = FormValidator::new();

    if name.trim().is_empty() {
        errors.field("name", "Name is required");
    }
    if email.trim().is_empty() {
        errors.field("email", "Email is required");
    } else if !email.contains('@') {
        errors.field("email", "Please enter a valid email address");
    }
    if message.trim().is_empty() {
        errors.field("message", "Message is required");
    } else if message.trim().len() < 10 {
        errors.field("message", "Message must be at least 10 characters");
    }

    if errors.has_errors() {
        return error_fragment(errors.html());
    }

    // In production: send email, save to DB, etc.
    tracing::info!(
        "Contact form: {} <{}> said: {}",
        name,
        email,
        message
    );

    success_fragment(html! {
        <div style="background: #e8f5e9; color: #2e7d32; padding: 1rem; border-radius: 4px; margin-bottom: 1rem;">
            <strong>"Thanks, {name}!"</strong>" Your message has been sent. We'll get back to you at "{&email}
        </div>
    })
}

/// Handles blog/post like increments
#[azumi::action]
pub async fn like_post(slug: String) -> ActionResult {
    let posts = get_posts();
    let post = posts.iter().find(|p| p.slug == slug);

    match post {
        Some(p) => {
            let new_count = increment_likes(&p.slug);
            success_fragment(html! {
                <span style="color: #888; font-size: 0.875rem;">{new_count} " likes"</span>
            })
        }
        None => {
            error_fragment(html! {
                <span style="color: #d32f2f; font-size: 0.875rem;">"Post not found"</span>
            })
        }
    }
}