extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, parse_macro_input, punctuated::Punctuated, token::Comma, Attribute,
    Data, DataStruct, DeriveInput, Error, Expr, Field, Fields, Generics, LitBool, LitStr, Token,
    Type, TypePath, TypeTuple,
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
        return Err(syn::Error::new_spanned(
            ident,
            "'New' can only be derived for structs",
        ));
    };

    let props = MainProps::from_attributes(&attributes)?;
    let new_name = props.rename;
    let public = if props.public { quote!(pub) } else { quote!() };
    let constant = if props.constant {
        quote!(const)
    } else {
        quote!()
    };

    let fields_with_types_and_settings = collect_field_datas(&fields)?;

    if props.constant {
        for field in &fields_with_types_and_settings {
            if field.into {
                return Err(syn::Error::new_spanned(
                    &field.name,
                    "'into' is not allowed in const constructors",
                ));
            }

            if matches!(field.default, DefaultValue::Trait) {
                return Err(syn::Error::new_spanned(
                    &field.name,
                    "Default::default() is not allowed in const constructors",
                ));
            }
        }
    }

    let defaults = build_default_initializers(&fields_with_types_and_settings, is_named);

    let (constructor_field, pass_value) =
        build_constructor_arguments(fields_with_types_and_settings, defaults, is_named);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let constructor = generate_constructor(
        is_named,
        constructor_field,
        pass_value,
        new_name,
        public,
        constant,
    );

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

struct FieldData {
    name: Ident,
    field_type: Type,
    default: DefaultValue,
    into: bool,
}

fn collect_field_datas(fields: &Punctuated<Field, Comma>) -> syn::Result<Vec<FieldData>> {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| collect_field_data(index, field))
        .collect()
}

fn collect_field_data(index: usize, field: &Field) -> syn::Result<FieldData> {
    let ident = field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("_{}", index));
    let ty = field.ty.clone();
    let (default, into) = read_field_settings(field)?;

    Ok(FieldData {
        name: ident,
        field_type: ty,
        default,
        into,
    })
}

fn build_default_initializers(
    field_specs: &[FieldData],
    is_named: bool,
) -> Vec<Option<TokenStream>> {
    field_specs
        .iter()
        .map(|field_data| build_default_initializer(field_data, is_named))
        .collect()
}

fn build_default_initializer(field: &FieldData, is_named: bool) -> Option<TokenStream> {
    let FieldData { name, default, .. } = field;

    if is_named {
        build_named_initializer(name, default)
    } else {
        build_unnamed_initializer(default)
    }
}

fn build_named_initializer(name: &Ident, default: &DefaultValue) -> Option<TokenStream> {
    use DefaultValue::{CustomFunction, PhantomData, Trait, Unit};

    match default {
        DefaultValue::None => None,
        Unit => Some(quote!(#name: ())),
        PhantomData => Some(quote!(#name: ::core::marker::PhantomData)),
        Trait => Some(quote!(#name: Default::default())),
        CustomFunction(function) => Some(quote!(#name: #function)),
    }
}

fn build_unnamed_initializer(default: &DefaultValue) -> Option<TokenStream> {
    use DefaultValue::{CustomFunction, PhantomData, Trait, Unit};

    match default {
        DefaultValue::None => None,
        Unit => Some(quote!(())),
        PhantomData => Some(quote!(::core::marker::PhantomData)),
        Trait => Some(quote!(Default::default())),
        CustomFunction(function) => Some(quote!(#function)),
    }
}

fn build_constructor_arguments(
    fields: Vec<FieldData>,
    defaults: Vec<Option<TokenStream>>,
    is_named: bool,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let (parameter, pass_value): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .zip(defaults)
        .map(|(field, default)| build_constructor_argument(field, default, is_named))
        .unzip();

    let constructor_field = parameter.into_iter().flatten().collect();

    (constructor_field, pass_value)
}

fn build_constructor_argument(
    field: FieldData,
    default: Option<TokenStream>,
    is_named: bool,
) -> (Option<TokenStream>, TokenStream) {
    let FieldData {
        name,
        field_type,
        into,
        ..
    } = field;

    match default {
        Some(token) => (None, token),
        None => {
            if into {
                let parameter = quote!(#name: impl ::core::convert::Into<#field_type>);

                let pass_value = if is_named {
                    quote!(#name: #name.into())
                } else {
                    quote!(#name.into())
                };

                (Some(parameter), pass_value)
            } else {
                let parameter = quote!(#name: #field_type);

                let pass_value = if is_named {
                    quote!(#name: #name)
                } else {
                    quote!(#name)
                };

                (Some(parameter), pass_value)
            }
        }
    }
}

fn generate_constructor(
    is_named: bool,
    constructor_field: Vec<TokenStream>,
    pass_value: Vec<TokenStream>,
    new_name: Ident,
    public: TokenStream,
    constant: TokenStream,
) -> TokenStream {
    if constructor_field.is_empty() && pass_value.is_empty() && !is_named {
        return quote! {
            #[must_use]
            #public #constant fn #new_name() -> Self {
                Self
            }
        };
    }

    if is_named {
        return quote! {
            #[must_use]
            #public #constant fn #new_name(#(#constructor_field),*) -> Self {
                Self {
                    #(#pass_value),*
                }
            }
        };
    }

    quote! {
        #[must_use]
        #public #constant fn #new_name(#(#constructor_field),*) -> Self {
            Self(#(#pass_value),* )
        }
    }
}

#[derive(Debug)]
enum DefaultValue {
    None,
    Unit,
    PhantomData,
    Trait,
    CustomFunction(TokenStream),
}

fn read_field_settings(field: &Field) -> syn::Result<(DefaultValue, bool)> {
    let mut default_value = DefaultValue::None;
    let mut into = false;

    let mut seen_new_attribute = false;

    for attribute in &field.attrs {
        if !attribute.path().is_ident("new") {
            continue;
        }

        if seen_new_attribute {
            return Err(syn::Error::new_spanned(
                attribute,
                "Multiple #[new(...)] attributes found. Ensure the field has only one #[new(...)] attribute.",
            ));
        }

        seen_new_attribute = true;

        let mut has_arguments = false;

        attribute.parse_nested_meta(|meta| {
            has_arguments = true;
            field_settings_parser(meta, &mut default_value, &mut into)
        })?;

        if !has_arguments {
            return Err(syn::Error::new_spanned(
                attribute,
                "Expected an argument in #[new(...)] attribute.",
            ));
        }
    }

    if into && !matches!(default_value, DefaultValue::None) {
        return Err(syn::Error::new_spanned(
            field,
            "'into' and 'default' cannot be combined in the same #[new(...)] attribute.",
        ));
    }

    detect_automatic_defaults(&mut default_value, field);
    Ok((default_value, into))
}

fn detect_automatic_defaults(default_value: &mut DefaultValue, field: &Field) {
    if !matches!(&default_value, DefaultValue::None) {
        return;
    }

    if is_phantom_data(&field.ty) {
        *default_value = DefaultValue::PhantomData;
        return;
    }

    let Type::Tuple(TypeTuple { elems, .. }) = &field.ty else {
        return;
    };

    if elems.is_empty() {
        *default_value = DefaultValue::Unit;
    }
}

fn field_settings_parser(
    meta: ParseNestedMeta<'_>,
    default_value: &mut DefaultValue,
    into: &mut bool,
) -> syn::Result<()> {
    if meta.path.is_ident("into") {
        if *into {
            return Err(meta.error("Duplicate 'into' key found in #[new(...)] attribute."));
        }

        *into = true;
        return Ok(());
    }

    if meta.path.is_ident("default") {
        if !matches!(default_value, DefaultValue::None) {
            return Err(meta.error("Duplicate 'default' key found in #[new(...)] attribute."));
        }

        if !meta.input.peek(Token![=]) {
            *default_value = DefaultValue::Trait;
            return Ok(());
        }

        meta.input.parse::<Token![=]>()?;
        let default_expression: Expr = meta.input.parse()?;
        *default_value = DefaultValue::CustomFunction(default_expression.into_token_stream());
        return Ok(());
    }

    Err(meta.error("Unknown argument found in #[new(...)] attribute."))
}

fn is_phantom_data(ty: &Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };

    let Some(last) = path.segments.last() else {
        return false;
    };

    return last.ident == "PhantomData";
}

struct MainProps {
    pub public: bool,
    pub rename: Ident,
    pub constant: bool,
}

impl MainProps {
    fn from_attributes(attributes: &[Attribute]) -> syn::Result<Self> {
        let mut public = None;
        let mut rename = None;
        let mut constant = None;

        let mut seen_new_attribute = false;

        for attribute in attributes {
            if !attribute.path().is_ident("new") {
                continue;
            }

            if seen_new_attribute {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "Multiple #[new(...)] attributes found. Ensure only one is used on the struct.",
                ));
            }

            seen_new_attribute = true;
            let mut has_arguments = false;

            attribute.parse_nested_meta(|meta| {
                has_arguments = true;
                main_props_parser(meta, &mut public, &mut rename, &mut constant)
            })?;

            if !has_arguments {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "Expected at least one argument inside #[new(...)] attribute.",
                ));
            }
        }

        Ok(Self {
            public: public.unwrap_or(true),
            rename: rename.unwrap_or_else(|| Ident::new("new", Span::call_site())),
            constant: constant.unwrap_or(false),
        })
    }
}

fn main_props_parser(
    meta: ParseNestedMeta<'_>,
    public: &mut Option<bool>,
    rename: &mut Option<Ident>,
    constant: &mut Option<bool>,
) -> syn::Result<()> {
    if meta.path.is_ident("pub") {
        if public.is_some() {
            return Err(meta.error("Duplicate 'pub' key found in #[new(...)] attribute."));
        }

        let value = meta.value()?;
        let lit: LitBool = value.parse()?;
        *public = Some(lit.value);
        return Ok(());
    }

    if meta.path.is_ident("rename") {
        if rename.is_some() {
            return Err(meta.error("Duplicate 'rename' key found in #[new(...)] attribute."));
        }

        let value = meta.value()?;
        let lit: LitStr = value.parse()?;
        *rename = Some(Ident::new(&lit.value(), lit.span()));
        return Ok(());
    }

    if meta.path.is_ident("const") {
        if constant.is_some() {
            return Err(meta.error("Duplicate 'const' key found in #[new(...)] attribute."));
        }

        let value = meta.value()?;
        let lit: LitBool = value.parse()?;
        *constant = Some(lit.value);
        return Ok(());
    }

    return Err(meta.error("Unknown argument in #[new(...)] attribute."));
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;
