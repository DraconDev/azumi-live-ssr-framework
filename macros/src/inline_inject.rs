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
        let target: LitStr = input.parse()?;
        let _eq = input.parse()?;
        let value = input.parse()?;

        let target_str = target.value();
        let lower = target_str.to_lowercase();
        if lower.contains("</script") || lower.contains("</style") {
            return Err(syn::Error::new(
                target.span(),
                "json_data! target name must not contain closing tag sequences"
            ));
        }

        let valid = target_str.split('.').all(|seg| {
            !seg.is_empty()
            && seg.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
            && seg.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
        });
        if !valid {
            return Err(syn::Error::new(
                target.span(),
                "json_data! target must be a valid JS identifier path (e.g., \"APP_DATA\", \"window.config\", or \"$selector\")"
            ));
        }

        Ok(JsonDataInput { target, _eq, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse2;

    fn parse_target(target_lit: &str) -> Result<JsonDataInput, syn::Error> {
        let tokens: proc_macro2::TokenStream = format!(r#""{}" = value"#, target_lit).parse().unwrap();
        parse2::<JsonDataInput>(tokens)
    }

    #[test]
    fn test_simple_identifier() {
        assert!(parse_target("APP_DATA").is_ok());
    }

    #[test]
    fn test_dotted_path() {
        assert!(parse_target("window.config").is_ok());
    }

    #[test]
    fn test_deep_dotted_path() {
        assert!(parse_target("a.b.c").is_ok());
    }

    #[test]
    fn test_underscore_prefix() {
        assert!(parse_target("__DATA__").is_ok());
    }

    #[test]
    fn test_dollar_prefix() {
        assert!(parse_target("$app").is_ok());
    }

    #[test]
    fn test_dollar_in_path() {
        assert!(parse_target("$.data").is_ok());
    }

    #[test]
    fn test_jquery_style() {
        assert!(parse_target("jQuery.fn").is_ok());
    }

    #[test]
    fn test_numeric_start_rejected() {
        assert!(parse_target("0data").is_err());
    }

    #[test]
    fn test_empty_segment_rejected() {
        assert!(parse_target("window..data").is_err());
    }

    #[test]
    fn test_dot_only_rejected() {
        assert!(parse_target(".").is_err());
    }

    #[test]
    fn test_hyphen_rejected() {
        assert!(parse_target("my-data").is_err());
    }

    #[test]
    fn test_closing_script_rejected() {
        assert!(parse_target("</script").is_err());
    }

    #[test]
    fn test_closing_style_mixed_case_rejected() {
        assert!(parse_target("</STYLE").is_err());
    }
}
