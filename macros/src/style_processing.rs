//! Style extraction and hoisting for the `html!` macro.
//!
//! This module handles style processing in a single unified pass:
//!
//! **`process_all_styles`** — Single-pass extraction that returns:
//! - Hoisted CSS bindings (TokenStream for `style!` macro blocks)
//! - Global CSS content (for `<style>` global and `style!` global blocks)
//! - Scoped CSS content (for `<style>` tags and `style!` scoped blocks)
//! - Whether dynamic styles exist (`<style>{variable}</style>`)

use crate::token_parser;

/// Result of processing all styles in a single pass.
pub(crate) struct StyleExtraction {
    pub bindings: proc_macro2::TokenStream,
    pub global_css: String,
    pub scoped_css: String,
    pub has_dynamic_styles: bool,
}

/// Single-pass extraction of all style information from the node tree.
///
/// This replaces the previous two-pass approach (separate `process_styles` +
/// `collect_all_styles`) with one traversal that captures everything.
pub(crate) fn process_all_styles(nodes: &[token_parser::Node]) -> StyleExtraction {
    let mut result = StyleExtraction {
        bindings: proc_macro2::TokenStream::new(),
        global_css: String::new(),
        scoped_css: String::new(),
        has_dynamic_styles: false,
    };
    process_styles_recursive(nodes, &mut result);
    result
}

fn process_styles_recursive(nodes: &[token_parser::Node], result: &mut StyleExtraction) {
    for node in nodes {
        match node {
            // Handle <style> elements
            token_parser::Node::Element(elem) => {
                if elem.name == "style" {
                    if let Some(_src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        // External style reference — no content to collect
                    } else {
                        for child in &elem.children {
                            match child {
                                token_parser::Node::Text(text) => {
                                    result.scoped_css.push_str(&text.content);
                                    result.scoped_css.push('\n');
                                }
                                token_parser::Node::Expression(_) => {
                                    result.has_dynamic_styles = true;
                                }
                                _ => {}
                            }
                        }
                    }
                } else {
                    process_styles_recursive(&elem.children, result);
                }
            }
            token_parser::Node::Fragment(frag) => {
                process_styles_recursive(&frag.children, result);
            }
            // Handle style! macro blocks
            token_parser::Node::Block(token_parser::Block::Style(style_block)) => {
                if style_block.is_global {
                    let output = crate::style::process_global_style_macro(style_block.content.clone());
                    result.bindings.extend(output.bindings);
                    result.global_css.push_str(&output.css);
                } else {
                    let output = crate::style::process_style_macro(style_block.content.clone());
                    result.bindings.extend(output.bindings);
                    result.scoped_css.push_str(&output.css);
                }
            }
            // Handle control flow blocks
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    process_styles_recursive(&if_block.then_branch, result);
                    if let Some(else_branch) = &if_block.else_branch {
                        process_styles_recursive(else_branch, result);
                    }
                }
                token_parser::Block::For(for_block) => {
                    process_styles_recursive(&for_block.body, result);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        process_styles_recursive(&arm.body, result);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    process_styles_recursive(&call_block.children, result);
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
