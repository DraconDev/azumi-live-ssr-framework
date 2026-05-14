//! Azumi Testing Harness
//!
//! Utilities for testing components and live actions without a browser.

use crate::Component;
use scraper::{Html, Selector};

/// Renders a component to a string for testing.
///
/// # Example
///
/// ```rust,ignore
/// use azumi::test;
/// let component = azumi::html! { <div>"Hello"</div> };
/// let html = test::render(&component);
/// assert!(html.contains("Hello"));
/// ```
pub fn render(component: &impl Component) -> String {
    crate::render_to_string(component)
}

/// Renders a component to a `Vec<u8>` via `render_to_writer`.
///
/// Useful for testing that `render_to_writer` produces the same output
/// as `render_to_string`.
pub fn render_to_vec(component: &impl Component) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::render_to_writer(component, &mut buf).expect("render_to_writer should not fail");
    buf
}

/// Parses HTML and asserts that a selector exists and optionally matches text.
///
/// # Example
///
/// ```rust,ignore
/// use azumi::test;
/// let component = azumi::html! { <div class="title">"Welcome"</div> };
/// let html = test::render(&component);
/// test::assert_selector(&html, ".title", Some("Welcome"));
/// ```
pub fn assert_selector(html: &str, selector_str: &str, expected_text: Option<&str>) {
    let document = Html::parse_fragment(html);
    let selector = Selector::parse(selector_str).expect("Invalid CSS selector");

    let element = document.select(&selector).next();

    if element.is_none() {
        panic!(
            "Assertion failed: Selector '{}' not found in HTML.\nHTML output:\n{}",
            selector_str, html
        );
    }

    if let Some(expected) = expected_text {
        let element = element.unwrap();
        let actual_text = element.text().collect::<Vec<_>>().join("");
        let actual_trimmed = actual_text.trim();

        if actual_trimmed != expected {
            panic!(
                "Assertion failed: Selector '{}' text mismatch.\nExpected: '{}'\nActual:   '{}'",
                selector_str, expected, actual_trimmed
            );
        }
    }
}

/// Helper struct for testing Live Components
pub struct Simulator<T> {
    pub state: T,
}

impl<T> Simulator<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }

    /// Execute an action on the state
    pub fn act<F>(&mut self, action: F)
    where
        F: FnOnce(&mut T),
    {
        action(&mut self.state);
    }
}

/// Creates a simulator for a live state struct
pub fn simulate<T>(state: T) -> Simulator<T> {
    Simulator::new(state)
}
