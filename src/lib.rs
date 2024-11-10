use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Sampleable)]
pub fn sampleable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => generate_struct_impl(name, &fields.named),
                _ => panic!("Only named fields are supported"),
            }
        },
        Data::Enum(_) => generate_enum_impl(name),
        _ => panic!("Only structs and enums are supported"),
    }
}

fn generate_struct_impl(name: &syn::Ident, fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>) -> TokenStream {
    let field_samples = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        
        quote! {
            #field_name: {
                let field_config = config.get(#field_name_str)
                    .and_then(|v| v.as_object())
                    .ok_or_else(|| format!("Missing or invalid configuration for field '{}'", #field_name_str))?;
                
                let generator_type = field_config.get("type")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| format!("Missing type for field '{}'", #field_name_str))?;

                use fake::Fake;
                use fake::faker::*;
                use fake::locales::EN;
                use rand::Rng;

                match generator_type {
                    "name" => {
                        let subtype = field_config.get("subtype").and_then(|v| v.as_str()).unwrap_or("name");
                        match subtype {
                            "first_name" => name::en::FirstName().fake::<String>(),
                            "last_name" => name::en::LastName().fake::<String>(),
                            "title" => name::en::Title().fake::<String>(),
                            "suffix" => name::en::Suffix().fake::<String>(),
                            "name" => name::en::Name().fake::<String>(),
                            "name_with_title" => name::en::NameWithTitle().fake::<String>(),
                            _ => return Err(format!("Unsupported name subtype: {}", subtype)),
                        }
                    },
                    "number" => {
                        let params = field_config.get("params").and_then(|v| v.as_object());
                        let is_float = params.and_then(|p| p.get("float")).and_then(|v| v.as_bool()).unwrap_or(false);

                        if is_float {
                            let min = params.and_then(|p| p.get("min")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let max = params.and_then(|p| p.get("max")).and_then(|v| v.as_f64()).unwrap_or(100.0);
                            let decimals = params.and_then(|p| p.get("decimals")).and_then(|v| v.as_u64()).unwrap_or(2);
                            format!("{:.*}", decimals as usize, rand::thread_rng().gen_range(min..=max))
                        } else {
                            let min = params.and_then(|p| p.get("min")).and_then(|v| v.as_i64()).unwrap_or(0);
                            let max = params.and_then(|p| p.get("max")).and_then(|v| v.as_i64()).unwrap_or(100);
                            rand::thread_rng().gen_range(min..=max).to_string()
                        }
                    },
                    "boolean" => {
                        let prob = field_config.get("params")
                            .and_then(|p| p.as_object())
                            .and_then(|p| p.get("true_probability"))
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.5);
                        rand::thread_rng().gen_bool(prob).to_string()
                    },
                    "uuid" => {
                        use uuid::Uuid;
                        Uuid::new_v4().to_string()
                    },
                    _ => return Err(format!("Unsupported type '{}' for field '{}'", generator_type, #field_name_str)),
                }
            }
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn sample_with_config(config: &::serde_json::Map<String, ::serde_json::Value>) -> Result<Self, String> {
                Ok(Self {
                    #(#field_samples),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

fn generate_enum_impl(name: &syn::Ident) -> TokenStream {
    let expanded = quote! {
        impl #name {
            pub fn sample_with_config(_config: &::serde_json::Map<String, ::serde_json::Value>) -> Result<Self, String> {
                Err("Enum sampling not yet implemented".to_string())
            }
        }
    };

    TokenStream::from(expanded)
}