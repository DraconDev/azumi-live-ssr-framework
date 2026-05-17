//! HTML structure validation rules.
//!
//! Each function validates a specific structural rule (table children,
//! list nesting, form nesting, etc.) and returns compile errors as TokenStreams.

use crate::token_parser::Element;
use proc_macro2::TokenStream;
use quote::quote_spanned;

/// Rule: Only valid children allowed inside `<table>`.
pub fn validate_table_children(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "table" {
        return vec![];
    }

    let valid_table_children = [
        "caption", "colgroup", "thead", "tbody", "tfoot", "tr", "style", "script", "template",
    ];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if !valid_table_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid child <{}> inside <table>. Valid children: caption, colgroup, thead, tbody, tfoot, tr",
                    child_elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule: Only `<li>` allowed as direct children of `<ul>` and `<ol>`.
pub fn validate_list_children(elem: &Element) -> Vec<TokenStream> {
    let valid_list_children = ["li", "script", "template", "style"];

    if elem.name != "ul" && elem.name != "ol" {
        return vec![];
    }

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if !valid_list_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid child <{}> inside <{}>. Only <li>, <script>, <template>, and <style> are allowed",
                    child_elem.name, elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule: No nested `<form>` elements.
pub fn validate_nested_forms(elem: &Element, is_inside_form: bool) -> Vec<TokenStream> {
    if elem.name == "form" && is_inside_form {
        let msg = "Nested <form> elements are not allowed in HTML. Move the inner form outside the outer form.";
        return vec![quote_spanned! { elem.span =>
            compile_error!(#msg);
        }];
    }
    vec![]
}

/// Rule: No interactive content inside `<button>`.
pub fn validate_button_interactive(elem: &Element, is_inside_button: bool) -> Vec<TokenStream> {
    let interactive_tags = ["a", "button", "input", "select", "textarea"];

    if is_inside_button && interactive_tags.contains(&elem.name.as_str()) {
        let msg = format!(
            "Interactive element <{}> must not appear inside <button>. HTML spec forbids nested interactive content.",
            elem.name
        );
        return vec![quote_spanned! { elem.span =>
            compile_error!(#msg);
        }];
    }
    vec![]
}

/// Rule: Block elements are not allowed inside `<p>`.
pub fn validate_paragraph_content(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "p" {
        return vec![];
    }

    let block_elements = [
        "address", "article", "aside", "blockquote", "details", "div", "dl", "fieldset",
        "figcaption", "figure", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6", "header",
        "hgroup", "hr", "main", "menu", "nav", "ol", "p", "pre", "section", "table", "ul",
    ];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if block_elements.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Block element <{}> must not appear inside <p>. HTML spec automatically closes <p> before block elements.",
                    child_elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule: No nested `<a>` elements.
pub fn validate_anchor_nesting(elem: &Element, is_inside_anchor: bool) -> Vec<TokenStream> {
    if elem.name == "a" && is_inside_anchor {
        let msg = "Nested <a> elements are not allowed in HTML. Move the inner link outside the outer link.";
        return vec![quote_spanned! { elem.span =>
            compile_error!(#msg);
        }];
    }
    vec![]
}

/// Rule: Only phrasing content allowed inside heading elements.
pub fn validate_heading_content(elem: &Element) -> Vec<TokenStream> {
    let heading_tags = ["h1", "h2", "h3", "h4", "h5", "h6"];

    if !heading_tags.contains(&elem.name.as_str()) {
        return vec![];
    }

    let non_phrasing_content = [
        "div", "p", "ul", "ol", "table", "form", "fieldset", "blockquote", "pre", "hr",
        "address", "article", "aside", "details", "figcaption", "figure", "footer", "header",
        "hgroup", "main", "menu", "nav", "section",
    ];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if non_phrasing_content.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Block element <{}> must not appear inside <{}>. Headings may only contain phrasing (inline) content.",
                    child_elem.name, elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}
