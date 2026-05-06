mod accessibility_validator;
mod action;
mod asset_rewriter;
mod codegen;
mod component;
mod context;
mod css;
mod css_validator;
mod head;
mod html_structure_validator;
mod inline_inject;
mod live;
mod page;
mod schema;
mod style;
mod style_processing;
mod token_parser;
mod validators;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;

#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
    head::expand_head(input)
}

#[proc_macro_attribute]
pub fn page(attr: TokenStream, item: TokenStream) -> TokenStream {
    page::expand_page(attr, item)
}

#[cfg(feature = "schema")]
#[proc_macro_derive(Schema, attributes(schema))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    schema::derive_schema(input)
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    component::expand_component(item)
}

#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    action::expand_action(item)
}

#[proc_macro_attribute]
pub fn live(attr: TokenStream, item: TokenStream) -> TokenStream {
    live::expand_live(attr, item)
}

#[proc_macro_attribute]
pub fn live_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    live::expand_live_impl(attr, item)
}

#[proc_macro_attribute]
pub fn predict(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro]
pub fn json_data(input: TokenStream) -> TokenStream {
    inline_inject::expand_json_data(input)
}

use context::GenerationContext;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as token_parser::HtmlInput);
    let mut nodes = input.nodes;

    // Auto-scope Asset Paths (Rewrites /img/logo.png -> /assets/logo.a8b9.png)
    asset_rewriter::rewrite_nodes(&mut nodes);

    // 1. Process styles (hoist <style> tags)
    let (style_bindings, _scoped_css, _global_css) = style_processing::process_styles(&nodes);

    // 2. Generate HTML string construction code
    let html_construction = generate_nodes(&nodes);

    // 3. Generate bind validation checks
    let mut validation_checks = Vec::new();
    validators::collect_bind_checks(&nodes, &mut validation_checks);

    let expanded = quote! {
        {
            // Import FallbackRender to ensure render_azumi works even if trait not imported by user
            #[allow(unused_imports)]
            use azumi::FallbackRender;

            // Inject style bindings (hoisted)
            #style_bindings

            // Validation block (compile-time only)
            const _: () = {
                #(#validation_checks)*
            };

            // Runtime HTML generation
            // IMPORTANT: Use `from_fn_once` instead of `from_fn` because the html!
            // closure may capture owned values (via `move`) that are also used in
            // component props. `FnOnce` closures can consume captured values, while
            // `Fn` closures can only borrow them, causing "cannot move" errors.
            //
            // `FnOnceComponent` caches its rendered result, so the closure is only
            // invoked once - which is the typical case for a complete HTML page.
            azumi::from_fn_once(move |f| {
                #html_construction
            })
        }
    };

    TokenStream::from(expanded)
}

fn generate_nodes(nodes: &[token_parser::Node]) -> proc_macro2::TokenStream {
    let body = generate_body(nodes, first_node_span(nodes));
    quote! {
        #body
        Ok(())
    }
}

fn first_node_span(nodes: &[token_parser::Node]) -> Option<(usize, usize)> {
    fn find_span(node: &token_parser::Node) -> Option<(usize, usize)> {
        match node {
            token_parser::Node::Element(elem) => {
                let loc = elem.span.start();
                Some((loc.line, loc.column))
            }
            token_parser::Node::Text(text) => {
                let loc = text.span.start();
                Some((loc.line, loc.column))
            }
            token_parser::Node::RawText(text) => {
                let loc = text.span.start();
                Some((loc.line, loc.column))
            }
            token_parser::Node::Expression(expr) => {
                let loc = expr.span.start();
                Some((loc.line, loc.column))
            }
            token_parser::Node::Fragment(frag) => {
                for child in &frag.children {
                    if let Some(span) = find_span(child) {
                        return Some(span);
                    }
                }
                None
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    if let Some(span) = first_node_span(&if_block.then_branch) {
                        return Some(span);
                    }
                    if let Some(else_branch) = &if_block.else_branch {
                        if let Some(span) = first_node_span(else_branch) {
                            return Some(span);
                        }
                    }
                    let loc = if_block.span.start();
                    Some((loc.line, loc.column))
                }
                token_parser::Block::For(for_block) => {
                    if let Some(span) = first_node_span(&for_block.body) {
                        return Some(span);
                    }
                    let loc = for_block.span.start();
                    Some((loc.line, loc.column))
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        if let Some(span) = first_node_span(&arm.body) {
                            return Some(span);
                        }
                    }
                    let loc = match_block.span.start();
                    Some((loc.line, loc.column))
                }
                token_parser::Block::Call(call_block) => {
                    if let Some(span) = first_node_span(&call_block.children) {
                        return Some(span);
                    }
                    let loc = call_block.span.start();
                    Some((loc.line, loc.column))
                }
                token_parser::Block::Let(let_block) => {
                    let loc = let_block.span.start();
                    Some((loc.line, loc.column))
                }
                token_parser::Block::Style(style_block) => {
                    let loc = style_block.span.start();
                    Some((loc.line, loc.column))
                }
                token_parser::Block::Component(comp_block) => {
                    let loc = comp_block.span.start();
                    Some((loc.line, loc.column))
                }
            },
            token_parser::Node::Comment(comment) => {
                let loc = comment.span.start();
                Some((loc.line, loc.column))
            }
            token_parser::Node::Doctype(doctype) => {
                let loc = doctype.span.start();
                Some((loc.line, loc.column))
            }
        }
    }

    for node in nodes {
        if let Some(span) = find_span(node) {
            return Some(span);
        }
    }
    None
}

fn azumi_scope_id_from_span(line: usize, col: usize) -> String {
    use fnv::FnvHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = FnvHasher::default();
    line.hash(&mut hasher);
    col.hash(&mut hasher);
    format!("s{:x}", hasher.finish())
}




fn generate_body(
    nodes: &[token_parser::Node],
    span: Option<(usize, usize)>,
) -> proc_macro2::TokenStream {
    let css_validation_errors = css_validator::validate_component_css(nodes);
    if !css_validation_errors.is_empty() {
        return css_validation_errors;
    }

    let order_errors = html_structure_validator::validate_node_order(nodes);
    if !order_errors.is_empty() {
        let mut tokens = proc_macro2::TokenStream::new();
        for err in order_errors {
            tokens.extend(err);
        }
        return tokens;
    }

    let raw_warnings = html_structure_validator::validate_raw_usage(nodes);
    if !raw_warnings.is_empty() {
        let mut tokens = proc_macro2::TokenStream::new();
        for warn in raw_warnings {
            tokens.extend(warn);
        }
        return tokens;
    }

    let format_warnings = validators::validate_format_in_expressions(nodes);
    if !format_warnings.is_empty() {
        let mut tokens = proc_macro2::TokenStream::new();
        for warn in format_warnings {
            tokens.extend(warn);
        }
        return tokens;
    }

    let (global_css, scoped_css) = style_processing::collect_all_styles(nodes);
    let (valid_classes, valid_ids) = crate::css::extract_selectors(&scoped_css);

    let style_validation_errors =
        validators::validate_nodes(nodes, &valid_classes, &valid_ids, !scoped_css.is_empty());
    if !style_validation_errors.is_empty() {
        return style_validation_errors;
    }

    let has_global = !global_css.is_empty();
    let has_scoped = !scoped_css.is_empty();

    if has_global || has_scoped {
        let (scoped_output, scope_id) = if has_scoped {
            let scope_id = span
                .map(|(line, col)| azumi_scope_id_from_span(line, col))
                .unwrap_or_else(|| {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};
                    let mut hasher = DefaultHasher::new();
                    scoped_css.hash(&mut hasher);
                    format!("s{:x}", hasher.finish())
                });
            (
                crate::css::scope_css(&scoped_css, &scope_id),
                Some(scope_id),
            )
        } else {
            (String::new(), None)
        };

        // INVARIANT: scope_id is always Some when has_scoped is true
        // This is guaranteed by the code logic above
        debug_assert!(
            !has_scoped || scope_id.is_some(),
            "scope_id must be Some when has_scoped is true"
        );
        let scope_id_str = scope_id.as_deref().unwrap_or("");

        let css_to_inject = if has_global {
            if has_scoped {
                format!(
                    "<style>{}</style><style data-azumi-scope=\"{}\">{}</style>",
                    global_css, scope_id_str, scoped_output
                )
            } else {
                format!("<style>{}</style>", global_css)
            }
        } else if has_scoped {
            format!(
                "<style data-azumi-scope=\"{}\">{}</style>",
                scope_id_str, scoped_output
            )
        } else {
            String::new()
        };

        let (body_content, css_injected) = if let Some(sid) = &scope_id {
            let mut working_nodes = nodes.to_vec();
            let injected = style_processing::inject_css_into_head(&mut working_nodes, &css_to_inject);
            let ctx = GenerationContext::with_scope(
                sid.clone(),
                valid_classes.clone(),
                valid_ids.clone(),
            );
            (codegen::generate_body_with_context(&working_nodes, &ctx), injected)
        } else {
            let mut temp_nodes = nodes.to_vec();
            let injected = style_processing::inject_css_into_head(&mut temp_nodes, &css_to_inject);
            (
                codegen::generate_body_with_context(&temp_nodes, &GenerationContext::normal()),
                injected,
            )
        };

        if css_injected {
            body_content
        } else {
            quote! {
                write!(f, "{}", #css_to_inject)?;
                #body_content
            }
        }
    } else {
        codegen::generate_body_with_context(nodes, &GenerationContext::normal())
    }
}

