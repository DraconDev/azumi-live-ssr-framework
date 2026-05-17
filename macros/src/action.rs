use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Type};

fn is_extractor_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        // Only match unqualified types (single segment) to avoid false positives
        // on user-defined types with the same name (e.g., my_app::State)
        if type_path.path.segments.len() != 1 {
            return false;
        }
        if let Some(seg) = type_path.path.segments.first() {
            let name = seg.ident.to_string();
            return matches!(
                name.as_str(),
                "State" | "Extension" | "Form" | "Json" | "Path" | "Query"
            );
        }
    }
    false
}

fn is_action_result(return_type: &syn::ReturnType) -> bool {
    if let syn::ReturnType::Type(_, ty) = return_type {
        if let Type::Path(type_path) = ty.as_ref() {
            if let Some(seg) = type_path.path.segments.last() {
                return seg.ident == "ActionResult";
            }
        }
    }
    false
}

fn is_impl_component(return_type: &syn::ReturnType) -> bool {
    let syn::ReturnType::Type(_, ty) = return_type else {
        return false;
    };
    let Type::ImplTrait(impl_trait) = ty.as_ref() else {
        return false;
    };
    let Some(bounds) = impl_trait.bounds.first() else {
        return false;
    };
    let syn::TypeParamBound::Trait(trait_bound) = bounds else {
        return false;
    };
    let Some(seg) = trait_bound.path.segments.last() else {
        return false;
    };
    seg.ident == "Component"
}

pub fn expand_action(item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_async = &input_fn.sig.asyncness;
    let fn_args = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    let wrapper_name = quote::format_ident!("{}_handler", fn_name);
    let router_helper_name = quote::format_ident!("{}_router", fn_name);

    // Separate extractors from payloads
    let mut extractor_params = Vec::new();
    let mut payload_params = Vec::new();
    let mut call_args = Vec::new();

    for arg in fn_args.iter() {
        if let FnArg::Typed(pat_type) = arg {
            let pat = &pat_type.pat;
            let ty = &pat_type.ty;
            if is_extractor_type(ty) {
                // Pass through as-is
                extractor_params.push(quote! { #pat_type });
                call_args.push(quote! { #pat });
            } else {
                // Auto-wrap in Form<>
                payload_params.push(quote! {
                    axum::extract::Form(#pat): axum::extract::Form<#ty>
                });
                call_args.push(quote! { #pat });
            }
        }
    }

    // Determine how to handle the return type
    let return_handling = if is_action_result(fn_output) {
        quote! {
            let result = #fn_name(#(#call_args),*).await;
            use axum::response::IntoResponse;
            result.into_response()
        }
    } else if is_impl_component(fn_output) {
        quote! {
            let result = #fn_name(#(#call_args),*).await;
            axum::response::Html(azumi::render_to_string(&result))
        }
    } else {
        quote! {
            #fn_name(#(#call_args),*).await
        }
    };

    let path_const_name = quote::format_ident!("{}_PATH", fn_name);
    let action_path_str = format!("/_azumi/action/{}", fn_name);

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_async fn #fn_name(#fn_args) #fn_output {
            #fn_block
        }

        /// The URL path for this action.
        ///
        /// Use this constant in `html!` `az-action` attributes
        /// to prevent action URL typos at compile time.
        ///
        /// # Example
        ///
        /// ```rust,ignore
        /// #[azumi::action]
        /// fn like_post(form: LikeForm) -> ActionResult { ... }
        ///
        /// html! {
        ///     <form az-action={like_post_PATH} az-target={"#like-area"}>
        /// }
        /// ```
        #[allow(non_upper_case_globals)]
        pub const #path_const_name: &str = #action_path_str;

        pub async fn #wrapper_name(
            #(#extractor_params,)*
            #(#payload_params),*
        ) -> impl axum::response::IntoResponse {
            #return_handling
        }

        #[allow(non_snake_case)]
        pub fn #router_helper_name() -> axum::routing::MethodRouter<()> {
            axum::routing::post(#wrapper_name)
        }

        azumi::inventory::submit! {
            azumi::action::ActionEntry {
                path: #action_path_str,
                handler: #router_helper_name,
            }
        }
    };

    TokenStream::from(expanded)
}
