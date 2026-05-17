use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, LitStr};

pub fn expand_page(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_block = &input.block;
    let fn_sig = &input.sig;

    // Parse optional route attribute: #[azumi::page(route = "/about")]
    let route_value: Option<String> = if !attr.is_empty() {
        let attr_stream: proc_macro2::TokenStream = attr.into();
        match syn::parse2::<RouteAttrs>(attr_stream) {
            Ok(attrs) => attrs.route,
            Err(e) => {
                return syn::Error::new(
                    e.span(),
                    format!("expected #[azumi::page(route = \"/path\")], got: {e}"),
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        None
    };

    // 1. Infer Title from Function Name
    // lesson_9 -> "Lesson 9", my_html_page -> "My HTML Page"
    let name_str = fn_name.to_string();
    let title = name_str
        .split('_')
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => {
                    let rest: String = c.collect();
                    // Preserve known acronyms (all-uppercase words of 2+ chars)
                    let word = if !rest.is_empty() && rest.chars().all(|ch| ch.is_uppercase() || ch.is_numeric()) {
                        format!("{}{}", f.to_uppercase(), rest)
                    } else {
                        f.to_uppercase().to_string() + &rest
                    };
                    word
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    // 2. Infer Description from Doc Comments
    // /// This is a description
    let mut description = String::new();
    for attr in &input.attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &meta.value
                {
                    let val = s.value();
                    let trimmed = val.trim();
                    if !description.is_empty() {
                        description.push(' ');
                    }
                    description.push_str(trimmed);
                }
            }
        }
    }

    let desc_tokens = if description.is_empty() {
        quote! { None }
    } else {
        quote! { Some(#description.to_string()) }
    };

    // 3. Generate Wrapper
    let inner_name = format_ident!("_inner_{}", fn_name);
    let mut inner_sig = fn_sig.clone();
    inner_sig.ident = inner_name.clone();

    // 4. Generate route constant if route is provided
    let route_const = if let Some(ref route) = route_value {
        let const_name = format_ident!("{}_ROUTE", fn_name);
        let route_str = route.as_str();
        quote! {
            /// The URL path for this page.
            /// Use this constant in `html!` and Axum router setup
            /// to prevent route typos at compile time.
            pub const #const_name: &str = #route_str;
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        // Inner implementation
        #[azumi::component]
        #inner_sig {
            #fn_block
        }

        // Public Wrapper - uses from_fn to set metadata INSIDE render
        #fn_vis fn #fn_name() -> impl azumi::Component {
            let title = #title.to_string();
            let description = #desc_tokens;
            azumi::from_fn(move |f| {
                use azumi::Component;
                // Set context INSIDE render - guard lives through rendering
                let _guard = azumi::context::set_page_meta(
                    Some(title.clone()),
                    description.clone(),
                    None
                );
                // Render inner (which calls Layout, which calls seo::render_automatic_seo)
                #inner_name ().render(f)
            })
        }

        // Route constant — available as `page_fn::ROUTE`
        #route_const
    };

    TokenStream::from(expanded)
}

/// Parser for `route = "/path"` attribute syntax.
struct RouteAttrs {
    route: Option<String>,
}

impl syn::parse::Parse for RouteAttrs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut route = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            if ident != "route" {
                return Err(syn::Error::new(ident.span(), "expected `route`"));
            }
            input.parse::<syn::Token![=]>()?;
            let lit: LitStr = input.parse()?;
            route = Some(lit.value());

            // Allow trailing comma for future attrs
            if input.is_empty() {
                break;
            }
            let _comma: Option<syn::Token![,]> = input.parse().ok();
        }

        Ok(RouteAttrs { route })
    }
}
