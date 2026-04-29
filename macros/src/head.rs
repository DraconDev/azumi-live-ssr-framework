use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, Token};

struct HeadArgs {
    title: Option<Expr>,
    description: Option<Expr>,
    image: Option<Expr>,
    url: Option<Expr>,
    type_: Option<Expr>,
}

impl Parse for HeadArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = HeadArgs {
            title: None,
            description: None,
            image: None,
            url: None,
            type_: None,
        };

        while !input.is_empty() {
            let key_str = if input.peek(Token![type]) {
                input.parse::<Token![type]>()?;
                "type".to_string()
            } else {
                input.parse::<Ident>()?.to_string()
            };

            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }

            match key_str.as_str() {
                "title" => args.title = Some(value),
                "description" => args.description = Some(value),
                "image" => args.image = Some(value),
                "url" => args.url = Some(value),
                "type" => args.type_ = Some(value),
                _ => {
                    return Err(syn::Error::new(
                        input.span(),
                        "Unknown key. Supported keys: title, description, image, url, type",
                    ))
                }
            }
        }

        if args.title.is_none() {
            return Err(input.error("Missing required field: title"));
        }
        // Description is no longer strictly required by parser, as runtime can fallback

        Ok(args)
    }
}

pub fn expand_head(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as HeadArgs);

    let title = args.title.expect("head! macro requires title");

    let description = if let Some(d) = args.description {
        quote! { Some(#d) }
    } else {
        quote! { None }
    };

    let image = if let Some(i) = args.image {
        quote! { Some(#i) }
    } else {
        quote! { None }
    };

    let url = if let Some(u) = args.url {
        quote! { Some(#u) }
    } else {
        quote! { None }
    };

    let type_ = if let Some(t) = args.type_ {
        quote! { Some(#t) }
    } else {
        quote! { None }
    };

    let expanded = quote! {
        {
            azumi::seo::generate_head(
                #title,
                #description,
                #image,
                #url,
                #type_,
            )
        }
    };

    TokenStream::from(expanded)
}
