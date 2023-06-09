extern crate proc_macro;


use syn::{parse_macro_input, DeriveInput, Error, Type, Attribute, NestedMeta, Meta};
use syn::{Field, Data, Fields, DataStruct, Generics};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote_spanned, quote};
use syn::{Result, Token};
use syn::parse::{ParseStream};
use quote::ToTokens;

// Inspired by a part of SeaORM: https://github.com/SeaQL/sea-orm/blob/master/sea-orm-macros/src/derives/active_model.rs
#[proc_macro_derive(New, attributes(new))]
pub fn derive_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, generics, attrs, .. } = parse_macro_input!(input);

    derive_new_impl(ident, data, generics, attrs)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}


pub(crate) fn derive_new_impl(
    ident: Ident,
    data: Data,
    generics: Generics,
    attrs: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let (fields, is_named) = match &data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(named) => (named.named.clone(), true),
            Fields::Unnamed(unnamed) => (unnamed.unnamed.clone(), false),
            Fields::Unit => Default::default(),
        },
        _ => {
            return Ok(quote_spanned! {
                ident.span() => compile_error!("you can only derive New on structs");
            });
        }
    };

    let props = MainProps::from_attrs(&attrs)?;

    let fields_with_types_and_settings: Vec<(Ident, Type, DefaultValue)> = fields.iter()
        .enumerate()
        .map(|(index, field)| {
            let ident = field.ident.clone().unwrap_or_else(|| format_ident!("_{}", index));
            let typed_name = field.ty.clone();
            let default_value = read_default_value(field)?;
            Ok((ident, typed_name, default_value))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let defaults: Vec<_> = fields_with_types_and_settings.iter()
        .map(|(field_name, _, value)| {
            match value {
                DefaultValue::DefaultFunction(func) => {
                    if is_named {
                        Some(quote!(#field_name: (#func)))
                    } else {
                        Some(quote!(#func))
                    }
                }
                DefaultValue::Default => {
                    if is_named {
                        Some(quote!(#field_name: Default::default()))
                    } else {
                        Some(quote!(Default::default()))
                    }
                }
                _ => None
            }
        })
        .collect();

    let (value_in_constructor, pass_value ): (Vec<_>, Vec<_>) = fields_with_types_and_settings
        .into_iter()
        .zip(defaults)
        .map(|((field, field_type, _), default)| {
            match default {
                Some(token) if is_named => {
                    (None, token)
                }
                None => {
                    let field_with_type = quote!(#field: #field_type);
                    (Some(field_with_type), quote!(#field))
                }
                Some(token) => {
                    (None, token)
                }
            }
        }).unzip();

    let constructor_field: Vec<_> = value_in_constructor.into_iter().flatten().collect();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let new_name = props.rename_arg;
    let public = if props.public {
        quote!(pub)
    } else {
        quote!()
    };

    let constructor = if constructor_field.is_empty() &&
        pass_value.is_empty() &&
        !is_named {
        quote! {
            #public fn #new_name() -> Self {
                Self
            }
        }
    } else if is_named {
        quote! {
            #public fn #new_name(#(#constructor_field),*) -> Self {
                Self {
                    #(#pass_value),*
                }
            }
        }
    } else {
        quote! {
            #public fn #new_name(#(#constructor_field),*) -> Self {
                Self(#(#pass_value),* )
            }
        }
    };

    Ok(quote!(
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            #constructor
        }
    ))
}

#[derive(Debug)]
enum DefaultValue {
    None,
    Default,
    DefaultFunction(proc_macro2::TokenStream),
}

// generated by chat gpt - 4
fn read_default_value(field: &Field) -> Result<DefaultValue> {
    let mut default_value: DefaultValue = DefaultValue::None;

    for attribute in &field.attrs {
        if attribute.path.is_ident("new") {
            if let DefaultValue::None = default_value {
                let component_args = attribute.parse_args_with(|input: ParseStream| {
                    if input.is_empty() {
                        return Err(input.error("Expected an argument after #[new(...)]."));
                    }

                    let _default_keyword: Token![default] = input.parse()?;

                    if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?;

                        let default_expr: syn::Expr = input.parse()?;
                        Ok(DefaultValue::DefaultFunction(default_expr.into_token_stream()))
                    } else {
                        Ok(DefaultValue::Default)
                    }
                });

                match component_args {
                    Ok(value) => default_value = value,
                    Err(err) => return Err(err),
                }
            } else {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "Multiple #[new(...)] annotations found. Ensure the field has only one #[new(...)] annotation.",
                ));
            }
        }
    }

    if matches!(&default_value, DefaultValue::None) {
        if let syn::Type::Tuple(syn::TypeTuple { elems, .. }) = &field.ty {
            if elems.is_empty() {
                default_value = DefaultValue::Default;
            }
        } else if is_phantom_data(&field.ty) {
            default_value = DefaultValue::Default;
        }
    }


    Ok(default_value)
}

fn is_phantom_data(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        let segments = &path.segments;
        if let Some(last_segment) = segments.last() {
            if last_segment.ident == "PhantomData" {
                return true;
            }
            if last_segment.ident == "marker" {
                if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(inner_ty) = arg {
                            return is_phantom_data(inner_ty);
                        }
                    }
                }
            }
        }
    }

    false
}

struct MainProps {
    pub public: bool,
    pub rename_arg: Ident,
}

impl MainProps {
    // generated by chat gpt - 4
    fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let mut pub_flag = None;
        let mut constructor_name = None;

        for attr in attrs {
            if attr.path.is_ident("new") {
                let meta = attr.parse_meta()?;

                if let Meta::List(meta_list) = meta {
                    for item in meta_list.nested {
                        if let NestedMeta::Meta(Meta::NameValue(name_value)) = item {
                            if name_value.path.is_ident("pub") {
                                if pub_flag.is_some() {
                                    return Err(Error::new_spanned(
                                        name_value,
                                        "Duplicate 'pub' key found in #[new] attributes.",
                                    ));
                                }

                                if let syn::Lit::Bool(lit_bool) = name_value.lit {
                                    pub_flag = Some(lit_bool.value);
                                } else {
                                    return Err(Error::new_spanned(
                                        name_value.lit,
                                        "The 'pub' argument must be a boolean value.",
                                    ));
                                }
                            } else if name_value.path.is_ident("rename") {
                                if constructor_name.is_some() {
                                    return Err(Error::new_spanned(
                                        name_value,
                                        "Duplicate 'rename' key found in #[new] attributes.",
                                    ));
                                }

                                if let syn::Lit::Str(lit_str) = name_value.lit {
                                    constructor_name =
                                        Some(Ident::new(&lit_str.value(), lit_str.span()));
                                } else {
                                    return Err(Error::new_spanned(
                                        name_value.lit,
                                        "The 'rename' argument must be a string value.",
                                    ));
                                }
                            } else {
                                return Err(Error::new_spanned(
                                    name_value.path,
                                    "Unknown argument in #[new] attribute.",
                                ));
                            }
                        }
                    }
                } else {
                    return Err(Error::new_spanned(
                        meta,
                        "The #[new] attribute should be a list, e.g., #[new(pub = true, rename = \"new2\")]",
                    ));
                }
            }
        }

        Ok(Self {
            public: pub_flag.unwrap_or(true),
            rename_arg: constructor_name.unwrap_or_else(|| Ident::new("new", Span::call_site())),
        })
    }
}