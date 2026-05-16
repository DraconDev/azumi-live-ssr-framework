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

/// Asserts that a CSS selector does NOT match any element in the rendered HTML.
///
/// # Example
///
/// ```rust,ignore
/// let html = test::render(&component);
/// test::assert_not_selector(&html, ".error", "No error should be shown");
/// ```
pub fn assert_not_selector(html: &str, selector_str: &str, msg: &str) {
    let document = Html::parse_fragment(html);
    let selector = Selector::parse(selector_str).expect("Invalid CSS selector");

    if document.select(&selector).next().is_some() {
        panic!(
            "Assertion failed: Selector '{}' unexpectedly found in HTML.\nMessage: {}\nHTML output:\n{}",
            selector_str, msg, html
        );
    }
}

/// Asserts that the rendered HTML does NOT contain the specified text.
pub fn assert_no_text(html: &str, unexpected: &str, msg: &str) {
    if html.contains(unexpected) {
        panic!(
            "Assertion failed: Unexpected text '{}' found in HTML.\nMessage: {}\nHTML output:\n{}",
            unexpected, msg, html
        );
    }
}

pub fn simulate<T>(state: T) -> Simulator<T> {
    Simulator::new(state)
}

#[cfg(test)]
mod framework_tests {
    use super::*;
    use crate::{render_to_string, render_page, Component};

    struct HelloComponent;

    impl Component for HelloComponent {
        fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "<div>Hello world</div>")
        }
    }

    #[test]
    fn test_render_to_writer_matches_render_to_string() {
        let comp = HelloComponent;
        let from_string = render_to_string(&comp);
        let from_writer = render_to_vec(&comp);
        assert_eq!(
            from_string.as_bytes(),
            from_writer.as_slice(),
            "render_to_writer should produce identical output to render_to_string"
        );
    }

    #[tokio::test]
    async fn test_render_page_basic() {
        let comp = HelloComponent;
        let output = render_page(&comp).await;
        assert_eq!(
            output, "<div>Hello world</div>",
            "render_page should produce HTML output"
        );
    }

    #[tokio::test]
    async fn test_render_page_matches_render_to_string() {
        let comp = HelloComponent;
        let from_sync = render_to_string(&comp);
        let from_async = render_page(&comp).await;
        assert_eq!(
            from_sync, from_async,
            "render_page should produce same output as render_to_string"
        );
    }

    #[test]
    fn test_assert_not_selector_passes() {
        let html = "<div>Hello</div>";
        assert_not_selector(html, ".missing", "Should not find .missing");
    }

    #[test]
    #[should_panic(expected = "unexpectedly found")]
    fn test_assert_not_selector_fails() {
        let html = "<div class=\"found\">Hello</div>";
        assert_not_selector(html, ".found", "Should panic");
    }

    #[test]
    fn test_assert_no_text_passes() {
        let html = "<div>Hello</div>";
        assert_no_text(html, "error", "Should not contain error");
    }

    #[test]
    #[should_panic(expected = "Unexpected text")]
    fn test_assert_no_text_fails() {
        let html = "<div class=\"error\">Bad</div>";
        assert_no_text(html, "error", "Should panic");
    }
}
