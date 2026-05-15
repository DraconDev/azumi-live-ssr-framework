//! Form validation helpers for Azumi.
//!
//! Provides lightweight, server-side form validation with automatic
//! error rendering and ARIA accessibility attributes.
//!
//! # Example
//!
//! ```rust,ignore
//! use azumi::form::{FormValidator, ValidatedForm, ValidationErrors};
//!
//! let mut validator = FormValidator::new();
//! validator.field("email", &form.email)
//!     .required()
//!     .email();
//! validator.field("password", &form.password)
//!     .required()
//!     .min_length(8);
//!
//! if let Some(errors) = validator.validate() {
//!     // Render form with errors
//!     return html! {
//!         <form az-action="/register" az-target="#form">
//!             {ValidatedForm::input("email", &form.email, errors.get("email"))}
//!             {ValidatedForm::password("password", &form.password, errors.get("password"))}
//!         </form>
//!     };
//! }
//! ```

use crate::{Component, from_fn};

/// A single validation error for a form field.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

/// Collection of validation errors, keyed by field name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidationErrors {
    errors: Vec<FieldError>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.errors.push(FieldError {
            field: field.into(),
            message: message.into(),
        });
    }

    pub fn get(&self, field: &str) -> Option<&str> {
        self.errors
            .iter()
            .find(|e| e.field == field)
            .map(|e| e.message.as_str())
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_field(&self, field: &str) -> bool {
        self.errors.iter().any(|e| e.field == field)
    }

    pub fn iter(&self) -> impl Iterator<Item = &FieldError> {
        self.errors.iter()
    }
}

/// Builder for validating a single field.
pub struct FieldValidator<'a> {
    name: &'a str,
    value: &'a str,
    errors: &'a mut ValidationErrors,
}

impl<'a> FieldValidator<'a> {
    pub fn required(self) -> Self {
        if self.value.trim().is_empty() {
            self.errors.add(self.name, "This field is required");
        }
        self
    }

    pub fn min_length(self, min: usize) -> Self {
        if self.value.chars().count() < min {
            self.errors.add(
                self.name,
                format!("Must be at least {} characters", min),
            );
        }
        self
    }

    pub fn max_length(self, max: usize) -> Self {
        if self.value.chars().count() > max {
            self.errors.add(
                self.name,
                format!("Must be at most {} characters", max),
            );
        }
        self
    }

    pub fn email(self) -> Self {
        if self.value.is_empty() {
            return self;
        }
        let is_valid = self.value.contains('@')
            && !self.value.starts_with('@')
            && self.value.split('@').nth(1).is_some_and(|domain| domain.contains('.'));
        if !is_valid {
            self.errors.add(self.name, "Please enter a valid email address");
        }
        self
    }

    pub fn url(self) -> Self {
        if self.value.is_empty() {
            return self;
        }
        let after_scheme = if self.value.starts_with("http://") {
            &self.value[7..]
        } else if self.value.starts_with("https://") {
            &self.value[8..]
        } else {
            ""
        };
        let is_valid = !after_scheme.is_empty() && !after_scheme.starts_with('@');
        if !is_valid {
            self.errors.add(self.name, "Please enter a valid URL");
        }
        self
    }

    pub fn custom<F>(self, check: F, message: impl Into<String>) -> Self
    where
        F: FnOnce(&str) -> bool,
    {
        if !self.value.is_empty() && !check(self.value) {
            self.errors.add(self.name, message);
        }
        self
    }
}

/// Form validation builder.
///
/// Chain `.field()` calls to validate each field, then call `.validate()`
/// to get errors (or `None` if all valid).
#[derive(Default)]
pub struct FormValidator {
    errors: ValidationErrors,
}

impl FormValidator {
    pub fn new() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }

    pub fn field<'a>(&'a mut self, name: &'a str, value: &'a str) -> FieldValidator<'a> {
        FieldValidator {
            name,
            value,
            errors: &mut self.errors,
        }
    }

    /// Returns Some(errors) if any field failed validation.
    /// Returns None if all fields are valid.
    pub fn validate(self) -> Option<ValidationErrors> {
        if self.errors.is_empty() {
            None
        } else {
            Some(self.errors)
        }
    }

    /// Like validate() but returns the errors struct directly (empty if valid).
    pub fn finish(self) -> ValidationErrors {
        self.errors
    }
}

fn html_escape(s: &str) -> String {
    crate::escape_html(s)
}

/// Pre-built form components with validation integration.
///
/// These are helper functions for common form patterns. They're unstyled
/// (no CSS classes) so you can wrap them in your own styled components.
pub struct ValidatedForm;

impl ValidatedForm {
    fn render_input(
        input_type: &str,
        name: &str,
        value: &str,
        error: Option<&str>,
    ) -> impl Component {
        let input_type = input_type.to_string();
        let has_error = error.is_some();
        let name = name.to_string();
        let value = value.to_string();
        let error = error.map(|s| s.to_string());
        from_fn(move |w| {
            write!(w, "<div>")?;
            write!(
                w,
                "<input type=\"{}\" name=\"{}\" value=\"{}\"",
                html_escape(&input_type),
                html_escape(&name),
                html_escape(&value)
            )?;
            if has_error {
                write!(
                    w,
                    " aria-invalid=\"true\" aria-describedby=\"{}_error\"",
                    html_escape(&name)
                )?;
            }
            write!(w, "/>")?;
            if let Some(ref msg) = error {
                write!(
                    w,
                    "<div id=\"{}_error\" class=\"field_error\" role=\"alert\">{}</div>",
                    html_escape(&name),
                    html_escape(msg)
                )?;
            }
            write!(w, "</div>")?;
            Ok(())
        })
    }

    /// Render a text input with optional validation error.
    pub fn input(name: &str, value: &str, error: Option<&str>) -> impl Component {
        Self::render_input("text", name, value, error)
    }

    /// Render an email input with optional validation error.
    pub fn email(name: &str, value: &str, error: Option<&str>) -> impl Component {
        Self::render_input("email", name, value, error)
    }

    /// Render a password input with optional validation error.
    pub fn password(name: &str, value: &str, error: Option<&str>) -> impl Component {
        Self::render_input("password", name, value, error)
    }

    /// Render a textarea with optional validation error.
    pub fn textarea(name: &str, value: &str, error: Option<&str>) -> impl Component {
        let has_error = error.is_some();
        let name = name.to_string();
        let value = value.to_string();
        let error = error.map(|s| s.to_string());
        from_fn(move |w| {
            write!(w, "<div>")?;
            write!(w, "<textarea name=\"{}\"", html_escape(&name))?;
            if has_error {
                write!(w, " aria-invalid=\"true\" aria-describedby=\"{}_error\"", html_escape(&name))?;
            }
            write!(w, ">{}</textarea>", html_escape(&value))?;
            if let Some(ref msg) = error {
                write!(w, "<div id=\"{}_error\" class=\"field_error\" role=\"alert\">{}</div>", html_escape(&name), html_escape(msg))?;
            }
            write!(w, "</div>")?;
            Ok(())
        })
    }

    /// Render a select dropdown with options and optional error.
    pub fn select(
        name: &str,
        value: &str,
        options: &[(&str, &str)],
        error: Option<&str>,
    ) -> impl Component {
        let has_error = error.is_some();
        let name = name.to_string();
        let value = value.to_string();
        let options: Vec<(String, String)> = options
            .iter()
            .map(|(v, l)| (v.to_string(), l.to_string()))
            .collect();
        let error = error.map(|s| s.to_string());
        from_fn(move |w| {
            write!(w, "<div>")?;
            write!(w, "<select name=\"{}\"", html_escape(&name))?;
            if has_error {
                write!(w, " aria-invalid=\"true\" aria-describedby=\"{}_error\"", html_escape(&name))?;
            }
            write!(w, ">")?;
            for (opt_value, opt_label) in &options {
                let selected = if *opt_value == value { " selected" } else { "" };
                write!(w, "<option value=\"{}\"{}>{}</option>", html_escape(opt_value), selected, html_escape(opt_label))?;
            }
            write!(w, "</select>")?;
            if let Some(ref msg) = error {
                write!(w, "<div id=\"{}_error\" class=\"field_error\" role=\"alert\">{}</div>", html_escape(&name), html_escape(msg))?;
            }
            write!(w, "</div>")?;
            Ok(())
        })
    }

    /// Render a summary of all validation errors at the top of a form.
    pub fn error_summary(errors: &ValidationErrors) -> impl Component {
        let items: Vec<(String, String)> = if errors.is_empty() {
            Vec::new()
        } else {
            errors
                .iter()
                .map(|e| (e.field.clone(), e.message.clone()))
                .collect()
        };
        from_fn(move |w| {
            if items.is_empty() {
                return Ok(());
            }
            write!(w, "<div class=\"form_errors\" role=\"alert\">")?;
            write!(w, "<p>Please fix the following errors:</p>")?;
            write!(w, "<ul>")?;
            for (field, message) in &items {
                write!(w, "<li><a href=\"#{}_error\">{}: {}</a></li>", html_escape(field), html_escape(field), html_escape(message))?;
            }
            write!(w, "</ul></div>")?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_field() {
        let mut v = FormValidator::new();
        v.field("name", "").required();
        let errors = v.finish();
        assert!(errors.has_field("name"));
        assert_eq!(errors.get("name"), Some("This field is required"));
    }

    #[test]
    fn test_min_length() {
        let mut v = FormValidator::new();
        v.field("pass", "123").min_length(8);
        assert!(v.finish().has_field("pass"));
    }

    #[test]
    fn test_email_validation() {
        let mut v = FormValidator::new();
        v.field("email", "not-an-email").email();
        assert!(v.finish().has_field("email"));

        let mut v2 = FormValidator::new();
        v2.field("email", "user@example.com").email();
        assert!(!v2.finish().has_field("email"));
    }

    #[test]
    fn test_validate_returns_none_when_valid() {
        let mut v = FormValidator::new();
        v.field("name", "Alice").required().min_length(2);
        assert!(v.validate().is_none());
    }

    #[test]
    fn test_validate_returns_some_when_invalid() {
        let mut v = FormValidator::new();
        v.field("name", "").required();
        assert!(v.validate().is_some());
    }

    #[test]
    fn test_multiple_fields() {
        let mut v = FormValidator::new();
        v.field("email", "").required().email();
        v.field("password", "123").required().min_length(8);
        let errors = v.finish();
        assert_eq!(errors.iter().count(), 2);
    }

    #[test]
    fn test_custom_validator() {
        let mut v = FormValidator::new();
        v.field("code", "abc").custom(|s| s == "xyz", "Code must be xyz");
        let errors = v.finish();
        assert_eq!(errors.get("code"), Some("Code must be xyz"));
    }

    #[test]
    fn test_url_short_strings_dont_panic() {
        for input in &["", "a", "ab", "abc", "://", "http:", "https:", "ftp://example.com"] {
            let mut v = FormValidator::new();
            v.field("url", input).url();
            let _ = v.finish();
        }
    }

    #[test]
    fn test_url_valid_https() {
        let mut v = FormValidator::new();
        v.field("url", "https://example.com").url();
        assert!(!v.finish().has_field("url"));
    }

    #[test]
    fn test_url_valid_http() {
        let mut v = FormValidator::new();
        v.field("url", "http://example.com/path").url();
        assert!(!v.finish().has_field("url"));
    }

    #[test]
    fn test_url_rejects_scheme_only() {
        let mut v = FormValidator::new();
        v.field("url", "https://").url();
        assert!(v.finish().has_field("url"));
    }

    #[test]
    fn test_url_rejects_at_sign_after_scheme() {
        let mut v = FormValidator::new();
        v.field("url", "https://@evil.com").url();
        assert!(v.finish().has_field("url"));
    }

    #[test]
    fn test_url_rejects_no_scheme() {
        let mut v = FormValidator::new();
        v.field("url", "example.com").url();
        assert!(v.finish().has_field("url"));
    }
}
