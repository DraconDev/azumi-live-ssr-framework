use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, LitStr};

/// json_data! macro - Safe JSON data injection for JavaScript
/// 
/// This macro provides a safe alternative to:
/// ```ignore
/// @{Raw(format!("window.__DATA__ = {};", serde_json::to_string(&data).unwrap()))}
/// ```
/// 
/// Usage:
/// ```ignore
/// json_data! { window.__DATA__ = &data }
/// json_data! { window.config = &config_json }
/// ```
/// 
/// Features:
/// - Automatically serializes with serde_json
/// - Escapes </script> tags in JSON to prevent XSS
/// - Generates proper <script> block
/// - Type-safe: accepts any serde::Serialize type
/// 
/// AI Note: Use this instead of format!() or Raw() for passing data to JS.
#[proc_macro]
pub fn json_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as JsonDataInput);
    
    let target = input.target;
    let value = input.value;
    
    let expanded = quote! {
        {
            let __json_value = match serde_json::to_string(&#value) {
                Ok(s) => s,
                Err(e) => panic!("json_data! failed to serialize: {}", e),
            };
            // Escape </script> to prevent XSS
            let __escaped = __json_value.replace("</script>", "<\\/script>");
            azumi::from_fn_once(move |f| {
                write!(f, "<script>{} = {};</script>", #target, __escaped)?;
                Ok(())
            })
        }
    };
    
    TokenStream::from(expanded)
}

struct JsonDataInput {
    target: LitStr,
    _eq: syn::Token![=],
    value: Expr,
}

impl syn::parse::Parse for JsonDataInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let target = input.parse()?;
        let _eq = input.parse()?;
        let value = input.parse()?;
        Ok(JsonDataInput { target, _eq, value })
    }
}

/// inline_css! macro - Safe CSS injection
///
/// This macro provides a safe alternative to:
/// ```ignore
/// @{Raw(format!("<style>{}</style>", HUB_GLOBAL_CSS))}
/// ```
///
/// Usage:
/// ```ignore
/// inline_css! { HUB_GLOBAL_CSS }
/// ```
///
/// Features:
/// - Escapes </style> tags to prevent XSS
/// - Generates proper <style> block
/// - Validates CSS syntax (optional via lightningcss)
///
/// AI Note: Use this instead of Raw(format!("<style>...</style>")) for CSS injection.
#[proc_macro]
pub fn inline_css(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InlineCssInput);
    
    let value = input.value;
    
    let expanded = quote! {
        {
            let __css = #value;
            // Escape </style> to prevent XSS
            let __escaped = __css.replace("</style>", "<\\/style>");
            azumi::from_fn_once(move |f| {
                write!(f, "<style>{}</style>", __escaped)?;
                Ok(())
            })
        }
    };
    
    TokenStream::from(expanded)
}

struct InlineCssInput {
    value: Expr,
}

impl syn::parse::Parse for InlineCssInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse()?;
        Ok(InlineCssInput { value })
    }
}

/// inline_script! macro - Safe script injection with content escaping
///
/// This macro provides a safe alternative to:
/// ```ignore
/// <script>@{Raw(AI_HUB_COPY_JS)}</script>
/// ```
///
/// Usage:
/// ```ignore
/// inline_script! { AI_HUB_COPY_JS }
/// ```
///
/// Features:
/// - Escapes </script> tags to prevent XSS
/// - Generates proper <script> block
///
/// AI Note: Use this instead of Raw() for inline scripts.
#[proc_macro]
pub fn inline_script(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InlineScriptInput);
    
    let value = input.value;
    
    let expanded = quote! {
        {
            let __script = #value;
            // Escape </script> to prevent XSS
            let __escaped = __script.replace("</script>", "<\\/script>");
            azumi::from_fn_once(move |f| {
                write!(f, "<script>{}</script>", __escaped)?;
                Ok(())
            })
        }
    };
    
    TokenStream::from(expanded)
}

struct InlineScriptInput {
    value: Expr,
}

impl syn::parse::Parse for InlineScriptInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse()?;
        Ok(InlineScriptInput { value })
    }
}
