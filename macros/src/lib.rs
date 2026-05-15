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
use quote::quote;
use syn::parse_macro_input;

#[doc(hidden)]
#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
    head::expand_head(input)
}

#[doc(hidden)]
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
/// Use `#[azumi::live]` instead — this macro is retained for backward compatibility.
#[doc(hidden)]
pub fn live_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    live::expand_live_impl(attr, item)
}

#[doc(hidden)]
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
    let f_ident = proc_macro2::Ident::new("f", proc_macro2::Span::call_site());
    let html_construction = generate_nodes(&nodes, &f_ident);

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
            azumi::from_fn_once(move |#f_ident| {
                #html_construction
            })
        }
    };

    TokenStream::from(expanded)
}

fn generate_nodes(nodes: &[token_parser::Node], f_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    let body = generate_body(nodes, codegen::first_node_span(nodes), f_ident);
    quote! {
        #body
        Ok(())
    }
}


fn generate_body(
    nodes: &[token_parser::Node],
    span: Option<(usize, usize)>,
    f_ident: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    // Collect ALL validation errors from all validators instead of short-circuiting
    let mut all_errors = proc_macro2::TokenStream::new();

    let css_validation_errors = css_validator::validate_component_css(nodes);
    all_errors.extend(css_validation_errors);

    let order_errors = html_structure_validator::validate_node_order(nodes);
    for err in order_errors {
        all_errors.extend(err);
    }

    let raw_warnings = html_structure_validator::validate_raw_usage(nodes);
    for warn in raw_warnings {
        all_errors.extend(warn);
    }

    let format_warnings = validators::validate_format_in_expressions(nodes);
    for warn in format_warnings {
        all_errors.extend(warn);
    }

    let (global_css, scoped_css, has_dynamic_styles) = style_processing::collect_all_styles(nodes);
    let (valid_classes, valid_ids) = crate::css::extract_selectors(&scoped_css);

    let has_scoped_css = !scoped_css.is_empty() || has_dynamic_styles;
    let style_validation_errors =
        validators::validate_nodes(nodes, &valid_classes, &valid_ids, has_scoped_css, has_dynamic_styles);
    all_errors.extend(style_validation_errors);

    // If any validator produced errors, return them all at once
    if !all_errors.is_empty() {
        return all_errors;
    }

    let has_global = !global_css.is_empty();
    let has_scoped = !scoped_css.is_empty();

    if has_global || has_scoped {
        let (scoped_output, scope_id) = if has_scoped {
            let scope_id = span
                .map(|(line, col)| codegen::azumi_scope_id_from_span(line, col))
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
            (codegen::generate_body_with_context(&working_nodes, &ctx, f_ident), injected)
        } else {
            let mut temp_nodes = nodes.to_vec();
            let injected = style_processing::inject_css_into_head(&mut temp_nodes, &css_to_inject);
            (
                codegen::generate_body_with_context(&temp_nodes, &GenerationContext::normal(), f_ident),
                injected,
            )
        };

        if css_injected {
            body_content
        } else {
            quote! {
                write!(#f_ident, "{}", #css_to_inject)?;
                #body_content
            }
        }
    } else {
        codegen::generate_body_with_context(nodes, &GenerationContext::normal(), f_ident)
    }
}

