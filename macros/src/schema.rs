use heck::ToLowerCamelCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit};

#[cfg(feature = "schema")]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Extract @type from struct-level #[schema(type = "...")] attribute or use struct name
    let schema_type = extract_schema_type(&input.attrs, name.to_string());

    // Only work with named structs
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "Schema can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "Schema can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Process each field
    let mut field_serializations = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // Check for #[schema(skip)]
        if should_skip_field(&field.attrs) {
            continue;
        }

        // Get JSON key name (either from #[schema(name = "...")] or camelCase conversion)
        let json_key = extract_field_name(&field.attrs)
            .unwrap_or_else(|| field_name.to_string().to_lower_camel_case());

        // Generate serialization code based on type
        let serialization = generate_field_serialization(field_name, field_type, &json_key);

        field_serializations.push(serialization);
    }

    // Generate the implementation
    let expanded = quote! {
        impl azumi::Schema for #name {
            fn to_schema_script(&self) -> String {
                let json_value = self.to_schema_json_value();
                let json_string = serde_json::to_string_pretty(&json_value)
                    .expect("Failed to serialize schema to JSON");
                let escaped_json = json_string
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;");
                format!(
                    "<script type=\"application/ld+json\">\n{}\n</script>",
                    escaped_json
                )
            }

            fn to_schema_json_value(&self) -> serde_json::Value {
                use serde_json::Value;

                // Autoref specialization machinery
                struct Wrapper<'a, T: ?Sized>(&'a T);

                // Priority 1: Schema types (inherent method)
                impl<'a, T: azumi::Schema + ?Sized> Wrapper<'a, T> {
                    fn convert(&self) -> Value {
                        self.0.to_schema_json_value()
                    }
                }

                // Priority 2: Serialize types (trait method)
                trait Fallback {
                    fn convert(&self) -> Value;
                }

                impl<'a, T: serde::Serialize + ?Sized> Fallback for Wrapper<'a, T> {
                    fn convert(&self) -> Value {
                        serde_json::to_value(self.0).unwrap_or_else(|e| {
                            eprintln!("Azumi schema warning: failed to serialize field: {}", e);
                            Value::Null
                        })
                    }
                }

                let mut map = serde_json::Map::new();
                map.insert(
                    "@context".to_string(),
                    Value::String("https://schema.org".to_string())
                );
                map.insert(
                    "@type".to_string(),
                    Value::String(#schema_type.to_string())
                );

                #(#field_serializations)*

                Value::Object(map)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Extract schema type from #[schema(type = "...")] attribute
#[allow(dead_code)]
fn extract_schema_type(attrs: &[syn::Attribute], default: String) -> String {
    let mut schema_type = default;

    for attr in attrs {
        if !attr.path().is_ident("schema") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("type") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(lit) = s {
                    schema_type = lit.value();
                }
                return Ok(());
            }
            Ok(())
        });
    }
    schema_type
}

/// Check if field has #[schema(skip)]
#[allow(dead_code)]
fn should_skip_field(attrs: &[syn::Attribute]) -> bool {
    let mut skip = false;

    for attr in attrs {
        if !attr.path().is_ident("schema") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                skip = true;
            }
            Ok(())
        });
    }
    skip
}

/// Extract custom field name from #[schema(name = "...")]
#[allow(dead_code)]
fn extract_field_name(attrs: &[syn::Attribute]) -> Option<String> {
    let mut name = None;

    for attr in attrs {
        if !attr.path().is_ident("schema") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                let value = meta.value()?;
                let s: Lit = value.parse()?;
                if let Lit::Str(lit) = s {
                    name = Some(lit.value());
                }
                return Ok(());
            }
            Ok(())
        });
    }
    name
}

/// Check if a syn::Type is an Option<T>
#[allow(dead_code)]
fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(seg) = type_path.path.segments.last() {
            return seg.ident == "Option";
        }
    }
    false
}

/// Check if a syn::Type is a Vec<T>
#[allow(dead_code)]
fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(seg) = type_path.path.segments.last() {
            return seg.ident == "Vec";
        }
    }
    false
}

/// Generate field serialization code based on the type
#[allow(dead_code)]
fn generate_field_serialization(
    field_name: &syn::Ident,
    field_type: &syn::Type,
    json_key: &str,
) -> proc_macro2::TokenStream {
    // Handle Option<T>
    if is_option_type(field_type) {
        return quote! {
            if let Some(ref value) = self.#field_name {
                map.insert(
                    #json_key.to_string(),
                    Wrapper(value).convert()
                );
            }
        };
    }

    // Handle Vec<T>
    if is_vec_type(field_type) {
        return quote! {
            {
                let array: Vec<Value> = self.#field_name
                    .iter()
                    .map(|item| Wrapper(item).convert())
                    .collect();
                map.insert(
                    #json_key.to_string(),
                    Value::Array(array)
                );
            }
        };
    }

    // Default: direct serialization
    quote! {
        map.insert(
            #json_key.to_string(),
            Wrapper(&self.#field_name).convert()
        );
    }
}
