//! Style extraction and hoisting for the `html!` macro.
//!
//! This module handles two distinct but related operations:
//!
//! 1. **`process_styles`** — Processes `style!` macro blocks within `html!`,
//!    generating hoisted CSS bindings.
//! 2. **`collect_all_styles`** — Extracts inline `<style>` tag content and
//!    `style!` block content for CSS scoping and injection.

use crate::token_parser;

/// Process `style!` macro blocks and generate hoisted CSS bindings.
///
/// Returns `(bindings, scoped_css, global_css)` where:
/// - `bindings` are TokenStream variable assignments for the hoisted styles
/// - `scoped_css` is CSS that should be scoped to this component
/// - `global_css` is CSS that should be injected globally
pub(crate) fn process_styles(
    nodes: &[token_parser::Node],
) -> (proc_macro2::TokenStream, String, String) {
    let mut bindings = proc_macro2::TokenStream::new();
    let mut scoped_css = String::new();
    let mut global_css = String::new();

    for node in nodes {
        match node {
            token_parser::Node::Block(token_parser::Block::Style(style_block)) => {
                if style_block.is_global {
                    let output = crate::style::process_global_style_macro(style_block.content.clone());
                    bindings.extend(output.bindings);
                    global_css.push_str(&output.css);
                } else {
                    let output = crate::style::process_style_macro(style_block.content.clone());
                    bindings.extend(output.bindings);
                    scoped_css.push_str(&output.css);
                }
            }
            token_parser::Node::Element(elem) => {
                let (child_bindings, child_scoped, child_global) = process_styles(&elem.children);
                bindings.extend(child_bindings);
                scoped_css.push_str(&child_scoped);
                global_css.push_str(&child_global);
            }
            token_parser::Node::Fragment(frag) => {
                let (child_bindings, child_scoped, child_global) = process_styles(&frag.children);
                bindings.extend(child_bindings);
                scoped_css.push_str(&child_scoped);
                global_css.push_str(&child_global);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let (b, s, g) = process_styles(&if_block.then_branch);
                    bindings.extend(b);
                    scoped_css.push_str(&s);
                    global_css.push_str(&g);
                    if let Some(else_branch) = &if_block.else_branch {
                        let (b, s, g) = process_styles(else_branch);
                        bindings.extend(b);
                        scoped_css.push_str(&s);
                        global_css.push_str(&g);
                    }
                }
                token_parser::Block::For(for_block) => {
                    let (b, s, g) = process_styles(&for_block.body);
                    bindings.extend(b);
                    scoped_css.push_str(&s);
                    global_css.push_str(&g);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        let (b, s, g) = process_styles(&arm.body);
                        bindings.extend(b);
                        scoped_css.push_str(&s);
                        global_css.push_str(&g);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    let (b, s, g) = process_styles(&call_block.children);
                    bindings.extend(b);
                    scoped_css.push_str(&s);
                    global_css.push_str(&g);
                }
                _ => {}
            },
            _ => {}
        }
    }

    (bindings, scoped_css, global_css)
}

/// Collect all CSS from inline `<style>` tags and `style!` blocks.
///
/// Returns `(global_css, scoped_css)` for injection into the HTML head.
pub(crate) fn collect_all_styles(nodes: &[token_parser::Node]) -> (String, String) {
    let mut global_css = String::new();
    let mut scoped_css = String::new();
    collect_styles_recursive(nodes, &mut global_css, &mut scoped_css);
    (global_css, scoped_css)
}

fn collect_styles_recursive(
    nodes: &[token_parser::Node],
    global_css: &mut String,
    scoped_css: &mut String,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "style" {
                    if let Some(_src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                    } else {
                        for child in &elem.children {
                            match child {
                                token_parser::Node::Text(text) => {
                                    scoped_css.push_str(&text.content);
                                    scoped_css.push('\n');
                                }
                                token_parser::Node::Expression(expr) => {
                                    let expr_str = expr.to_string();
                                    scoped_css.push_str(&expr_str);
                                    scoped_css.push('\n');
                                }
                                _ => {}
                            }
                        }
                    }
                } else {
                    collect_styles_recursive(&elem.children, global_css, scoped_css);
                }
            }
            token_parser::Node::Fragment(frag) => {
                collect_styles_recursive(&frag.children, global_css, scoped_css);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::Style(style_block) => {
                    let css_content =
                        crate::style::reconstruct_css_from_tokens(style_block.content.clone());
                    if style_block.is_global {
                        global_css.push_str(&css_content);
                        global_css.push('\n');
                    } else {
                        scoped_css.push_str(&css_content);
                        scoped_css.push('\n');
                    }
                }
                token_parser::Block::If(if_block) => {
                    collect_styles_recursive(&if_block.then_branch, global_css, scoped_css);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_styles_recursive(else_branch, global_css, scoped_css);
                    }
                }
                token_parser::Block::For(for_block) => {
                    collect_styles_recursive(&for_block.body, global_css, scoped_css);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_styles_recursive(&arm.body, global_css, scoped_css);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    collect_styles_recursive(&call_block.children, global_css, scoped_css);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Inject CSS text into the first `<head>` element found in the node tree.
///
/// Returns `true` if a `<head>` was found and modified, `false` otherwise.
#[allow(clippy::ptr_arg)]
#[allow(clippy::collapsible_match)]
pub(crate) fn inject_css_into_head(nodes: &mut Vec<token_parser::Node>, css: &str) -> bool {
    for node in nodes.iter_mut() {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "head" {
                    let content = css.to_string();
                    let text_node = token_parser::Node::RawText(token_parser::Text {
                        content,
                        span: elem.span,
                    });
                    elem.children.insert(0, text_node);
                    return true;
                }
                if inject_css_into_head(&mut elem.children, css) {
                    return true;
                }
            }
            token_parser::Node::Fragment(frag) => {
                if inject_css_into_head(&mut frag.children, css) {
                    return true;
                }
            }
            token_parser::Node::Block(token_parser::Block::If(if_block)) => {
                if inject_css_into_head(&mut if_block.then_branch, css) {
                    return true;
                } else if let Some(else_branch) = &mut if_block.else_branch {
                    inject_css_into_head(else_branch, css);
                }
            }
            _ => {}
        }
    }
    false
}
