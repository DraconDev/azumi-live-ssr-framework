pub mod like;
pub mod contact;

// Re-export main components and types
pub use like::{like_button, LikeState};
// Blog contact form action (defined in examples/blog/actions.rs)
pub use crate::examples::blog::actions::contact_action;
