use crate::examples::blog::data::get_posts;
use crate::examples::blog::PostLikes;
use crate::actions::contact::{ContactForm, client_validate};
use azumi::prelude::*;

/// Like/unlike a blog post
#[azumi::action]
pub async fn blog_like(
    state: axum::extract::State<azumi::live::LiveState>,
    form: azumi::action::Form<PostLikes>,
) -> impl Component {
    let post_id: usize = form.data.post_id.parse().unwrap_or(0);
    let post_likes = state.get::<PostLikes>();

    if post_id > 0 && post_id <= post_likes.counts.len() {
        post_likes.counts[post_id - 1] += 1;
    }

    html! {
        <span style="color: #ff4081;">{post_likes.counts[post_id.saturating_sub(1)]}" likes"</span>
    }
}

/// Submit contact form
#[azumi::action]
pub async fn contact_submit_action(
    form: azumi::action::Form<ContactForm>,
) -> azumi::action::ActionResult {
    let errors = client_validate(&form.data);

    if !errors.is_empty() {
        let mut validator = azumi::form::FormValidator::<ContactForm>::new();
        for (field, error) in &errors {
            validator.add_error(*field, error.to_string());
        }
        validator.set_data(form.data.clone());
        validator.mark_submitted();

        return error_fragment(validator);
    }

    // In a real app, you'd send an email or save to a database here
    let mut validator = azumi::form::FormValidator::<ContactForm>::new();
    validator.set_data(form.data.clone());
    validator.mark_submitted();

    success_fragment(validator)
}