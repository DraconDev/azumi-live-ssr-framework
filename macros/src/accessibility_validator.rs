use crate::token_parser::{AttributeValue, Element};
use proc_macro2::TokenStream;
use quote::quote_spanned;

/// Rule 1: Every <img> tag MUST have an alt attribute
pub fn validate_img_alt(elem: &Element) -> Option<TokenStream> {
    if elem.name != "img" {
        return None;
    }

    // Check if alt attribute exists
    let has_alt = elem.attrs.iter().any(|attr| attr.name == "alt");

    if !has_alt {
        Some(quote_spanned! { elem.span =>
            compile_error!("<img> is missing 'alt' attribute. Add alt=\"description\" or alt=\"\" for decorative images.");
        })
    } else {
        None
    }
}

/// Rule 2: The type attribute on <input> and <button> must be valid
pub fn validate_input_type(elem: &Element) -> Option<TokenStream> {
    // Only validate input and button elements
    if elem.name != "input" && elem.name != "button" {
        return None;
    }

    // Find type attribute
    let type_attr = elem.attrs.iter().find(|attr| attr.name == "type")?;

    // Only validate static values (not dynamic expressions)
    if let AttributeValue::Static(type_value) = &type_attr.value {
        let valid = match elem.name.as_str() {
            "input" => is_valid_input_type(type_value),
            "button" => is_valid_button_type(type_value),
            _ => return None,
        };

        if !valid {
            let suggestion = suggest_type_correction(type_value, elem.name == "input");
            let msg = format!(
                "Invalid <{}> type=\"{}\". {}",
                elem.name, type_value, suggestion
            );

            return Some(quote_spanned! { type_attr.span =>
                compile_error!(#msg);
            });
        }
    }

    None
}

/// Rule 3: ARIA role values must be valid
pub fn validate_aria_roles(elem: &Element) -> Option<TokenStream> {
    // Find role attribute
    let role_attr = elem.attrs.iter().find(|attr| attr.name == "role")?;

    // Only validate static values
    if let AttributeValue::Static(role_value) = &role_attr.value {
        if !is_valid_aria_role(role_value) {
            let msg = format!(
                "Invalid role=\"{}\". Must be a valid WAI-ARIA role. See: https://www.w3.org/TR/wai-aria-1.2/#role_definitions",
                role_value
            );

            return Some(quote_spanned! { role_attr.span =>
                compile_error!(#msg);
            });
        }
    }

    None
}

/// Rule 4: Buttons must have content OR aria-label OR title
pub fn validate_button_content(elem: &crate::token_parser::Element) -> Option<TokenStream> {
    if elem.name != "button" {
        return None;
    }

    // Check if button has text content (children)
    let has_content = !elem.children.is_empty();

    // Check if button has aria-label or title attribute
    let has_aria_label = elem.attrs.iter().any(|attr| attr.name == "aria-label");
    let has_title = elem.attrs.iter().any(|attr| attr.name == "title");

    if !has_content && !has_aria_label && !has_title {
        Some(quote_spanned! { elem.span =>
            compile_error!("<button> has no accessible text. Add text content, aria-label=\"...\", or title=\"...\".");
        })
    } else {
        None
    }
}

// Helper: Valid input types (HTML5)
fn is_valid_input_type(type_value: &str) -> bool {
    matches!(
        type_value,
        "text"
            | "password"
            | "email"
            | "number"
            | "tel"
            | "url"
            | "search"
            | "date"
            | "time"
            | "datetime-local"
            | "month"
            | "week"
            | "color"
            | "file"
            | "hidden"
            | "checkbox"
            | "radio"
            | "submit"
            | "reset"
            | "button"
            | "range"
            | "image"
    )
}

// Helper: Valid button types
fn is_valid_button_type(type_value: &str) -> bool {
    matches!(type_value, "button" | "submit" | "reset")
}

// Helper: Suggest corrections for common typos
fn suggest_type_correction(type_value: &str, is_input: bool) -> String {
    match type_value {
        "txt" => "Did you mean 'text'?".to_string(),
        "sumbit" => "Did you mean 'submit'?".to_string(),
        "buton" => "Did you mean 'button'?".to_string(),
        "num" => "Did you mean 'number'?".to_string(),
        "mail" => "Did you mean 'email'?".to_string(),
        _ => {
            if is_input {
                "Valid types: text, password, email, number, tel, url, search, date, time, checkbox, radio, submit, reset, button, etc.".to_string()
            } else {
                "Valid types: button, submit, reset".to_string()
            }
        }
    }
}

// Helper: Valid ARIA roles (WAI-ARIA 1.2)
fn is_valid_aria_role(role_value: &str) -> bool {
    matches!(
        role_value,
        // Widget roles
        "button" | "checkbox" | "gridcell" | "link" | "menuitem" | "menuitemcheckbox"
            | "menuitemradio" | "option" | "progressbar" | "radio" | "scrollbar"
            | "searchbox" | "slider" | "spinbutton" | "switch"
            | "tab" | "tabpanel" | "textbox" | "treeitem"
            // Composite widget roles
            | "combobox" | "grid" | "listbox" | "menu" | "menubar" | "radiogroup"
            | "tablist" | "tree" | "treegrid"
            // Document structure roles
            | "application" | "article" | "cell" | "columnheader" | "definition"
            | "directory" | "document" | "feed" | "figure" | "group" | "heading"
            | "img" | "list" | "listitem" | "math" | "none" | "note" | "presentation"
            | "row" | "rowgroup" | "rowheader" | "separator" | "table" | "term"
            | "toolbar" | "tooltip"
            // Landmark roles
            | "banner" | "complementary" | "contentinfo" | "form" | "main"
            | "navigation" | "region" | "search"
            // Live region roles
            | "alert" | "log" | "marquee" | "status" | "timer"
            // Window roles
            | "alertdialog" | "dialog"
    )
}

/// Rule 5: Links with target="_blank" must have rel="noopener" (security)
pub fn validate_anchor_target_blank(elem: &Element) -> Option<TokenStream> {
    if elem.name != "a" {
        return None;
    }

    let target_attr = elem.attrs.iter().find(|attr| attr.name == "target");

    if let Some(target) = target_attr {
        if let AttributeValue::Static(val) = &target.value {
            if val == "_blank" {
                // Check for rel attribute
                let rel_attr = elem.attrs.iter().find(|attr| attr.name == "rel");

                let has_noopener = if let Some(rel) = rel_attr {
                    if let AttributeValue::Static(rel_val) = &rel.value {
                        rel_val.contains("noopener") || rel_val.contains("noreferrer")
                    } else {
                        // Dynamic rel attribute - assume it's handled or warn?
                        // For now, let's be strict about static values.
                        false
                    }
                } else {
                    false
                };

                if !has_noopener {
                    return Some(quote_spanned! { target.span =>
                        compile_error!("Security Risk: Links with target=\"_blank\" must have rel=\"noopener\" or rel=\"noreferrer\" to prevent Reverse Tabnabbing attacks.");
                    });
                }
            }
        }
    }

    None
}

/// Rule 7: aria-* attribute values must be valid
/// Validates known aria attributes with enumerated values.
/// Unknown aria attributes are skipped (forward compatibility).
pub fn validate_aria_values(elem: &Element) -> Option<TokenStream> {
    for attr in &elem.attrs {
        if !attr.name.starts_with("aria-") {
            continue;
        }

        // Only validate static values
        let AttributeValue::Static(val) = &attr.value else {
            continue;
        };

        if let Some(msg) = validate_single_aria_attr(&attr.name, val) {
            return Some(quote_spanned! { attr.span =>
                compile_error!(#msg);
            });
        }
    }
    None
}

fn validate_single_aria_attr(name: &str, value: &str) -> Option<String> {
    match name {
        // Boolean/tristate attributes
        "aria-expanded" | "aria-pressed"
            if !matches!(value, "true" | "false" | "undefined") =>
        {
            Some(format!(
                "Invalid {}=\"{}\". Valid values: true, false, undefined",
                name, value
            ))
        }
        "aria-checked"
            if !matches!(value, "true" | "false" | "mixed" | "undefined") =>
        {
            Some(format!(
                "Invalid {}=\"{}\". Valid values: true, false, mixed, undefined",
                name, value
            ))
        }
        "aria-disabled" | "aria-readonly" | "aria-required" | "aria-hidden"
        | "aria-selected" | "aria-modal" | "aria-busy" | "aria-grabbed"
        | "aria-atomic" | "aria-multiline" | "aria-multiselectable"
        | "aria-orientation"
            if !matches!(value, "true" | "false") =>
        {
            Some(format!(
                "Invalid {}=\"{}\". Valid values: true, false",
                name, value
            ))
        }
        // Token list attributes
        "aria-relevant" => {
            for token in value.split_whitespace() {
                if !matches!(token, "additions" | "removals" | "text" | "all") {
                    return Some(format!(
                        "Invalid aria-relevant token \"{}\". Valid tokens: additions, removals, text, all",
                        token
                    ));
                }
            }
            None
        }
        "aria-live" if !matches!(value, "off" | "polite" | "assertive") => Some(format!(
            "Invalid aria-live=\"{}\". Valid values: off, polite, assertive",
            value
        )),
        "aria-dropeffect" => {
            for token in value.split_whitespace() {
                if !matches!(token, "copy" | "move" | "link" | "execute" | "popup" | "none") {
                    return Some(format!(
                        "Invalid aria-dropeffect token \"{}\". Valid tokens: copy, move, link, execute, popup, none",
                        token
                    ));
                }
            }
            None
        }
        "aria-haspopup"
            if !matches!(value, "false" | "true" | "menu" | "listbox" | "tree" | "grid" | "dialog") =>
        {
            Some(format!(
                "Invalid aria-haspopup=\"{}\". Valid values: false, true, menu, listbox, tree, grid, dialog",
                value
            ))
        }
        // Skip unknown aria-* attributes for forward compatibility
        _ => None,
    }
}

pub fn validate_iframe_title(
    elem: &crate::token_parser::Element,
) -> Option<proc_macro2::TokenStream> {
    if elem.name != "iframe" {
        return None;
    }

    let has_title = elem.attrs.iter().any(|attr| attr.name == "title");

    if !has_title {
        Some(quote_spanned! { elem.span =>
            compile_error!("<iframe> tags must have a 'title' attribute for accessibility.");
        })
    } else {
        None
    }
}

// Test helpers
#[cfg(test)]
fn test_element(name: &str) -> Element {
    use proc_macro2::Span;
    Element {
        name: name.to_string(),
        attrs: vec![],
        children: vec![],
        bind_struct: None,
        span: Span::call_site(),
        full_span: Span::call_site(),
    }
}

#[cfg(test)]
fn test_element_with_attrs(name: &str, attrs: Vec<(&str, &str)>) -> Element {
    use proc_macro2::Span;
    let mut elem = test_element(name);
    for (name, value) in attrs {
        elem.attrs.push(crate::token_parser::Attribute {
            name: name.to_string(),
            name_span: Span::call_site(),
            value: AttributeValue::Static(value.to_string()),
            span: Span::call_site(),
            value_span: None,
        });
    }
    elem
}

#[cfg(test)]
fn test_element_with_children(name: &str, children: Vec<crate::token_parser::Node>) -> Element {
    let mut elem = test_element(name);
    elem.children = children;
    elem
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // validate_img_alt
    // =========================================================================

    #[test]
    fn test_img_without_alt_fails() {
        let elem = test_element("img");
        let result = validate_img_alt(&elem);
        assert!(result.is_some(), "img without alt should fail validation");
        let msg = result.unwrap().to_string();
        assert!(msg.contains("missing 'alt'"), "Error should mention missing alt: {}", msg);
    }

    #[test]
    fn test_img_with_alt_passes() {
        let elem = test_element_with_attrs("img", vec![("alt", "Description")]);
        let result = validate_img_alt(&elem);
        assert!(result.is_none(), "img with alt should pass validation");
    }

    #[test]
    fn test_img_empty_alt_passes() {
        let elem = test_element_with_attrs("img", vec![("alt", "")]);
        let result = validate_img_alt(&elem);
        assert!(result.is_none(), "img with empty alt (decorative) should pass");
    }

    #[test]
    fn test_non_img_ignored() {
        let elem = test_element("div");
        let result = validate_img_alt(&elem);
        assert!(result.is_none(), "non-img elements should be ignored");
    }

    // =========================================================================
    // validate_input_type
    // =========================================================================

    #[test]
    fn test_valid_input_type_text() {
        let elem = test_element_with_attrs("input", vec![("type", "text")]);
        let result = validate_input_type(&elem);
        assert!(result.is_none(), "input type=text should be valid");
    }

    #[test]
    fn test_valid_input_type_email() {
        let elem = test_element_with_attrs("input", vec![("type", "email")]);
        let result = validate_input_type(&elem);
        assert!(result.is_none(), "input type=email should be valid");
    }

    #[test]
    fn test_invalid_input_type_fails() {
        let elem = test_element_with_attrs("input", vec![("type", "invalid_type")]);
        let result = validate_input_type(&elem);
        assert!(result.is_some(), "invalid input type should fail");
    }

    #[test]
    fn test_valid_button_type_submit() {
        let elem = test_element_with_attrs("button", vec![("type", "submit")]);
        let result = validate_input_type(&elem);
        assert!(result.is_none(), "button type=submit should be valid");
    }

    #[test]
    fn test_invalid_button_type_fails() {
        let elem = test_element_with_attrs("button", vec![("type", "invalid")]);
        let result = validate_input_type(&elem);
        assert!(result.is_some(), "invalid button type should fail");
    }

    #[test]
    fn test_input_without_type_ignored() {
        let elem = test_element("input");
        let result = validate_input_type(&elem);
        assert!(result.is_none(), "input without type attr should be ignored");
    }

    // =========================================================================
    // validate_aria_roles
    // =========================================================================

    #[test]
    fn test_valid_aria_role_button() {
        let elem = test_element_with_attrs("div", vec![("role", "button")]);
        let result = validate_aria_roles(&elem);
        assert!(result.is_none(), "role=button should be valid");
    }

    #[test]
    fn test_valid_aria_role_navigation() {
        let elem = test_element_with_attrs("nav", vec![("role", "navigation")]);
        let result = validate_aria_roles(&elem);
        assert!(result.is_none(), "role=navigation should be valid");
    }

    #[test]
    fn test_invalid_aria_role_fails() {
        let elem = test_element_with_attrs("div", vec![("role", "notarole")]);
        let result = validate_aria_roles(&elem);
        assert!(result.is_some(), "invalid aria role should fail");
    }

    #[test]
    fn test_element_without_role_ignored() {
        let elem = test_element("div");
        let result = validate_aria_roles(&elem);
        assert!(result.is_none(), "element without role should be ignored");
    }

    // =========================================================================
    // validate_button_content
    // =========================================================================

    #[test]
    fn test_button_with_text_passes() {
        let text = crate::token_parser::Text { content: "Click me".to_string(), span: proc_macro2::Span::call_site() };
        let elem = test_element_with_children("button", vec![crate::token_parser::Node::Text(text)]);
        let result = validate_button_content(&elem);
        assert!(result.is_none(), "button with text should pass");
    }

    #[test]
    fn test_button_with_aria_label_passes() {
        let elem = test_element_with_attrs("button", vec![("aria-label", "Close")]);
        let result = validate_button_content(&elem);
        assert!(result.is_none(), "button with aria-label should pass");
    }

    #[test]
    fn test_button_with_title_passes() {
        let elem = test_element_with_attrs("button", vec![("title", "Submit")]);
        let result = validate_button_content(&elem);
        assert!(result.is_none(), "button with title should pass");
    }

    #[test]
    fn test_button_without_content_fails() {
        let elem = test_element("button");
        let result = validate_button_content(&elem);
        assert!(result.is_some(), "button without accessible text should fail");
    }

    #[test]
    fn test_non_button_ignored() {
        let elem = test_element("div");
        let result = validate_button_content(&elem);
        assert!(result.is_none(), "non-button elements should be ignored");
    }

    // =========================================================================
    // validate_anchor_target_blank
    // =========================================================================

    #[test]
    fn test_anchor_without_target_passes() {
        let elem = test_element_with_attrs("a", vec![("href", "https://example.com")]);
        let result = validate_anchor_target_blank(&elem);
        assert!(result.is_none(), "anchor without target should pass");
    }

    #[test]
    fn test_anchor_target_blank_with_rel_passes() {
        let elem = test_element_with_attrs("a", vec![
            ("href", "https://example.com"),
            ("target", "_blank"),
            ("rel", "noopener noreferrer"),
        ]);
        let result = validate_anchor_target_blank(&elem);
        assert!(result.is_none(), "target=_blank with rel should pass");
    }

    #[test]
    fn test_anchor_target_blank_without_rel_fails() {
        let elem = test_element_with_attrs("a", vec![
            ("href", "https://example.com"),
            ("target", "_blank"),
        ]);
        let result = validate_anchor_target_blank(&elem);
        assert!(result.is_some(), "target=_blank without rel should fail");
    }

    #[test]
    fn test_anchor_target_self_passes() {
        let elem = test_element_with_attrs("a", vec![
            ("href", "/page"),
            ("target", "_self"),
        ]);
        let result = validate_anchor_target_blank(&elem);
        assert!(result.is_none(), "target=_self should pass");
    }

    #[test]
    fn test_non_anchor_ignored() {
        let elem = test_element("button");
        let result = validate_anchor_target_blank(&elem);
        assert!(result.is_none(), "non-anchor elements should be ignored");
    }

    // =========================================================================
    // validate_aria_values
    // =========================================================================

    #[test]
    fn test_aria_expanded_true_passes() {
        let elem = test_element_with_attrs("button", vec![("aria-expanded", "true")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "aria-expanded=true should pass");
    }

    #[test]
    fn test_aria_expanded_false_passes() {
        let elem = test_element_with_attrs("button", vec![("aria-expanded", "false")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "aria-expanded=false should pass");
    }

    #[test]
    fn test_aria_expanded_yes_fails() {
        let elem = test_element_with_attrs("button", vec![("aria-expanded", "yes")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_some(), "aria-expanded=yes should fail");
    }

    #[test]
    fn test_aria_expanded_open_fails() {
        let elem = test_element_with_attrs("div", vec![("aria-expanded", "open")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_some(), "aria-expanded=open should fail");
    }

    #[test]
    fn test_aria_checked_mixed_passes() {
        let elem = test_element_with_attrs("input", vec![("aria-checked", "mixed")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "aria-checked=mixed should pass");
    }

    #[test]
    fn test_aria_hidden_true_passes() {
        let elem = test_element_with_attrs("div", vec![("aria-hidden", "true")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "aria-hidden=true should pass");
    }

    #[test]
    fn test_aria_hidden_yes_fails() {
        let elem = test_element_with_attrs("div", vec![("aria-hidden", "yes")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_some(), "aria-hidden=yes should fail");
    }

    #[test]
    fn test_aria_live_polite_passes() {
        let elem = test_element_with_attrs("div", vec![("aria-live", "polite")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "aria-live=polite should pass");
    }

    #[test]
    fn test_aria_live_gentle_fails() {
        let elem = test_element_with_attrs("div", vec![("aria-live", "gentle")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_some(), "aria-live=gentle should fail");
    }

    #[test]
    fn test_aria_haspopup_menu_passes() {
        let elem = test_element_with_attrs("button", vec![("aria-haspopup", "menu")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "aria-haspopup=menu should pass");
    }

    #[test]
    fn test_aria_haspopup_true_passes() {
        let elem = test_element_with_attrs("button", vec![("aria-haspopup", "true")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "aria-haspopup=true should pass");
    }

    #[test]
    fn test_aria_haspopup_popup_fails() {
        let elem = test_element_with_attrs("button", vec![("aria-haspopup", "popup")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_some(), "aria-haspopup=popup should fail");
    }

    #[test]
    fn test_aria_unknown_attribute_skipped() {
        // Unknown aria-* attributes are skipped for forward compatibility
        let elem = test_element_with_attrs("div", vec![("aria-future-attr", "anything")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "unknown aria-* attributes should be skipped");
    }

    #[test]
    fn test_aria_label_not_validated() {
        // aria-label accepts free-form text, not enumerated values
        let elem = test_element_with_attrs("button", vec![("aria-label", "Close menu")]);
        let result = validate_aria_values(&elem);
        assert!(result.is_none(), "aria-label should not be value-validated");
    }

    // =========================================================================
    // validate_iframe_title
    // =========================================================================

    #[test]
    fn test_iframe_without_title_fails() {
        let elem = test_element("iframe");
        let result = validate_iframe_title(&elem);
        assert!(result.is_some(), "iframe without title should fail");
    }

    #[test]
    fn test_iframe_with_title_passes() {
        let elem = test_element_with_attrs("iframe", vec![("title", "Map")]);
        let result = validate_iframe_title(&elem);
        assert!(result.is_none(), "iframe with title should pass");
    }

    #[test]
    fn test_non_iframe_ignored() {
        let elem = test_element("div");
        let result = validate_iframe_title(&elem);
        assert!(result.is_none(), "non-iframe elements should be ignored");
    }
}
