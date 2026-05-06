mod codegen;
mod component;
mod accessibility_validator;
mod action;
mod asset_rewriter;
mod css;
mod css_validator;
mod context;
mod head;
mod html_structure_validator;
mod inline_inject;
mod live;
mod page;
#[cfg(feature = "schema")]
mod schema;
mod style;
mod token_parser;

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
    let (style_bindings, _scoped_css, _global_css) = process_styles(&nodes);

    // 2. Generate HTML string construction code
    let html_construction = generate_nodes(&nodes);

    // 3. Generate bind validation checks
    let mut validation_checks = Vec::new();
    collect_bind_checks(&nodes, &mut validation_checks);

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

fn process_styles(nodes: &[token_parser::Node]) -> (proc_macro2::TokenStream, String, String) {
    let mut bindings = proc_macro2::TokenStream::new();
    let mut scoped_css = String::new();
    let mut global_css = String::new();

    for node in nodes {
        match node {
            token_parser::Node::Block(token_parser::Block::Style(style_block)) => {
                if style_block.is_global {
                    let output = style::process_global_style_macro(style_block.content.clone());
                    bindings.extend(output.bindings);
                    global_css.push_str(&output.css);
                } else {
                    let output = style::process_style_macro(style_block.content.clone());
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

fn collect_bind_checks(nodes: &[token_parser::Node], checks: &mut Vec<proc_macro2::TokenStream>) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if let Some(struct_path) = &elem.bind_struct {
                    let mut field_accesses = Vec::new();
                    collect_input_names(&elem.children, struct_path, &mut field_accesses);

                    if !field_accesses.is_empty() {
                        let check_fn_name =
                            quote::format_ident!("azumi_bind_check_{}", checks.len());

                        let check_block = quote! {
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
                                let mut field_path = quote! { data };
                                for ident in &field_idents {
                                    field_path = quote! { #field_path.#ident };
                                }
                                errors.push(quote! {
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

/// Validate that expressions don't use format! to build HTML/CSS/JS strings
/// This is a common AI anti-pattern: using format!("<div>{}</div>", value)
/// instead of proper html! macro interpolation or safe injection macros
fn validate_format_in_expressions(nodes: &[token_parser::Node]) -> Vec<proc_macro2::TokenStream> {
    let mut errors = vec![];

    fn check_node(node: &token_parser::Node, errors: &mut Vec<proc_macro2::TokenStream>) {
        match node {
            token_parser::Node::Expression(expr) => {
                let content_str = expr.content.to_string();
                let normalized = content_str.replace(' ', "");
                
                // Check for format! usage that's building HTML-like strings
                if normalized.contains("format!") {
                    let has_web_content = content_str.contains('<') 
                        || content_str.contains('>')
                        || content_str.contains("</")
                        || content_str.contains("href=")
                        || content_str.contains("class=")
                        || content_str.contains("style=")
                        // CSS patterns ({{ is format! escape for literal { in CSS)
                        || content_str.contains("{{")
                        || content_str.contains("window.")
                        || content_str.contains("document.")
                        || content_str.contains("addEventListener")
                        || content_str.contains("serde_json::to_string")
                        || content_str.contains("JSON.parse")
                        || content_str.contains("innerHTML")
                        || content_str.contains(".createElement")
                        || content_str.contains("document.write")
                        || content_str.contains("document.cookie");
                    
                    if has_web_content {
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
                                See: AI_GUIDE_FOR_WRITING_AZUMI.md"
                            );
                        });
                    }
                }
            }
            token_parser::Node::Element(elem) => {
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



fn collect_all_styles(nodes: &[token_parser::Node]) -> (String, String) {
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
                            if let token_parser::Node::Text(text) = child {
                                scoped_css.push_str(&text.content);
                                scoped_css.push('\n');
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

    let format_warnings = validate_format_in_expressions(nodes);
    if !format_warnings.is_empty() {
        let mut tokens = proc_macro2::TokenStream::new();
        for warn in format_warnings {
            tokens.extend(warn);
        }
        return tokens;
    }

    let (global_css, scoped_css) = collect_all_styles(nodes);
    let (valid_classes, valid_ids) = crate::css::extract_selectors(&scoped_css);

    let style_validation_errors =
        validate_nodes(nodes, &valid_classes, &valid_ids, !scoped_css.is_empty());
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
            let injected = inject_css_into_head(&mut working_nodes, &css_to_inject);
            let ctx = GenerationContext::with_scope(
                sid.clone(),
                valid_classes.clone(),
                valid_ids.clone(),
            );
            (codegen::generate_body_with_context(&working_nodes, &ctx), injected)
        } else {
            let mut temp_nodes = nodes.to_vec();
            let injected = inject_css_into_head(&mut temp_nodes, &css_to_inject);
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

#[allow(clippy::ptr_arg)]
#[allow(clippy::collapsible_match)]
fn inject_css_into_head(nodes: &mut Vec<token_parser::Node>, css: &str) -> bool {
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

mod validators;
    nodes: &[token_parser::Node],
    valid_classes: &std::collections::HashSet<String>,
    valid_ids: &std::collections::HashSet<String>,
    has_scoped_css: bool,
) -> proc_macro2::TokenStream {
    use quote::quote_spanned;
    let mut errors = vec![];

    #[allow(clippy::too_many_arguments)]
    fn collect_errors_recursive(
        nodes: &[token_parser::Node],
        valid_classes: &std::collections::HashSet<String>,
        valid_ids: &std::collections::HashSet<String>,
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
                                            // Check if a dashed version might exist (e.g., my_class -> my-class)
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

                        if let Some(err) = html_structure_validator::validate_attribute_name(attr) {
                            errors.push(err);
                        }
                    }

                    // Accessibility validations
                    if let Some(err) = accessibility_validator::validate_img_alt(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_input_type(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_aria_roles(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_button_content(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_anchor_target_blank(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_iframe_title(elem) {
                        errors.push(err);
                    }

                    // HTML structure validations
                    for err in html_structure_validator::validate_table_children(elem) {
                        errors.push(err);
                    }
                    for err in html_structure_validator::validate_list_children(elem) {
                        errors.push(err);
                    }
                    for err in html_structure_validator::validate_nested_forms(elem, is_inside_form)
                    {
                        errors.push(err);
                    }
                    for err in html_structure_validator::validate_button_interactive(
                        elem,
                        is_inside_button,
                    ) {
                        errors.push(err);
                    }
                    for err in html_structure_validator::validate_paragraph_content(elem) {
                        errors.push(err);
                    }
                    for err in
                        html_structure_validator::validate_anchor_nesting(elem, is_inside_anchor)
                    {
                        errors.push(err);
                    }
                    for err in html_structure_validator::validate_heading_content(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = html_structure_validator::validate_tag_name(elem) {
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
