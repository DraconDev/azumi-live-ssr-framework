//! Server-Sent Events (SSE) helpers for Azumi.
//!
//! Provides a lightweight wrapper around Axum's SSE support for
//! streaming HTML fragments or JSON to connected clients.
//!
//! # Example
//!
//! ```rust,ignore
//! use azumi::streaming::{sse, SseEvent};
//! use std::time::Duration;
//! use tokio::time::interval;
//!
//! async fn notifications() -> impl axum::response::IntoResponse {
//!     let stream = async_stream::stream! {
//!         let mut ticker = interval(Duration::from_secs(5));
//!         loop {
//!             ticker.tick().await;
//!             yield SseEvent::fragment(html! {
//!                 <div class="notification">"New message received"</div>
//!             });
//!         }
//!     };
//!     sse(stream)
//! }
//! ```

#[cfg(feature = "axum")]
use axum::response::{sse::Event, Sse};
#[cfg(feature = "axum")]
use futures::stream::Stream;
#[cfg(feature = "axum")]
use std::convert::Infallible;

use crate::Component;

/// A single SSE event.
///
/// Construct with `SseEvent::fragment(component)` for HTML fragments
/// or `SseEvent::json(data)` for JSON payloads.
pub struct SseEvent {
    event: String,
    data: String,
    id: Option<String>,
}

impl SseEvent {
    /// Create an event carrying an HTML fragment.
    ///
    /// The fragment will be swapped into the target element on the client
    /// when used with `az-target`.
    pub fn fragment(component: impl Component) -> Self {
        Self {
            event: "fragment".to_string(),
            data: crate::render_to_string(&component),
            id: None,
        }
    }

    /// Create an event carrying raw JSON data.
    pub fn json(data: impl Into<String>) -> Self {
        Self {
            event: "json".to_string(),
            data: data.into(),
            id: None,
        }
    }

    /// Create a heartbeat event (keeps connection alive).
    pub fn heartbeat() -> Self {
        Self {
            event: "ping".to_string(),
            data: String::new(),
            id: None,
        }
    }

    /// Set the event ID for replay/resume support.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set a custom event name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.event = name.into();
        self
    }

    #[cfg(feature = "axum")]
    fn into_axum_event(self) -> Event {
        let mut event = Event::default().event(self.event).data(self.data);
        if let Some(id) = self.id {
            event = event.id(id);
        }
        event
    }
}

/// Convert a stream of `SseEvent` into an Axum SSE response.
///
/// Requires the `axum` feature (enabled by default).
#[cfg(feature = "axum")]
pub fn sse<S>(stream: S) -> impl axum::response::IntoResponse
where
    S: Stream<Item = SseEvent> + Send + 'static,
{
    let mapped = futures::stream::StreamExt::map(stream, |e| Ok::<_, Infallible>(e.into_axum_event()));
    Sse::new(mapped).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("ping"),
    )
}
