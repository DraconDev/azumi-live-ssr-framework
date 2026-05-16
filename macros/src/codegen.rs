//! Code generation for the `html!` macro body.
//!
//! This module contains `generate_body_with_context`, the core recursive
//! function that walks the parsed AST (Nodes) and emits `proc_macro2::TokenStream`
//! instructions for rendering HTML at runtime.
//!
//! Extracted from `macros/src/lib.rs` (~400 lines) to keep the main macro
//! dispatch file focused on validation pipeline orchestration.

use crate::context::{Context, GenerationContext};
use crate::token_parser;
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser};

// ---------------------------------------------------------------------------
// Helpers for parsing Component arguments
// ---------------------------------------------------------------------------

pub(crate) struct KeyValueArg {
    pub(crate) key: syn::Ident,
    pub(crate) value: syn::Expr,
}

impl Parse for KeyValueArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse()?;
        Ok(KeyValueArg { key, value })
    }
}

pub(crate) fn parse_args(
    tokens: proc_macro2::TokenStream,
) -> syn::Result<Vec<KeyValueArg>> {
    let parser =
        syn::punctuated::Punctuated::<KeyValueArg, syn::Token![,]>::parse_terminated;
    parser.parse2(tokens).map(|p| p.into_iter().collect())
}

// Resolve component path — components use exact module names (no suffix).
pub(crate) fn resolve_component_path(path: &syn::Path) -> syn::Path {
    path.clone()
}

// Helper for parsing space-separated expressions (e.g. class={expr1 expr2}).
pub(crate) fn parse_multi_exprs(
    input: ParseStream,
) -> syn::Result<Vec<syn::Expr>> {
    let mut exprs = Vec::new();
    while !input.is_empty() {
        exprs.push(input.parse()?);
    }
    Ok(exprs)
}

// ---------------------------------------------------------------------------
// Scope ID helpers
// ---------------------------------------------------------------------------

pub(crate) fn first_node_span(nodes: &[token_parser::Node]) -> Option<(usize, usize)> {
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

pub(crate) fn azumi_scope_id_from_span(line: usize, col: usize) -> String {
    use fnv::FnvHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = FnvHasher::default();
    line.hash(&mut hasher);
    col.hash(&mut hasher);
    format!("s{:x}", hasher.finish())
}

// ---------------------------------------------------------------------------
// Core code-generation function
// ---------------------------------------------------------------------------

/// Recursively generate `write!` instructions for a list of AST nodes.
///
/// The `ctx` parameter carries rendering mode (Normal / Script / Style),
/// CSS scope ID, and valid class/ID sets so that child nodes are generated
/// in the correct context (e.g., `<script>` children use script escaping).
pub(crate) fn generate_body_with_context(
    nodes: &[token_parser::Node],
    ctx: &GenerationContext,
    f_ident: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let mut instructions = Vec::new();

    for node in nodes {
        match node {
            token_parser::Node::Text(text) => {
                let content = &text.content;
                if !content.is_empty() {
                    instructions.push(quote! {
                        write!(#f_ident, "{}", azumi::Escaped(#content))?;
                    });
                }
            }
            token_parser::Node::RawText(text) => {
                let content = &text.content;
                if !content.is_empty() {
                    instructions.push(quote! {
                        write!(#f_ident, "{}", #content)?;
                    });
                }
            }
            token_parser::Node::Element(elem) => {
                let name = &elem.name;

                instructions.push(quote! {
                   write!(#f_ident, "<{}", #name)?;
                });

                for attr in &elem.attrs {
                    let attr_name = &attr.name;

                    // Handle az-* attributes (DSL treated as string)
                    if attr_name.starts_with("az-") {
                        // SPECIAL CASE: az-scope should evaluate its value as a Rust expression
                        if attr_name == "az-scope" {
                            match &attr.value {
                                token_parser::AttributeValue::Dynamic(tokens) => {
                                    // Evaluate the expression and escape for HTML attribute
                                    instructions.push(quote! {
                                        let __scope_val: String = #tokens;
                                        write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(&__scope_val))?;
                                    });
                                }
                                token_parser::AttributeValue::Static(val) => {
                                    instructions.push(quote! {
                                        write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(#val))?;
                                    });
                                }
                                _ => {}
                            }
                            continue;
                        }

                        // Other az-* attributes (like az-on) are DSL and treated as string literals
                        match &attr.value {
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                let s = tokens.to_string(); // Stringify tokens
                                instructions.push(quote! {
                                    write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(&#s))?;
                                });
                            }
                            token_parser::AttributeValue::Static(val) => {
                                instructions.push(quote! {
                                    write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(#val))?;
                                });
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // Handle on:* attributes (Events) - stringify method access
                    if attr_name.starts_with("on:") {
                        match &attr.value {
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                // Parse as expression to extracting name, but fallback to stringify
                                // Try to parse `obj.method` or `method`
                                let s = if let Ok(expr) = syn::parse2::<syn::Expr>(tokens.clone()) {
                                    match expr {
                                        syn::Expr::Field(f) => {
                                            if let syn::Member::Named(ident) = f.member {
                                                ident.to_string()
                                            } else {
                                                tokens.to_string().replace(" ", "")
                                            }
                                        }
                                        syn::Expr::Path(p) => {
                                            if let Some(ident) = p.path.get_ident() {
                                                ident.to_string()
                                            } else {
                                                tokens.to_string().replace(" ", "")
                                            }
                                        }
                                        syn::Expr::MethodCall(m) => {
                                            m.method.to_string()
                                        }
                                        _ => tokens.to_string().replace(" ", ""),
                                    }
                                } else {
                                    tokens.to_string().replace(" ", "")
                                };

                                let event_name =
                                    attr_name.strip_prefix("on:").unwrap_or(attr_name);
                                let dsl = format!("{} call {}", event_name, s);
                                instructions.push(quote! {
                                    write!(#f_ident, " az-on=\"{}\"", azumi::Escaped(&#dsl))?;
                                });
                            }
                            token_parser::AttributeValue::Static(val) => {
                                let event_name =
                                    attr_name.strip_prefix("on:").unwrap_or(attr_name);
                                let dsl = format!("{} call {}", event_name, val);
                                instructions.push(quote! {
                                    write!(#f_ident, " az-on=\"{}\"", azumi::Escaped(&#dsl))?;
                                });
                            }
                            _ => {}
                        }
                        continue;
                    }

                    if attr_name == "class" {
                        match &attr.value {
                            token_parser::AttributeValue::Static(val) => {
                                instructions.push(quote! {
                                    write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(#val))?;
                                });
                            }
                            token_parser::AttributeValue::Dynamic(tokens) => {
                                let exprs_res =
                                    syn::parse::Parser::parse2(parse_multi_exprs, tokens.clone());
                                match exprs_res {
                                    Ok(exprs) if !exprs.is_empty() => {
                                        let fmt = vec!["{}"; exprs.len()].join(" ");
                                        let mut format_args = Vec::new();
                                        for e in exprs {
                                            format_args.push(quote! { #e });
                                        }
                                        instructions.push(quote! {
                                            write!(#f_ident, " class=\"{}\"", azumi::Escaped(&format!(#fmt, #(#format_args),*)))?;
                                        });
                                    }
                                    _ => {
                                        instructions.push(quote! {
                                            write!(#f_ident, " class=\"{}\"", azumi::Escaped(&#tokens))?;
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }

                    if attr_name == "style" {
                        match &attr.value {
                            token_parser::AttributeValue::StyleDsl(props) => {
                                instructions
                                    .push(quote! { write!(#f_ident, " style=\"")?; });
                                for (i, (key, val)) in props.iter().enumerate() {
                                    if i > 0 {
                                        instructions
                                            .push(quote! { write!(#f_ident, "; ")?; });
                                    }
                                    instructions.push(quote! {
                                        write!(#f_ident, "{}: {}", azumi::Escaped(&#key), azumi::escape_css_string(&#val))?;
                                    });
                                }
                                instructions.push(quote! { write!(#f_ident, "\"")?; });
                            }
                            _ => match &attr.value {
                                token_parser::AttributeValue::Static(val) => {
                                    instructions.push(quote! {
                                        write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(#val))?;
                                    });
                                }
                                token_parser::AttributeValue::Dynamic(expr) => {
                                    instructions.push(quote! {
                                              write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(&#expr))?;
                                          });
                                }
                                _ => {}
                            },
                        }
                        continue;
                    }
                    match &attr.value {
                            token_parser::AttributeValue::Static(val) => {
                                instructions.push(quote! {
                                    write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(#val))?;
                                });
                            }
                            token_parser::AttributeValue::Dynamic(expr) => {
                                instructions.push(quote! {
                                    write!(#f_ident, " {}=\"{}\"", #attr_name, azumi::Escaped(&#expr))?;
                                });
                            }
                            token_parser::AttributeValue::None => {
                                instructions.push(quote! {
                                    write!(#f_ident, " {}", #attr_name)?;
                                });
                            }
                            _ => {}
                    }
                }

                if let Some(sid) = &ctx.scope_id {
                    instructions.push(quote! {
                        write!(#f_ident, " data-{}=\"{}\"", #sid, #sid)?;
                    });
                }

                instructions.push(quote! {
                   write!(#f_ident, ">")?;
                });

                let child_ctx = ctx.with_mode(if name == "script" {
                    Context::Script
                } else if name == "style" {
                    Context::Style
                } else {
                    ctx.mode
                });
                instructions
                    .push(generate_body_with_context(&elem.children, &child_ctx, f_ident));

                if !token_parser::VOID_ELEMENTS.contains(&name.as_str()) {
                    instructions.push(quote! {
                        write!(#f_ident, "</{}>", #name)?;
                    });
                }
            }
            token_parser::Node::Expression(expr) => {
                let tokens = &expr.content;
                match ctx.mode {
                    Context::Script => {
                        instructions.push(quote! {
                            write!(#f_ident, "{}", azumi::escape_script_content(&(#tokens)))?;
                        });
                    }
                    Context::Style => {
                        instructions.push(quote! {
                            write!(#f_ident, "{}", azumi::escape_style_content(&(#tokens)))?;
                        });
                    }
                    Context::Normal => {
                        instructions.push(quote! {
                            azumi::RenderWrapper(&(#tokens)).render_azumi(#f_ident)?;
                        });
                    }
                }
            }
            token_parser::Node::Fragment(frag) => {
                instructions.push(generate_body_with_context(&frag.children, ctx, f_ident));
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let cond = &if_block.condition;
                    let then_body =
                        generate_body_with_context(&if_block.then_branch, ctx, f_ident);
                    let else_part = if let Some(else_branch) = &if_block.else_branch {
                        let else_body =
                            generate_body_with_context(else_branch, ctx, f_ident);
                        quote! { else { #else_body } }
                    } else {
                        quote! {}
                    };

                    instructions.push(quote! {
                        if #cond {
                            #then_body
                        } #else_part
                    });
                }
                token_parser::Block::For(for_block) => {
                    let pat = &for_block.pattern;
                    let iter = &for_block.iterator;
                    let body = generate_body_with_context(&for_block.body, ctx, f_ident);

                    instructions.push(quote! {
                        for #pat in #iter {
                            #body
                        }
                    });
                }
                token_parser::Block::Match(match_block) => {
                    let expr = &match_block.expr;
                    let mut arms = Vec::new();
                    for arm in &match_block.arms {
                        let pat = &arm.pattern;
                        let body = generate_body_with_context(&arm.body, ctx, f_ident);
                        arms.push(quote! {
                            #pat => { #body }
                        });
                    }
                    instructions.push(quote! {
                        match #expr {
                            #(#arms),*
                        }
                    });
                }
                token_parser::Block::Call(call_block) => {
                    let func_path = &call_block.name;
                    let func_mod_path = resolve_component_path(func_path);

                    let args_list = match parse_args(call_block.args.clone()) {
                        Ok(a) => a,
                        Err(e) => {
                            instructions.push(e.to_compile_error());
                            Vec::new()
                        }
                    };

                    let setters = args_list.iter().map(|arg| {
                        let key = &arg.key;
                        let val = &arg.value;
                        quote! { .#key(#val) }
                    });

                    let builder_expr = quote! {
                        #func_mod_path::Props::builder()
                        #(#setters)*
                        .build()
                        .unwrap_or_else(|__e| panic!("Failed to build props for component {}: {}", stringify!(#func_mod_path), __e))
                    };

                    if call_block.children.is_empty() {
                        instructions.push(quote! {
                            azumi::RenderWrapper(&#func_mod_path::render(#builder_expr)).render_azumi(#f_ident)?;
                        });
                    } else {
                        let children_body =
                            generate_body_with_context(&call_block.children, ctx, f_ident);
                        let children_arg = quote! {
                            azumi::from_fn_once(move |#f_ident| {
                                #children_body
                                Ok(())
                            })
                        };

                        instructions.push(quote! {
                            azumi::RenderWrapper(&#func_mod_path::render(#builder_expr, #children_arg)).render_azumi(#f_ident)?;
                        });
                    }
                }
                token_parser::Block::Let(let_block) => {
                    let pat = &let_block.pattern;
                    let val = &let_block.value;
                    instructions.push(quote! {
                        let #pat = #val;
                    });
                }
                token_parser::Block::Style(_) => {
                    // Handled in hoisting pass
                }
                _ => {}
            },
            _ => {}
        }
    }

    quote! {
        #(#instructions)*
    }
}
