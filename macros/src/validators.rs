//! HTML structure and accessibility validators for the `html!` macro.
//!
//! This module contains `validate_nodes`, which performs comprehensive
//! validation of the parsed HTML AST before code generation:
//!
//! - Attribute validation (static class/id/style bans, DSL misuse)
//! - Accessibility checks (img alt, input types, ARIA roles, etc.)
//! - HTML structure rules (table children, nested forms, button content, etc.)

use crate::token_parser;
use quote::quote_spanned;
use std::collections::HashSet;

/// Validates an AST node list and returns compile-error tokens for any violations.
///
/// Checks performed:
/// - Static `class`, `id`, `style` attributes are banned (must use dynamic expressions)
/// - Class/ID variable names must correspond to valid CSS selectors
/// - Accessibility requirements (alt text, ARIA roles, button content, etc.)
/// - HTML structure rules (table children, nested forms, paragraph content, etc.)
#[allow(clippy::too_many_arguments)]
pub(crate) fn validate_nodes(
    nodes: &[token_parser::Node],
    valid_classes: &HashSet<String>,
    valid_ids: &HashSet<String>,
    has_dynamic_styles: bool,
) -> proc_macro2::TokenStream {
    let mut errors = vec![];

    fn collect_errors_recursive(
        nodes: &[token_parser::Node],
        valid_classes: &HashSet<String>,
        valid_ids: &HashSet<String>,
        has_dynamic_styles: bool,
        errors: &mut Vec<proc_macro2::TokenStream>,
        is_inside_form: bool,
        is_inside_button: bool,
        is_inside_anchor: bool,
    ) {
        for node in nodes {
            match node {
                token_parser::Node::Element(elem) => {
                    for attr in &elem.attrs {
                        let name = &attr.name;

                        if name == "style" {
                            if let token_parser::AttributeValue::Static(_) = &attr.value {
                                let error_span = attr.value_span.unwrap_or(attr.span);
                                errors.push(quote_spanned! { error_span =>
                                    compile_error!("Static style attributes are banned (e.g. style=\"...\"). Use style={--prop: value} instead (no space after --).");
                                });
                            }
                        }

                        if name == "class:external" {
                            // class:external bypasses validation — external CSS class names
                            // are still HTML-escaped but not validated against <style> blocks.
                        } else if name == "class" {
                            match &attr.value {
                                token_parser::AttributeValue::Static(_) => {
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!("Static class attributes are banned (e.g. class=\"...\"). Use class:external=\"...\" for third-party CSS or class={variable_name} for Azumi-managed classes.");
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(tokens) => {
                                    if let Ok(ident) = syn::parse2::<syn::Ident>(tokens.clone()) {
                                        let var_name = ident.to_string();
                                        if valid_ids.contains(&var_name)
                                            && !valid_classes.contains(&var_name)
                                        {
                                            let msg = format!(
                                                "Variable '{}' refers to an ID selector (#{}) but is used in 'class' attribute. Did you mean to use 'id={}'?",
                                                var_name, var_name, var_name
                                            );
                                            errors.push(quote_spanned! { ident.span() =>
                                                compile_error!(#msg);
                                            });
                                        } else if !valid_classes.contains(&var_name)
                                            && !valid_ids.contains(&var_name)
                                            && !has_dynamic_styles
                                        {
                                            let dashed = var_name.replace('_', "-");
                                            if valid_classes.contains(&dashed) {
                                                let msg = format!(
                                                    "Variable '{}' is not a valid class binding. Did you mean '{}'? Note: CSS class names with dashes must be written with underscores in Rust (e.g., 'my_class' for '.my-class').",
                                                    var_name, dashed
                                                );
                                                errors.push(quote_spanned! { ident.span() =>
                                                    compile_error!(#msg);
                                                });
                                            }
                                        }
                                    }
                                }
                                token_parser::AttributeValue::StyleDsl(_) => {
                                    errors.push(quote_spanned! { attr.span =>
                                        compile_error!("Style DSL syntax { --var: val } is only allowed in 'style' attribute.");
                                    });
                                }
                                _ => {}
                            }
                        }

                        if name == "id:external" {
                            // id:external bypasses validation — external IDs
                        } else if name == "id" {
                            match &attr.value {
                                token_parser::AttributeValue::Static(_) => {
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!("Static id attributes are banned (e.g. id=\"...\"). Use id:external=\"...\" for third-party IDs or id={variable_name} for Azumi-managed IDs.");
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(tokens) => {
                                    if let Ok(ident) = syn::parse2::<syn::Ident>(tokens.clone()) {
                                        let var_name = ident.to_string();
                                        if valid_classes.contains(&var_name)
                                            && !valid_ids.contains(&var_name)
                                            && !has_dynamic_styles
                                        {
                                            let msg = format!(
                                                "Variable '{}' refers to a Class selector (.{}) but is used in 'id' attribute. Did you mean to use 'class={}'?",
                                                var_name, var_name, var_name
                                            );
                                            errors.push(quote_spanned! { ident.span() =>
                                                compile_error!(#msg);
                                            });
                                        }
                                    }
                                }
                                token_parser::AttributeValue::StyleDsl(_) => {
                                    errors.push(quote_spanned! { attr.span =>
                                        compile_error!("Style DSL syntax { --var: val } is only allowed in 'style' attribute.");
                                    });
                                }
                                _ => {}
                            }
                        }

                        if let Some(err) = crate::html_structure_validator::validate_attribute_name(attr) {
                            errors.push(err);
                        }
                    }

                    // Accessibility validations
                    if let Some(err) = crate::accessibility_validator::validate_img_alt(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = crate::accessibility_validator::validate_input_type(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = crate::accessibility_validator::validate_aria_roles(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = crate::accessibility_validator::validate_button_content(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = crate::accessibility_validator::validate_anchor_target_blank(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = crate::accessibility_validator::validate_iframe_title(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = crate::accessibility_validator::validate_aria_values(elem) {
                        errors.push(err);
                    }

                    // HTML structure validations
                    for err in crate::html_structure_validator::validate_table_children(elem) {
                        errors.push(err);
                    }
                    for err in crate::html_structure_validator::validate_list_children(elem) {
                        errors.push(err);
                    }
                    for err in crate::html_structure_validator::validate_nested_forms(elem, is_inside_form) {
                        errors.push(err);
                    }
                    for err in crate::html_structure_validator::validate_button_interactive(
                        elem,
                        is_inside_button,
                    ) {
                        errors.push(err);
                    }
                    for err in crate::html_structure_validator::validate_paragraph_content(elem) {
                        errors.push(err);
                    }
                    for err in crate::html_structure_validator::validate_anchor_nesting(elem, is_inside_anchor) {
                        errors.push(err);
                    }
                    for err in crate::html_structure_validator::validate_heading_content(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = crate::html_structure_validator::validate_tag_name(elem) {
                        errors.push(err);
                    }

                    let new_inside_form = is_inside_form || elem.name == "form";
                    let new_inside_button = is_inside_button || elem.name == "button";
                    let new_inside_anchor = is_inside_anchor || elem.name == "a";

                    collect_errors_recursive(
                        &elem.children,
                        valid_classes,
                        valid_ids,
                        has_dynamic_styles,
                        errors,
                        new_inside_form,
                        new_inside_button,
                        new_inside_anchor,
                    );
                }
                token_parser::Node::Fragment(frag) => {
                    collect_errors_recursive(
                        &frag.children,
                        valid_classes,
                        valid_ids,
                        has_dynamic_styles,
                        errors,
                        is_inside_form,
                        is_inside_button,
                        is_inside_anchor,
                    );
                }
                token_parser::Node::Block(block) => match block {
                    token_parser::Block::If(if_block) => {
                        collect_errors_recursive(
                            &if_block.then_branch,
                            valid_classes,
                            valid_ids,
                            has_dynamic_styles,
                            errors,
                            is_inside_form,
                            is_inside_button,
                            is_inside_anchor,
                        );
                        if let Some(else_branch) = &if_block.else_branch {
                            collect_errors_recursive(
                                else_branch,
                                valid_classes,
                                valid_ids,
                                has_dynamic_styles,
                                errors,
                                is_inside_form,
                                is_inside_button,
                                is_inside_anchor,
                            );
                        }
                    }
                    token_parser::Block::For(for_block) => {
                        collect_errors_recursive(
                            &for_block.body,
                            valid_classes,
                            valid_ids,
                            has_dynamic_styles,
                            errors,
                            is_inside_form,
                            is_inside_button,
                            is_inside_anchor,
                        );
                    }
                    token_parser::Block::Match(match_block) => {
                        for arm in &match_block.arms {
                            collect_errors_recursive(
                                &arm.body,
                                valid_classes,
                                valid_ids,
                                has_dynamic_styles,
                                errors,
                                is_inside_form,
                                is_inside_button,
                                is_inside_anchor,
                            );
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    collect_errors_recursive(
        nodes,
        valid_classes,
        valid_ids,
        has_dynamic_styles,
        &mut errors,
        false,
        false,
        false,
    );

    let mut tokens = proc_macro2::TokenStream::new();
    for err in errors {
        tokens.extend(err);
    }
    tokens
}

// ============================================================================
// Bind Validation (compile-time field existence checks)
// ============================================================================

/// Collects validation checks for `bind` attributes to ensure fields exist.
pub(crate) fn collect_bind_checks(
    nodes: &[token_parser::Node],
    checks: &mut Vec<proc_macro2::TokenStream>,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if let Some(struct_path) = &elem.bind_struct {
                    let mut field_accesses = Vec::new();
                    collect_input_names(&elem.children, struct_path, &mut field_accesses);

                    if !field_accesses.is_empty() {
                        let check_fn_name =
                            quote::format_ident!("azumi_bind_check_{}", checks.len());

                        let check_block = quote::quote! {
                            #[allow(unused_variables, non_snake_case)]
                            fn #check_fn_name(data: &#struct_path) {
                                #(#field_accesses)*
                            }
                        };
                        checks.push(check_block);
                    }
                }
                collect_bind_checks(&elem.children, checks);
            }
            token_parser::Node::Fragment(frag) => {
                collect_bind_checks(&frag.children, checks);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    collect_bind_checks(&if_block.then_branch, checks);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_bind_checks(else_branch, checks);
                    }
                }
                token_parser::Block::For(for_block) => {
                    collect_bind_checks(&for_block.body, checks);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_bind_checks(&arm.body, checks);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    collect_bind_checks(&call_block.children, checks);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[allow(clippy::only_used_in_recursion)]
fn collect_input_names(
    nodes: &[token_parser::Node],
    bind_struct: &syn::Path,
    errors: &mut Vec<proc_macro2::TokenStream>,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "input" || elem.name == "textarea" || elem.name == "select" {
                    for attr in &elem.attrs {
                        if attr.name == "name" {
                            if let token_parser::AttributeValue::Static(name_str) = &attr.value {
                                let span = attr.value_span.unwrap_or(attr.span);
                                let parts: Vec<&str> = name_str.split('.').collect();
                                let mut all_valid = true;
                                for part in &parts {
                                    if !is_valid_identifier(part) {
                                        all_valid = false;
                                        let error_msg = format!("Invalid field name: {}", part);
                                        errors.push(
                                            quote_spanned! {span=> compile_error!(#error_msg); },
                                        );
                                        break;
                                    }
                                }

                                if !all_valid {
                                    continue;
                                }

                                let field_idents: Vec<proc_macro2::Ident> = parts
                                    .iter()
                                    .map(|s| proc_macro2::Ident::new(s, span))
                                    .collect();

                                // Generate validation code to check field path exists at compile time
                                // For "parent.child", generates: let _ = &data.parent.child;
                                let mut field_path = quote::quote! { data };
                                for ident in &field_idents {
                                    field_path = quote::quote! { #field_path.#ident };
                                }
                                errors.push(quote::quote! {
                                    let _ = &#field_path;
                                });
                            }
                        }
                    }
                }
                collect_input_names(&elem.children, bind_struct, errors);
            }
            token_parser::Node::Fragment(frag) => {
                collect_input_names(&frag.children, bind_struct, errors);
            }
            _ => {}
        }
    }
}

fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

// ============================================================================
// format!() Detection (AI Anti-Pattern Prevention)
// ============================================================================

/// Validate that expressions don't use format! to build HTML/CSS/JS strings.
///
/// This is a common AI anti-pattern: using format!("<div>{}</div>", value)
/// instead of proper html! macro interpolation or safe injection macros.
pub(crate) fn validate_format_in_expressions(
    nodes: &[token_parser::Node],
) -> Vec<proc_macro2::TokenStream> {
    let mut errors = vec![];

    fn check_node(node: &token_parser::Node, errors: &mut Vec<proc_macro2::TokenStream>) {
        match node {
            token_parser::Node::Expression(expr) => {
                let content_str = expr.content.to_string();
                let normalized = content_str.replace(' ', "");

                // Check for format! usage that's building HTML-like strings
                if normalized.contains("format!")
                    && has_web_content_pattern(&content_str)
                {
                        errors.push(quote_spanned! { expr.span =>
                            compile_error!(
                                "Azumi: format!() detected building HTML/CSS/JS strings.\n\n\
                                Using format!() to build web content defeats Azumi's compile-time safety.\n\
                                If you need to format a value for display, do it outside the html! macro\n\
                                and pass the result as a variable.\n\
                                \n\
                                ✅ Correct patterns:\n\
                                \n\
                                // For dynamic text - use html! interpolation:\n\
                                html! { <p>{value}</p> }\n\
                                \n\
                                // For dynamic attributes - use expression:\n\
                                html! { <a href={url}>{label}</a> }\n\
                                \n\
                                // For JSON data - use json_data! macro:\n\
                                html! { {azumi::json_data!(\"MY_DATA\" = &data)} }\n\
                                \n\
                                // For CSS - use <style> tag with {variable}:\n\
                                html! { <style>{GLOBAL_CSS}</style> }\n\
                                \n\
                                // For JavaScript - use <script> tag with {variable}:\n\
                                html! { <script>{TRACKING_JS}</script> }\n\
                                \n\
                                ❌ Wrong pattern:\n\
                                \n\
                                html! { {format!(\"<div>{}</div>\", value)} }\n\
                                \n\
                                See: docs/guide.md"
                            );
                        });
                }
            }
            token_parser::Node::Element(elem) => {
                for attr in &elem.attrs {
                    match &attr.value {
                        token_parser::AttributeValue::Dynamic(tokens) => {
                            let content_str = tokens.to_string();
                            let normalized = content_str.replace(' ', "");
                            if normalized.contains("format!") && has_web_content_pattern(&content_str) {
                                let span = attr.value_span.unwrap_or(attr.span);
                                errors.push(quote_spanned! { span =>
                                    compile_error!(
                                        "Azumi: format!() detected building HTML/CSS/JS strings in attribute.\n\n\
                                        Using format!() to build web content defeats Azumi's compile-time safety.\n\
                                        Build the string outside html! and pass it as a variable.\n\
                                        \n\
                                        ✅ Correct patterns:\n\
                                        \n\
                                        // Format outside html!, inject safely:\n\
                                        let url = format!(\"/api/{}\", id);\n\
                                        html! { <a href={url}>\"Link\"</a> }\n\
                                        \n\
                                        ❌ Wrong:\n\
                                        html! { <a href={format!(\"/api/{}\", id)}>\"Link\"</a> }\n\
                                        \n\
                                        See: docs/guide.md"
                                    );
                                });
                            }
                        }
                        token_parser::AttributeValue::StyleDsl(pairs) => {
                            for (_, value_tokens) in pairs {
                                let content_str = value_tokens.to_string();
                                let normalized = content_str.replace(' ', "");
                                if normalized.contains("format!") && has_web_content_pattern(&content_str) {
                                    let span = attr.span;
                                    errors.push(quote_spanned! { span =>
                                        compile_error!(
                                            "Azumi: format!() detected in style DSL value.\n\
                                            Build CSS strings outside html! and use <style>{variable}</style> instead."
                                        );
                                    });
                                }
                            }
                        }
                        _ => {}
                    }
                }
                for child in &elem.children {
                    check_node(child, errors);
                }
            }
            token_parser::Node::Fragment(frag) => {
                for child in &frag.children {
                    check_node(child, errors);
                }
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    for child in &if_block.then_branch {
                        check_node(child, errors);
                    }
                    if let Some(else_branch) = &if_block.else_branch {
                        for child in else_branch {
                            check_node(child, errors);
                        }
                    }
                }
                token_parser::Block::For(for_block) => {
                    for child in &for_block.body {
                        check_node(child, errors);
                    }
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        for child in &arm.body {
                            check_node(child, errors);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    for node in nodes {
        check_node(node, &mut errors);
    }

    errors
}

fn has_web_content_pattern(content_str: &str) -> bool {
    content_str.contains('<')
        || content_str.contains('>')
        || content_str.contains("</")
        || content_str.contains("href=")
        || content_str.contains("class=")
        || content_str.contains("style=")
        || content_str.contains("window.")
        || content_str.contains("document.")
        || content_str.contains("addEventListener")
        || content_str.contains("serde_json::to_string")
        || content_str.contains("JSON.parse")
        || content_str.contains("innerHTML")
        || content_str.contains(".createElement")
        || content_str.contains("document.write")
        || content_str.contains("document.cookie")
        || (content_str.contains("{{") && content_str.contains("}}") && looks_like_css(content_str))
}

fn looks_like_css(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower.contains("color:")
        || lower.contains("background:")
        || lower.contains("margin:")
        || lower.contains("padding:")
        || lower.contains("display:")
        || lower.contains("font-size:")
        || lower.contains("border:")
        || lower.contains("width:")
        || lower.contains("height:")
        || lower.contains("position:")
        || lower.contains("flex")
        || lower.contains("grid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looks_like_css_positive() {
        assert!(looks_like_css(".btn {{ color: red; }}"));
        assert!(looks_like_css("background: blue"));
        assert!(looks_like_css("display: FLEX"));
        assert!(looks_like_css("margin: 10px"));
    }

    #[test]
    fn test_looks_like_css_negative() {
        assert!(!looks_like_css("Hello {{name}}"));
        assert!(!looks_like_css("format!(\"result: {}\", val)"));
        assert!(!looks_like_css("just some text"));
    }
}
