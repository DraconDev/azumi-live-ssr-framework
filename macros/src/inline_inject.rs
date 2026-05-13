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
pub fn expand_json_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as JsonDataInput);
    
    let target = input.target;
    let value = input.value;
    
    let expanded = quote! {
        {
            let __json_value = serde_json::to_string(&#value)
                .unwrap_or_else(|e| {
                    panic!("json_data! failed to serialize variable '{}': {}", stringify!(#value), e)
                });
            // Escape </script> to prevent XSS (case-insensitive)
            let __escaped = azumi::escape_script_content(&__json_value);
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
