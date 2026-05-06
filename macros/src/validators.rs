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
    has_scoped_css: bool,
) -> proc_macro2::TokenStream {
    let mut errors = vec![];

    #[allow(clippy::too_many_arguments)]
    fn collect_errors_recursive(
        nodes: &[token_parser::Node],
        valid_classes: &HashSet<String>,
        valid_ids: &HashSet<String>,
        _has_scoped_css: bool,
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

                        if name == "class" {
                            match &attr.value {
                                token_parser::AttributeValue::Static(_) => {
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!("Static class attributes are banned (e.g. class=\"...\"). Use class={variable_name} instead.");
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

                        if name == "id" {
                            match &attr.value {
                                token_parser::AttributeValue::Static(_) => {
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!("Static id attributes are banned (e.g. id=\"...\"). Use id={variable_name} instead.");
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(tokens) => {
                                    if let Ok(ident) = syn::parse2::<syn::Ident>(tokens.clone()) {
                                        let var_name = ident.to_string();
                                        if valid_classes.contains(&var_name)
                                            && !valid_ids.contains(&var_name)
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
                        _has_scoped_css,
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
                        _has_scoped_css,
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
                            _has_scoped_css,
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
                                _has_scoped_css,
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
                            _has_scoped_css,
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
                                _has_scoped_css,
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
        has_scoped_css,
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
