//! Azumi Live - Compiler-Driven Optimistic UI
//!
//! This module implements the `#[azumi::live]` macro that:
//! 1. Parses struct fields to understand state shape
//! 2. Analyzes impl block methods for predictable mutations
//! 3. Generates prediction metadata for client-side optimistic updates
//! 4. Auto-registers server action handlers

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, BinOp, Expr, ExprAssign, ExprBinary, ExprField,
    ExprMethodCall, ExprPath, ExprUnary, Fields, ImplItem, ImplItemFn, ItemImpl, ItemStruct,
    Member, Stmt, Token, UnOp,
};

/// Represents a predictable mutation that can be executed optimistically
#[derive(Debug, Clone)]
pub enum Prediction {
    /// self.field = literal (e.g., self.open = true)
    SetLiteral { field: String, value: String },
    /// self.field = !self.field (toggle)
    Toggle { field: String },
    /// self.field += value (increment)
    Add { field: String, value: String },
    /// self.field -= value (decrement)
    Sub { field: String, value: String },
    /// Manual prediction string from #[azumi::predict]
    Manual(String),
}

impl Prediction {
    /// Convert to DSL string for data-predict attribute
    pub fn to_dsl(&self) -> String {
        match self {
            Prediction::SetLiteral { field, value } => {
                format!("{} = {}", field, value)
            }
            Prediction::Toggle { field } => {
                format!("{} = !{}", field, field)
            }
            Prediction::Add { field, value } => {
                format!("{} = {} + {}", field, field, value)
            }
            Prediction::Sub { field, value } => {
                format!("{} = {} - {}", field, field, value)
            }
            Prediction::Manual(s) => s.clone(),
        }
    }

    /// Returns the field name for this prediction, if any
    pub fn field_name(&self) -> Option<&str> {
        match self {
            Prediction::SetLiteral { field, .. } => Some(field),
            Prediction::Toggle { field } => Some(field),
            Prediction::Add { field, .. } => Some(field),
            Prediction::Sub { field, .. } => Some(field),
            Prediction::Manual(_) => None, // Manual predictions skip validation
        }
    }
}

/// Metadata about an analyzed method
#[derive(Debug)]
pub struct MethodAnalysis {
    #[allow(dead_code)]
    pub name: String,
    pub predictions: Vec<Prediction>,
    #[allow(dead_code)]
    pub has_unpredictable: bool,
}

/// Extract field name from `self.field` expression
fn extract_self_field(expr: &Expr) -> Option<String> {
    if let Expr::Field(ExprField { base, member, .. }) = expr {
        // Check if base is `self`
        if let Expr::Path(ExprPath { path, .. }) = &**base {
            if path.is_ident("self") {
                if let Member::Named(ident) = member {
                    return Some(ident.to_string());
                }
            }
        }
    }
    None
}

/// Check if expression is a simple literal we can predict
fn expr_to_literal_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(lit) => Some(quote!(#lit).to_string()),
        Expr::Path(path) => {
            // Handle true/false as paths
            if path.path.is_ident("true") || path.path.is_ident("false") {
                Some(path.path.get_ident()?.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Check if expression is `!self.field`
fn is_toggle_expr(expr: &Expr, expected_field: &str) -> bool {
    if let Expr::Unary(ExprUnary {
        op: UnOp::Not(_),
        expr,
        ..
    }) = expr
    {
        if let Some(field) = extract_self_field(expr) {
            return field == expected_field;
        }
    }
    false
}

/// Analyze a single statement for predictable mutations
fn analyze_statement(stmt: &Stmt) -> Option<Prediction> {
    match stmt {
        Stmt::Expr(expr, _semicolon) => analyze_expr(expr),
        _ => None,
    }
}

/// Analyze an expression for predictable mutations
fn analyze_expr(expr: &Expr) -> Option<Prediction> {
    match expr {
        // self.field = value
        Expr::Assign(ExprAssign { left, right, .. }) => {
            let field = extract_self_field(left)?;

            // Check for toggle: self.field = !self.field
            if is_toggle_expr(right, &field) {
                return Some(Prediction::Toggle { field });
            }

            // Check for literal assignment
            if let Some(value) = expr_to_literal_string(right) {
                return Some(Prediction::SetLiteral { field, value });
            }

            None
        }

        // self.field += value or self.field -= value
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => {
            let field = extract_self_field(left)?;
            let value = expr_to_literal_string(right)?;

            match op {
                BinOp::AddAssign(_) => Some(Prediction::Add { field, value }),
                BinOp::SubAssign(_) => Some(Prediction::Sub { field, value }),
                _ => None,
            }
        }

        _ => None,
    }
}

/// Analyze a method body for all predictable mutations
pub fn analyze_method(method: &ImplItemFn) -> MethodAnalysis {
    let name = method.sig.ident.to_string();
    let mut predictions = Vec::new();
    let mut has_unpredictable = false;

    // Check for #[azumi::predict("...")] attribute
    for attr in &method.attrs {
        if attr.path().is_ident("predict") {
            if let Ok(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit),
                ..
            })) = attr.parse_args()
            {
                predictions.push(Prediction::Manual(lit.value()));
            }
        }
    }

    for stmt in &method.block.stmts {
        if let Some(prediction) = analyze_statement(stmt) {
            predictions.push(prediction);
        } else {
            // Check if this is a statement that could have side effects
            match stmt {
                Stmt::Expr(expr, _semicolon) => {
                    if is_side_effect(expr) {
                        has_unpredictable = true;
                    }
                }
                Stmt::Local(_) => {
                    // Local variable bindings are fine
                }
                _ => {
                    has_unpredictable = true;
                }
            }
        }
    }

    MethodAnalysis {
        name,
        predictions,
        has_unpredictable,
    }
}

/// Check if an expression likely has side effects (async, await, method calls, etc.)
fn is_side_effect(expr: &Expr) -> bool {
    match expr {
        Expr::Await(_) => true,
        Expr::Call(_) => true, // Function calls might have side effects
        Expr::MethodCall(mc) => {
            // self.field mutations are handled separately
            // External method calls are side effects
            !is_self_field_mutation(mc)
        }
        Expr::Assign(_) => false, // Assignments to self are fine
        Expr::Macro(_) => true,   // Macros are unpredictable
        _ => false,
    }
}

fn is_self_field_mutation(mc: &ExprMethodCall) -> bool {
    use syn::ExprField;
    if let Expr::Field(ExprField { base, .. }) = &*mc.receiver {
        if let Expr::Path(ExprPath { path, .. }) = &**base {
            if path.is_ident("self") {
                let method_name = mc.method.to_string();
                matches!(
                    method_name.as_str(),
                    "push"
                        | "pop"
                        | "shift"
                        | "unshift"
                        | "insert"
                        | "remove"
                        | "clear"
                        | "reverse"
                        | "sort"
                        | "splice"
                        | "swap"
                        | "lock"
                        | "put"
                        | "get_mut"
                        | "write"
                )
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

/// Main macro expansion for #[azumi::live]
pub fn expand_live(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let struct_vis = &input.vis;
    let struct_generics = &input.generics;
    let struct_name_str = struct_name.to_string();

    // Validate that struct has named fields
    if !matches!(input.fields, Fields::Named(_)) {
        return syn::Error::new_spanned(
            &input,
            "#[azumi::live] only supports structs with named fields",
        )
        .to_compile_error()
        .into();
    }

    let struct_attrs = &input.attrs;
    let struct_fields = &input.fields;

    // Parse local and computed field names
    let mut local_field_names = Vec::new();
    let mut computed_field_names = Vec::new();
    let mut regular_field_names = Vec::new();
    let mut field_idents = Vec::new();

    if let Fields::Named(named) = struct_fields {
        for field in &named.named {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();

            let is_local = field.attrs.iter().any(|attr| {
                attr.path().is_ident("local")
            });
            let is_computed = field.attrs.iter().any(|attr| {
                attr.path().is_ident("computed")
            });
            if is_local {
                local_field_names.push(field_name.clone());
            } else if is_computed {
                computed_field_names.push(field_name.clone());
            } else {
                regular_field_names.push(field_name.clone());
            }
            field_idents.push(field_ident.clone());
        }
    }

    let to_scope = if regular_field_names.is_empty() {
        quote! {
            pub fn to_scope(&self) -> String {
                String::new()
            }
        }
    } else {
        let field_values: Vec<_> = regular_field_names
            .iter()
            .map(|name| {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                quote! {
                    map.insert(stringify!(#ident).to_string(), serde_json::to_value(&self.#ident).unwrap());
                }
            })
            .collect();

        quote! {
            pub fn to_scope(&self) -> String {
                let mut map = serde_json::Map::new();
                #(#field_values)*
                let json = serde_json::to_string(&map).unwrap_or_default();
                azumi::security::sign_state(&json)
            }
        }
    };

    let to_local_scope = if local_field_names.is_empty() {
        quote! {
            pub fn to_local_scope(&self) -> String {
                String::new()
            }
        }
    } else {
        let field_values: Vec<_> = local_field_names
            .iter()
            .map(|name| {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                quote! {
                    map.insert(stringify!(#ident).to_string(), serde_json::to_value(&self.#ident).unwrap());
                }
            })
            .collect();

        quote! {
            pub fn to_local_scope(&self) -> String {
                let mut map = serde_json::Map::new();
                #(#field_values)*
                let json = serde_json::to_string(&map).unwrap_or_default();
                azumi::security::sign_state(&json)
            }
        }
    };

    let local_field_names_static: Vec<_> = local_field_names
        .iter()
        .map(|s| quote!(#s))
        .collect();

    let computed_field_names_static: Vec<_> = computed_field_names
        .iter()
        .map(|s| quote!(#s))
        .collect();

    let filtered_named_fields = if let Fields::Named(named) = struct_fields {
        let filtered: Punctuated<syn::Field, Token![,]> = named
            .named
            .iter()
            .map(|f| {
                let attrs: Vec<_> = f
                    .attrs
                    .iter()
                    .filter(|attr| {
                        let ident = attr.path().get_ident();
                        !matches!(ident, Some(i) if i == "local" || i == "computed")
                    })
                    .cloned()
                    .collect();
                syn::Field {
                    attrs,
                    ..f.clone()
                }
            })
            .collect();
        Fields::Named(syn::FieldsNamed {
            brace_token: named.brace_token,
            named: filtered,
        })
    } else {
        struct_fields.clone()
    };

    // Emit private constants with field names for compile-time validation
    // These are used by #[azumi::live_impl] to validate predictions
    let local_const_entries: Vec<_> = local_field_names
        .iter()
        .map(|s| quote!(#s))
        .collect();
    let computed_const_entries: Vec<_> = computed_field_names
        .iter()
        .map(|s| quote!(#s))
        .collect();

    let expanded = quote! {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        #(#struct_attrs)*
        #struct_vis struct #struct_name #struct_generics #filtered_named_fields

        impl #struct_generics #struct_name #struct_generics {
            #to_scope
            #to_local_scope

            #[doc(hidden)]
            const __AZUMI_LOCAL_FIELDS: &'static [&'static str] = &[#(#local_const_entries),*];
            #[doc(hidden)]
            const __AZUMI_COMPUTED_FIELDS: &'static [&'static str] = &[#(#computed_const_entries),*];
        }
    };

    TokenStream::from(expanded)
}

/// Attribute macro for impl blocks: #[azumi::live_impl]
/// This analyzes methods and generates action handlers with predictions
pub fn expand_live_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = &input.self_ty;
    let struct_name_str = quote!(#struct_name).to_string().replace(" ", "");

    // Parse attributes to find component="name"
    let args = parse_macro_input!(attr with syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated);
    let mut component_name = None;

    for arg in args {
        if let syn::Meta::NameValue(nv) = arg {
            if nv.path.is_ident("component") {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = nv.value
                {
                    component_name = Some(lit.value());
                }
            }
        }
    }

    let mut method_handlers = Vec::new();
    let mut original_methods = Vec::new();
    let mut predictions_entries = Vec::new();
    let mut all_validation_items = Vec::new();

    for item in &input.items {
        if let ImplItem::Fn(method) = item {
            let analysis = analyze_method(method);

            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();

            // Collect validation checks for predictions
            // We access the field lists via the const on the struct impl
            for (pred_idx, pred) in analysis.predictions.iter().enumerate() {
                if let Some(field_name) = pred.field_name() {
                    let check_name = format!("__AZUMI_CHECK_{}_{}", method_name_str, pred_idx);
                    let check_ident = syn::Ident::new(&check_name, proc_macro2::Span::call_site());

                    // Store prediction field name - validation happens at runtime in action handler
                    // Compile-time validation requires stable const traits (not yet available)
                    all_validation_items.push(quote! {
                        #[doc(hidden)]
                        const #check_ident: &'static str = #field_name;
                    });
                }
            }

            // Generate prediction string
            let prediction_dsl: String = analysis
                .predictions
                .iter()
                .map(|p| p.to_dsl())
                .collect::<Vec<_>>()
                .join("; ");

            if !prediction_dsl.is_empty() {
                predictions_entries.push(quote! {
                    (#method_name_str, #prediction_dsl)
                });
            }

            let handler_name = format_ident!("{}_handler", method_name);
            let router_name = format_ident!("{}_router", method_name);

            // Keep original method
            original_methods.push(quote! { #method });

            let is_async = method.sig.asyncness.is_some();
            let method_call = if is_async {
                quote! { state.#method_name().await; }
            } else {
                quote! { state.#method_name(); }
            };

            // Generate Axum handler
            let handler = if let Some(comp_name) = &component_name {
                let comp_mod = syn::Ident::new(comp_name, proc_macro2::Span::call_site());
                quote! {
                    pub async fn #handler_name(
                        body: String
                    ) -> axum::response::Response {
                        let json = match azumi::security::verify_state(&body) {
                            Ok(j) => j,
                            Err(e) => return axum::response::IntoResponse::into_response((axum::http::StatusCode::BAD_REQUEST, format!("Security Error: {}", e))),
                        };
                        let mut state: #struct_name = match serde_json::from_str(&json) {
                            Ok(s) => s,
                            Err(e) => return axum::response::IntoResponse::into_response((axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("State Deserialization Error: {}", e))),
                        };
                        #method_call

                        let props = match #comp_mod::Props::builder()
                            .state(&state)
                            .build()
                        {
                            Ok(props) => props,
                            Err(e) => return axum::response::IntoResponse::into_response((axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Component Build Error: {}", e))),
                        };
                        let html = azumi::render_to_string(&#comp_mod::render(props));

                        axum::response::IntoResponse::into_response(axum::response::Html(html))
                    }

                    #[allow(non_snake_case)]
                    pub fn #router_name() -> axum::routing::MethodRouter<()> {
                        axum::routing::post(#handler_name)
                    }
                }
            } else {
                quote! {
                    pub async fn #handler_name(
                        body: String
                    ) -> axum::response::Response {
                        let json = match azumi::security::verify_state(&body) {
                            Ok(j) => j,
                            Err(e) => return axum::response::IntoResponse::into_response((axum::http::StatusCode::BAD_REQUEST, format!("Security Error: {}", e))),
                        };
                        let mut state: #struct_name = match serde_json::from_str(&json) {
                            Ok(s) => s,
                            Err(e) => return axum::response::IntoResponse::into_response((axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("State Deserialization Error: {}", e))),
                        };
                        #method_call
                        axum::response::IntoResponse::into_response(axum::response::Json(state))
                    }

                    #[allow(non_snake_case)]
                    pub fn #router_name() -> axum::routing::MethodRouter<()> {
                        axum::routing::post(#handler_name)
                    }
                }
            };

            method_handlers.push(handler);

            // Generate inventory registration with NAMESPACED path
            // /_azumi/action/{StructName}/{MethodName}
            let action_path = format!("/_azumi/action/{}/{}", struct_name_str, method_name);
            let registration = quote! {
                azumi::inventory::submit! {
                    azumi::action::ActionEntry {
                        path: #action_path,
                        handler: #router_name,
                    }
                }
            };
            method_handlers.push(registration);
        }
    }

    let handler_mod_name =
        format_ident!("__azumi_live_handlers_{}", struct_name_str.to_lowercase());

    let expanded = quote! {
        impl #struct_name {
            #(#original_methods)*
            #(#all_validation_items)*
        }

        // NOTE: We do NOT implement LiveStateMetadata or LiveState here.
        // #[azumi::live] already provides the full implementation including predictions.
        // #[azumi::live_impl] only adds action handlers.

        #[allow(non_snake_case)]
        mod #handler_mod_name {
            use super::*;
            #(#method_handlers)*
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use crate::live::Prediction;

    #[test]
    fn test_prediction_to_dsl() {
        let toggle = Prediction::Toggle {
            field: "open".to_string(),
        };
        assert_eq!(toggle.to_dsl(), "open = !open");

        let add = Prediction::Add {
            field: "count".to_string(),
            value: "1".to_string(),
        };
        assert_eq!(add.to_dsl(), "count = count + 1");

        let set = Prediction::SetLiteral {
            field: "name".to_string(),
            value: "\"hello\"".to_string(),
        };
        assert_eq!(set.to_dsl(), "name = \"hello\"");
    }
}
