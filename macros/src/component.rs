use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType, Type};

pub fn expand_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_block = input.block;
    let fn_output = &input.sig.output;

    // Parse arguments into props
    let mut props_fields = Vec::new();
    let mut props_init = Vec::new();
    let mut builder_fields = Vec::new();
    let mut builder_init = Vec::new();
    let mut builder_setters = Vec::new();
    let mut build_logic = Vec::new();
    let mut struct_fields = Vec::new();
    let mut has_children = false;
    let mut children_type = None;

    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, attrs, .. }) = arg {
            if let Pat::Ident(pat_ident) = &**pat {
                let ident = &pat_ident.ident;

                // Check if this is the special "children" parameter
                if ident == "children" {
                    has_children = true;
                    children_type = Some(ty.clone());
                    continue; // Don't add to Props
                }

                // Check for #[prop(default = ...)]
                let mut default_value = None;
                for attr in attrs {
                    if attr.path().is_ident("prop") {
                        if let Err(e) = attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("default") {
                                let value = meta.value()?;
                                let lit_str: syn::LitStr = value.parse()?;
                                default_value =
                                    Some(lit_str.parse::<syn::Expr>().map_err(|_| {
                                        meta.error("Invalid default value expression")
                                    })?);
                            }
                            Ok(())
                        }) {
                            let err = syn::Error::new_spanned(attr, format!("{}", e));
                            return proc_macro::TokenStream::from(err.to_compile_error());
                        }
                    }
                }

                props_fields.push(quote! {
                    pub #ident: #ty
                });
                props_init.push(quote! {
                    let #ident = props.#ident;
                });

                // Builder logic
                if let Some(default) = default_value {
                    builder_fields.push(quote! {
                        #ident: Option<#ty>
                    });
                    builder_init.push(quote! {
                        #ident: None
                    });
                    build_logic.push(quote! {
                        let #ident = self.#ident.unwrap_or_else(|| #default);
                    });
                } else {
                    builder_fields.push(quote! {
                        #ident: Option<#ty>
                    });
                    builder_init.push(quote! {
                        #ident: None
                    });
                    build_logic.push(quote! {
                        let #ident = self.#ident.ok_or(concat!("Missing required field: ", stringify!(#ident)))?;
                    });
                }

                builder_setters.push(quote! {
                    pub fn #ident(mut self, value: #ty) -> Self {
                        self.#ident = Some(value);
                        self
                    }
                });

                struct_fields.push(ident);
            }
        }
    }

    let fn_generics = &input.sig.generics;
    let (impl_generics, ty_generics, where_clause) = fn_generics.split_for_impl();

    // Check for live state parameter (first argument)
    // Only trigger for parameters named "state" with a reference to a user-defined type
    // This avoids false positives on primitive refs like &str
    let mut live_state_ident = None;
    let mut live_state_type = None;
    if let Some(FnArg::Typed(PatType { pat, ty, .. })) = input.sig.inputs.first() {
        if let Pat::Ident(pat_ident) = &**pat {
            // Only consider parameter if it's named "state"
            if pat_ident.ident == "state" {
                // We trust the trait bound to catch invalid types later
                // If 'state' doesn't implement LiveState, the generated code will fail to compile
                // which is better than silently verifying it.
                live_state_ident = Some(&pat_ident.ident);

                // Extract the inner type from references:
                // &T -> T, &mut T -> T, T -> T
                let inner_type: Box<Type> = match &**ty {
                    Type::Reference(ref_ty) => ref_ty.elem.clone(),
                    _ => ty.clone(),
                };
                live_state_type = Some(inner_type);
            }
        }
    }

    // Generate the output
    // Helper: generate scope div wrapper for live state components
    let scope_body = if let Some(state_ident) = live_state_ident {
        quote! {
            azumi::from_fn(move |f| {
                let scope_json = <_ as azumi::LiveState>::to_scope(#state_ident);
                let struct_name = <#live_state_type as azumi::LiveStateMetadata>::struct_name();
                let local_json = azumi::LiveState::to_local_scope(#state_ident);
                let predictions = <#live_state_type as azumi::LiveStateMetadata>::predictions();
                let predictions_json = serde_json::to_string(predictions).unwrap_or_default();
                write!(f, "<div az-scope=\"{}\" az-struct=\"{}\"", azumi::Escaped(&scope_json), azumi::Escaped(struct_name))?;
                if !local_json.is_empty() {
                    write!(f, " az-local-state=\"{}\"", azumi::Escaped(&local_json))?;
                }
                if !predictions_json.is_empty() && predictions_json != "[]" {
                    write!(f, " az-predictions=\"{}\"", azumi::Escaped(&predictions_json))?;
                }
                write!(f, " style=\"display: contents\">")?;
                let inner = #fn_block;
                inner.render(f)?;
                write!(f, "</div>")?;
                Ok(())
            })
        }
    } else {
        quote! { #fn_block }
    };

    let render_fn = if has_children {
        let children_ty = children_type.as_ref().unwrap();

        quote! {
            pub fn render #impl_generics (props: Props #ty_generics, children: #children_ty) #fn_output #where_clause {
                #(#props_init)*
                #scope_body
            }
        }
    } else {
        quote! {
            pub fn render #impl_generics (props: Props #ty_generics) #fn_output #where_clause {
                #(#props_init)*
                #scope_body
            }
        }
    };

    // Check if function name is snake_case
    let name_str = fn_name.to_string();
    let is_snake_case = name_str.chars().all(|c| c.is_lowercase() || c == '_');

    let (mod_name, wrapper_fn) = if is_snake_case {
        let mod_ident = syn::Ident::new(&name_str, fn_name.span());

        // Generate wrapper function for direct calls (e.g. @snake_case())
        // Note: This only works for components with no required props or children
        let wrapper = if has_children || !props_fields.is_empty() {
            // Don't generate wrapper if props/children are required
            quote! {}
        } else {
            quote! {
                #fn_vis fn #fn_name #fn_generics () -> impl azumi::Component #where_clause {
                    #mod_ident::render(#mod_ident::Props::builder().build().expect("Missing required props in wrapper call"))
                }
            }
        };

        (mod_ident, wrapper)
    } else {
        (fn_name.clone(), quote! {})
    };

    let expanded = quote! {
        #[allow(non_snake_case)]
        #fn_vis mod #mod_name {
            use super::*;
            use azumi::Component;

            pub struct Props #fn_generics #where_clause {
                #(#props_fields),*
            }

            pub struct PropsBuilder #fn_generics #where_clause {
                #(#builder_fields),*
            }

            impl #impl_generics Props #ty_generics #where_clause {
                pub fn builder() -> PropsBuilder #ty_generics {
                    PropsBuilder {
                        #(#builder_init),*
                    }
                }
            }

            impl #impl_generics PropsBuilder #ty_generics #where_clause {
                #(#builder_setters)*

                pub fn build(self) -> Result<Props #ty_generics, &'static str> {
                    #(#build_logic)*
                    Ok(Props {
                        #(#struct_fields: #struct_fields),*
                    })
                }
            }

            #render_fn
        }

        #wrapper_fn
    };

    proc_macro::TokenStream::from(expanded)
}
