use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType, Type};

/// Returns true if the type is a `&T` or `&mut T` reference without an explicit lifetime annotation.
fn is_reference_without_lifetime(ty: &Type) -> bool {
    matches!(ty, Type::Reference(r) if r.lifetime.is_none())
}

/// If the type is a `&T` without an explicit lifetime, returns a clone with the given lifetime injected.
/// Otherwise, returns the type unchanged.
fn inject_lifetime(ty: &Type, lifetime: &syn::Lifetime) -> Type {
    if let Type::Reference(ref_type) = ty {
        if ref_type.lifetime.is_none() {
            let mut modified = ref_type.clone();
            modified.lifetime = Some(lifetime.clone());
            return Type::Reference(modified);
        }
    }
    ty.clone()
}

/// Choose a synthetic lifetime name that doesn't conflict with existing generics.
fn choose_lifetime_name(generics: &syn::Generics) -> syn::Lifetime {
    let existing: Vec<String> = generics.lifetimes().map(|lt| lt.lifetime.ident.to_string()).collect();

    for candidate in ["a", "b", "c"] {
        if !existing.iter().any(|e| e == candidate) {
            return syn::Lifetime::new(
                &format!("'{}", candidate),
                proc_macro2::Span::call_site(),
            );
        }
    }

    syn::Lifetime::new("'azumi_lt", proc_macro2::Span::call_site())
}

/// Add a lifetime bound to the return type of the render function.
/// Only modifies `impl Trait` and `dyn Trait` return types.
fn add_lifetime_to_return_type(
    ret: &syn::ReturnType,
    lt: &syn::Lifetime,
) -> syn::ReturnType {
    match ret {
        syn::ReturnType::Default => ret.clone(),
        syn::ReturnType::Type(arrow, ty) => {
            let mut ty = ty.clone();
            match &mut *ty {
                Type::ImplTrait(impl_trait) => {
                    impl_trait.bounds.push(syn::TypeParamBound::Lifetime(lt.clone()));
                }
                Type::TraitObject(trait_obj) => {
                    trait_obj.bounds.push(syn::TypeParamBound::Lifetime(lt.clone()));
                }
                _ => {}
            }
            syn::ReturnType::Type(*arrow, ty)
        }
    }
}

pub fn expand_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_block = input.block;
    let fn_output = &input.sig.output;

    // --- Phase 1: Detect if any parameter has a reference type without explicit lifetime ---
    let mut needs_synthetic_lifetime = false;
    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            if let Pat::Ident(pat_ident) = &**pat {
                if pat_ident.ident != "children" && is_reference_without_lifetime(ty) {
                    needs_synthetic_lifetime = true;
                    break;
                }
            }
        }
    }

    // Choose synthetic lifetime and build modified generics
    let synthetic_lifetime: Option<syn::Lifetime> = if needs_synthetic_lifetime {
        Some(choose_lifetime_name(&input.sig.generics))
    } else {
        None
    };

    let modified_generics = if let Some(ref lt) = synthetic_lifetime {
        let mut generics = input.sig.generics.clone();
        generics.params.push(syn::GenericParam::Lifetime(
            syn::LifetimeParam::new(lt.clone()),
        ));
        generics
    } else {
        input.sig.generics.clone()
    };

    // Modified return type (with synthetic lifetime bound if needed)
    let render_output = if let Some(ref lt) = synthetic_lifetime {
        add_lifetime_to_return_type(fn_output, lt)
    } else {
        fn_output.clone()
    };

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

                // Compute effective type: inject synthetic lifetime if needed
                let effective_ty: Type = if let Some(ref lt) = synthetic_lifetime {
                    inject_lifetime(ty, lt)
                } else {
                    (**ty).clone()
                };

                props_fields.push(quote! {
                    pub #ident: #effective_ty
                });
                props_init.push(quote! {
                    let #ident = props.#ident;
                });

                // Builder logic
                if let Some(default) = default_value {
                    builder_fields.push(quote! {
                        #ident: Option<#effective_ty>
                    });
                    builder_init.push(quote! {
                        #ident: None
                    });
                    build_logic.push(quote! {
                        let #ident = self.#ident.unwrap_or_else(|| #default);
                    });
                } else {
                    builder_fields.push(quote! {
                        #ident: Option<#effective_ty>
                    });
                    builder_init.push(quote! {
                        #ident: None
                    });
                    build_logic.push(quote! {
                        let #ident = self.#ident.ok_or(concat!("Missing required field: ", stringify!(#ident)))?;
                    });
                }

                builder_setters.push(quote! {
                    pub fn #ident(mut self, value: #effective_ty) -> Self {
                        self.#ident = Some(value);
                        self
                    }
                });

                struct_fields.push(ident);
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = modified_generics.split_for_impl();

    // Check for live state parameter
    // Priority 1: explicit #[live_state] attribute on any parameter
    // Priority 2: fallback to parameter named "state" (backward compatibility)
    let mut live_state_ident = None;
    let mut live_state_type = None;

    // First pass: look for #[live_state] attribute
    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, attrs, .. }) = arg {
            let has_live_attr = attrs.iter().any(|a| a.path().is_ident("live_state"));
            if has_live_attr {
                if let Pat::Ident(pat_ident) = &**pat {
                    let inner_type: Box<Type> = match &**ty {
                        Type::Reference(ref_ty) => ref_ty.elem.clone(),
                        _ => ty.clone(),
                    };
                    live_state_ident = Some(&pat_ident.ident);
                    live_state_type = Some(inner_type);
                    break;
                }
            }
        }
    }

    // Second pass: fallback to "state" parameter name (backward compat)
    if live_state_ident.is_none() {
        if let Some(FnArg::Typed(PatType { pat, ty, .. })) = input.sig.inputs.first() {
            if let Pat::Ident(pat_ident) = &**pat {
                if pat_ident.ident == "state" {
                    let inner_type: Box<Type> = match &**ty {
                        Type::Reference(ref_ty) => ref_ty.elem.clone(),
                        _ => ty.clone(),
                    };
                    let is_primitive = if let Type::Path(type_path) = &*inner_type {
                        if let Some(seg) = type_path.path.segments.first() {
                            let name = seg.ident.to_string();
                            matches!(
                                name.as_str(),
                                "str" | "String" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
                                | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
                                | "f32" | "f64" | "bool" | "char"
                                | "Vec" | "HashMap" | "BTreeMap" | "Option" | "Result"
                            )
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    if !is_primitive {
                        live_state_ident = Some(&pat_ident.ident);
                        live_state_type = Some(inner_type);
                    }
                }
            }
        }
    }

    // Generate the output
    // Helper: generate scope div wrapper for live state components
    let scope_body = if let Some(state_ident) = live_state_ident {
        quote! {
            azumi::from_fn(move |f| {
                let scope_json = match <_ as azumi::LiveState>::try_to_scope(#state_ident) {
                    Ok(signed) => signed,
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!(
                            "⚠️  Azumi: Failed to serialize LiveState to JSON: {}. \
                            This usually means a field doesn't implement Serialize. \
                            Rendering scope with empty state. \
                            Check that all state fields implement serde::Serialize.",
                            e
                        );
                        azumi::security::sign_state("{}")
                    }
                };
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
            pub fn render #impl_generics (props: Props #ty_generics, children: #children_ty) #render_output #where_clause {
                #(#props_init)*
                #scope_body
            }
        }
    } else {
        quote! {
            pub fn render #impl_generics (props: Props #ty_generics) #render_output #where_clause {
                #(#props_init)*
                #scope_body
            }
        }
    };

    // Check if function name is snake_case
    let name_str = fn_name.to_string();
    let is_snake_case = name_str.chars().all(|c| c.is_lowercase() || c == '_' || c.is_ascii_digit());

    let (mod_name, wrapper_fn) = if is_snake_case {
        let mod_ident = syn::Ident::new(&name_str, fn_name.span());

        // Generate wrapper function for direct calls (e.g. @snake_case())
        // Note: This only works for components with no required props or children
        let wrapper = if has_children || !props_fields.is_empty() {
            // Don't generate wrapper if props/children are required
            quote! {}
        } else {
            quote! {
                #fn_vis fn #fn_name () -> impl azumi::Component {
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

            pub struct Props #modified_generics #where_clause {
                #(#props_fields),*
            }

            pub struct PropsBuilder #modified_generics #where_clause {
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
