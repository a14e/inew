extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    meta::ParseNestedMeta, parse::ParseStream, parse_macro_input, punctuated::Punctuated,
    token::Comma, Attribute, Data, DataStruct, DeriveInput, Error, Expr, Field, Fields,
    GenericArgument, Generics, LitBool, LitStr, PathArguments, Token, Type, TypePath, TypeTuple,
};

// Inspired by a part of SeaORM: https://github.com/SeaQL/sea-orm/blob/master/sea-orm-macros/src/derives/active_model.rs
#[proc_macro_derive(New, attributes(new))]
pub fn derive_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    derive_new_impl(ident, data, generics, attrs)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_new_impl(
    ident: Ident,
    data: Data,
    generics: Generics,
    attributes: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let Some((fields, is_named)) = extract_fields(&data) else {
        return Ok(quote_spanned! {
            ident.span() => compile_error!("you can only derive New on structs");
        });
    };

    let props = MainProps::from_attributes(&attributes)?;
    let new_name = props.rename_arg;
    let public = if props.public { quote!(pub) } else { quote!() };

    let fields_with_types_and_settings = collect_field_datas(&fields)?;
    let defaults = build_default_initializers(&fields_with_types_and_settings, is_named);

    let (constructor_field, pass_value) =
        build_constructor_arguments(fields_with_types_and_settings, defaults);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let constructor =
        generate_constructor(is_named, constructor_field, pass_value, new_name, public);

    Ok(quote!(
        #[automatically_derived]
        impl #impl_generics #ident #type_generics #where_clause {
            #constructor
        }
    ))
}

fn extract_fields(data: &Data) -> Option<(Punctuated<Field, Comma>, bool)> {
    match &data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(found) => Some((found.named.clone(), true)),
            Fields::Unnamed(found) => Some((found.unnamed.clone(), false)),
            Fields::Unit => Some(Default::default()),
        },
        _ => None,
    }
}

fn collect_field_datas(
    fields: &Punctuated<Field, Comma>,
) -> syn::Result<Vec<(Ident, Type, DefaultValue)>> {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| collect_field_data(index, field))
        .collect()
}

fn collect_field_data(index: usize, field: &Field) -> syn::Result<(Ident, Type, DefaultValue)> {
    let ident = field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("_{}", index));
    let ty = field.ty.clone();
    let default = read_default_value(field)?;

    Ok((ident, ty, default))
}

fn build_default_initializers(
    field_specs: &[(Ident, Type, DefaultValue)],
    is_named: bool,
) -> Vec<Option<TokenStream>> {
    field_specs
        .iter()
        .map(|(field_name, _, value)| build_default_initializer(field_name, value, is_named))
        .collect()
}

fn build_default_initializer(
    field_name: &Ident,
    value: &DefaultValue,
    is_named: bool,
) -> Option<TokenStream> {
    match value {
        DefaultValue::CustomFunction(function) => {
            if is_named {
                Some(quote!(#field_name: (#function)))
            } else {
                Some(quote!(#function))
            }
        }
        DefaultValue::Trait => {
            if is_named {
                Some(quote!(#field_name: Default::default()))
            } else {
                Some(quote!(Default::default()))
            }
        }
        DefaultValue::None => None,
    }
}

fn build_constructor_arguments(
    field_specs: Vec<(Ident, Type, DefaultValue)>,
    defaults: Vec<Option<TokenStream>>,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let (value_in_constructor, pass_value): (Vec<_>, Vec<_>) = field_specs
        .into_iter()
        .zip(defaults)
        .map(|((field, field_type, _), default)| {
            build_constructor_argument(field, field_type, default)
        })
        .unzip();

    let constructor_field = value_in_constructor.into_iter().flatten().collect();

    (constructor_field, pass_value)
}

fn build_constructor_argument(
    field: Ident,
    field_type: Type,
    default: Option<TokenStream>,
) -> (Option<TokenStream>, TokenStream) {
    match default {
        Some(token) => (None, token),
        None => {
            let field_with_type = quote!(#field: #field_type);
            (Some(field_with_type), quote!(#field))
        }
    }
}

fn generate_constructor(
    is_named: bool,
    constructor_field: Vec<TokenStream>,
    pass_value: Vec<TokenStream>,
    new_name: Ident,
    public: TokenStream,
) -> TokenStream {
    if constructor_field.is_empty() && pass_value.is_empty() && !is_named {
        return quote! {
            #public fn #new_name() -> Self {
                Self
            }
        };
    }

    if is_named {
        return quote! {
            #public fn #new_name(#(#constructor_field),*) -> Self {
                Self {
                    #(#pass_value),*
                }
            }
        };
    }

    quote! {
        #public fn #new_name(#(#constructor_field),*) -> Self {
            Self(#(#pass_value),* )
        }
    }
}

#[derive(Debug)]
enum DefaultValue {
    None,
    Trait,
    CustomFunction(TokenStream),
}

fn read_default_value(field: &Field) -> syn::Result<DefaultValue> {
    let mut default_value = DefaultValue::None;

    for attribute in &field.attrs {
        if !attribute.path().is_ident("new") {
            continue;
        }

        let DefaultValue::None = default_value else {
            return Err(syn::Error::new_spanned(
                attribute,
                "Multiple #[new(...)] annotations found. Ensure the field has only one #[new(...)] annotation.",
            ));
        };

        let component_args = attribute.parse_args_with(component_args_parser);

        match component_args {
            Ok(value) => default_value = value,
            Err(error) => return Err(error),
        }
    }

    if matches!(&default_value, DefaultValue::None) {
        if let Type::Tuple(TypeTuple {
            elems: elements, ..
        }) = &field.ty
        {
            if elements.is_empty() {
                default_value = DefaultValue::Trait;
            }
        } else if is_phantom_data(&field.ty) {
            default_value = DefaultValue::Trait;
        }
    }

    Ok(default_value)
}

fn component_args_parser(input: ParseStream) -> syn::Result<DefaultValue> {
    if input.is_empty() {
        return Err(input.error("Expected an argument after #[new(...)]."));
    }

    let _default_keyword: Token![default] = input.parse()?;

    if !input.peek(Token![=]) {
        return Ok(DefaultValue::Trait);
    }

    input.parse::<Token![=]>()?;
    let default_expression: Expr = input.parse()?;

    Ok(DefaultValue::CustomFunction(
        default_expression.into_token_stream(),
    ))
}

fn is_phantom_data(ty: &Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };

    let Some(last_segment) = &path.segments.last() else {
        return false;
    };

    if last_segment.ident == "PhantomData" {
        return true;
    }

    if last_segment.ident != "marker" {
        return false;
    }

    let PathArguments::AngleBracketed(args) = &last_segment.arguments else {
        return false;
    };

    for argument in &args.args {
        if let GenericArgument::Type(inner_type) = argument {
            return is_phantom_data(inner_type);
        }
    }

    false
}

struct MainProps {
    pub public: bool,
    pub rename_arg: Ident,
}

impl MainProps {
    fn from_attributes(attributes: &[Attribute]) -> syn::Result<Self> {
        let mut public = None;
        let mut rename = None;

        for attribute in attributes {
            if !attribute.path().is_ident("new") {
                continue;
            }

            attribute
                .parse_nested_meta(|meta| main_props_parser(meta, &mut public, &mut rename))?;
        }

        Ok(Self {
            public: public.unwrap_or(true),
            rename_arg: rename.unwrap_or_else(|| Ident::new("new", Span::call_site())),
        })
    }
}

fn main_props_parser(
    meta: ParseNestedMeta<'_>,
    public: &mut Option<bool>,
    rename: &mut Option<Ident>,
) -> syn::Result<()> {
    if meta.path.is_ident("pub") {
        if public.is_some() {
            return Err(meta.error("Duplicate 'pub' key found in #[new] attributes."));
        }

        let value = meta.value()?;
        let lit: LitBool = value.parse()?;
        *public = Some(lit.value);
    } else if meta.path.is_ident("rename") {
        if rename.is_some() {
            return Err(meta.error("Duplicate 'rename' key found in #[new] attributes."));
        }

        let value = meta.value()?;
        let lit: LitStr = value.parse()?;
        *rename = Some(Ident::new(&lit.value(), lit.span()));
    } else {
        return Err(meta.error("Unknown argument in #[new] attribute."));
    }

    Ok(())
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;
