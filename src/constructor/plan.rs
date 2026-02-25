use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, punctuated::Punctuated, token::Comma, Error, Expr, Field, Fields,
    GenericArgument, PathArguments, Token, Type, TypePath, TypeTuple,
};

pub(crate) struct ConstructorPlan {
    pub parameters: Vec<TokenStream>,
    pub pass_values: Vec<TokenStream>,
    pub shape: VariantShape,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum VariantShape {
    Unit,
    Tuple,
    Struct,
}

struct FieldData {
    pub name: Ident,
    pub field_type: Type,
    pub default: DefaultValue,
    pub into: bool,
    pub into_iter: Option<IntoIterKind>,
}

enum IntoIterKind {
    Inferred,
    Explicit(Box<Type>),
}

enum DefaultValue {
    None,
    Unit,
    PhantomData,
    Trait,
    CustomFunction(TokenStream),
}

pub(crate) fn build(fields: &Fields, is_const: bool) -> syn::Result<ConstructorPlan> {
    let punctuated = extract_punctuated_fields(fields);
    let field_data = collect_field_datas(&punctuated)?;

    if is_const {
        for field in &field_data {
            if matches!(field.default, DefaultValue::Trait) {
                return Err(Error::new_spanned(
                    &field.name,
                    "`default` cannot be used in constant constructors.",
                ));
            }

            if field.into {
                return Err(Error::new_spanned(
                    &field.name,
                    "`into` cannot be used in constant constructors.",
                ));
            }

            if field.into_iter.is_some() {
                return Err(Error::new_spanned(
                    &field.name,
                    "`into_iter` cannot be used in constant constructors.",
                ));
            }
        }
    }

    let shape = extract_shape(fields);
    let is_named = shape == VariantShape::Struct;
    let defaults = build_default_initializers(&field_data, is_named);
    let (parameters, pass_values) = build_constructor_parameters(field_data, defaults, is_named)?;

    Ok(ConstructorPlan {
        parameters,
        pass_values,
        shape,
    })
}

fn extract_punctuated_fields(fields: &Fields) -> Punctuated<Field, Comma> {
    match fields {
        Fields::Named(found) => found.named.clone(),
        Fields::Unnamed(found) => found.unnamed.clone(),
        Fields::Unit => Default::default(),
    }
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
    let (default, into, into_iter) = collect_field_options(field)?;

    Ok(FieldData {
        name: ident,
        field_type: ty,
        default,
        into,
        into_iter,
    })
}

fn collect_field_options(field: &Field) -> syn::Result<(DefaultValue, bool, Option<IntoIterKind>)> {
    let mut default_value = DefaultValue::None;
    let mut into = false;
    let mut into_iter = None;

    let mut seen_new_attribute = false;

    for attribute in &field.attrs {
        if !attribute.path().is_ident("new") {
            continue;
        }

        if seen_new_attribute {
            return Err(Error::new_spanned(
                attribute,
                "Multiple `#[new(...)]` attributes are not allowed.",
            ));
        }

        seen_new_attribute = true;

        let mut has_options = false;

        attribute.parse_nested_meta(|meta| {
            has_options = true;
            field_options_parser(meta, &mut default_value, &mut into, &mut into_iter)
        })?;

        if !has_options {
            return Err(Error::new_spanned(
                attribute,
                "Expected an argument in `#[new(...)]` attribute.",
            ));
        }
    }

    detect_automatic_defaults(&mut default_value, field);
    check_invalid_field_options(field, &default_value, into, &into_iter)?;
    Ok((default_value, into, into_iter))
}

fn field_options_parser(
    meta: ParseNestedMeta<'_>,
    default_value: &mut DefaultValue,
    into: &mut bool,
    into_iter: &mut Option<IntoIterKind>,
) -> syn::Result<()> {
    if meta.path.is_ident("default") {
        return parse_default(meta, default_value);
    }

    if meta.path.is_ident("into") {
        return parse_into(meta, into);
    }

    if meta.path.is_ident("into_iter") {
        return parse_into_iter(meta, into_iter);
    }

    Err(meta.error("Unknown argument in `#[new(...)]` for field. Expected one of: `default`, `into`, `into_iter`."))
}

fn parse_default(meta: ParseNestedMeta<'_>, default_value: &mut DefaultValue) -> syn::Result<()> {
    if !matches!(default_value, DefaultValue::None) {
        return Err(meta.error("`default` specified more than once in `#[new(...)]`."));
    }

    // #[new(default)]
    if !meta.input.peek(Token![=]) {
        *default_value = DefaultValue::Trait;
        return Ok(());
    }

    // #[new(default = ...)]
    meta.input.parse::<Token![=]>()?;
    let default_expression = meta.input.parse::<Expr>()?;
    *default_value = DefaultValue::CustomFunction(default_expression.into_token_stream());
    Ok(())
}

fn parse_into(meta: ParseNestedMeta<'_>, into: &mut bool) -> syn::Result<()> {
    if *into {
        return Err(meta.error("`into` specified more than once in `#[new(...)]`."));
    }

    // #[new(into)]
    *into = true;
    Ok(())
}

fn parse_into_iter(
    meta: ParseNestedMeta<'_>,
    into_iter: &mut Option<IntoIterKind>,
) -> syn::Result<()> {
    if into_iter.is_some() {
        return Err(meta.error("`into_iter` specified more than once in `#[new(...)]`."));
    }

    // #[new(into_iter)]
    if !meta.input.peek(Token![=]) {
        *into_iter = Some(IntoIterKind::Inferred);
        return Ok(());
    }

    // #[new(into_iter = ...)]
    meta.input.parse::<Token![=]>()?;
    let ty = meta.input.parse::<Type>()?;

    *into_iter = Some(IntoIterKind::Explicit(Box::new(ty)));
    Ok(())
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

fn is_phantom_data(ty: &Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };

    let segments = &path.segments;

    match segments.len() {
        // PhantomData<T>
        1 => segments[0].ident == "PhantomData",

        // core::marker::PhantomData<T>
        // std::marker::PhantomData<T>
        3 => {
            (segments[0].ident == "core" || segments[0].ident == "std")
                && segments[1].ident == "marker"
                && segments[2].ident == "PhantomData"
        }

        _ => false,
    }
}

fn check_invalid_field_options(
    field: &Field,
    default_value: &DefaultValue,
    into: bool,
    into_iter: &Option<IntoIterKind>,
) -> syn::Result<()> {
    if !matches!(default_value, DefaultValue::None) && into {
        return Err(Error::new_spanned(
            field,
            "`default` cannot be combined with `into` in `#[new(...)]`.",
        ));
    }

    if !matches!(default_value, DefaultValue::None) && into_iter.is_some() {
        return Err(Error::new_spanned(
            field,
            "`default` cannot be combined with `into_iter` in `#[new(...)]`.",
        ));
    }

    if into && into_iter.is_some() {
        return Err(Error::new_spanned(
            field,
            "`into` cannot be combined with `into_iter` in `#[new(...)]`.",
        ));
    }

    Ok(())
}

fn extract_shape(fields: &Fields) -> VariantShape {
    match fields {
        Fields::Unit => VariantShape::Unit,
        Fields::Unnamed(_) => VariantShape::Tuple,
        Fields::Named(_) => VariantShape::Struct,
    }
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
        Trait => Some(quote!(#name: ::core::default::Default::default())),
        CustomFunction(function) => Some(quote!(#name: #function)),
    }
}

fn build_unnamed_initializer(default: &DefaultValue) -> Option<TokenStream> {
    use DefaultValue::{CustomFunction, PhantomData, Trait, Unit};

    match default {
        DefaultValue::None => None,
        Unit => Some(quote!(())),
        PhantomData => Some(quote!(::core::marker::PhantomData)),
        Trait => Some(quote!(::core::default::Default::default())),
        CustomFunction(function) => Some(quote!(#function)),
    }
}

fn build_constructor_parameters(
    fields: Vec<FieldData>,
    defaults: Vec<Option<TokenStream>>,
    is_named: bool,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut parameters = Vec::new();
    let mut values = Vec::new();

    for (field, default) in fields.into_iter().zip(defaults) {
        let (parameter, pass_value) = build_constructor_argument(field, default, is_named)?;

        if let Some(stream) = parameter {
            parameters.push(stream);
        }

        values.push(pass_value);
    }

    Ok((parameters, values))
}

fn build_constructor_argument(
    field: FieldData,
    default: Option<TokenStream>,
    is_named: bool,
) -> syn::Result<(Option<TokenStream>, TokenStream)> {
    let FieldData {
        name,
        field_type,
        into,
        into_iter,
        ..
    } = field;

    match default {
        Some(token) => Ok((None, token)),
        None => {
            if into {
                let parameter = quote!(#name: impl ::core::convert::Into<#field_type>);

                let pass_value = if is_named {
                    quote!(#name: #name.into())
                } else {
                    quote!(#name.into())
                };

                Ok((Some(parameter), pass_value))
            } else if let Some(iter_kind) = into_iter {
                let inner_type = match iter_kind {
                    IntoIterKind::Inferred => extract_inner_type(&field_type)?,
                    IntoIterKind::Explicit(ty) => *ty,
                };

                let parameter = quote!(
                    #name: impl ::core::iter::IntoIterator<Item = #inner_type>
                );

                let pass_value = if is_named {
                    quote!(#name: #name.into_iter().collect())
                } else {
                    quote!(#name.into_iter().collect())
                };

                Ok((Some(parameter), pass_value))
            } else {
                let parameter = quote!(#name: #field_type);

                let pass_value = if is_named {
                    quote!(#name: #name)
                } else {
                    quote!(#name)
                };

                Ok((Some(parameter), pass_value))
            }
        }
    }
}

fn extract_inner_type(ty: &Type) -> syn::Result<Type> {
    let Type::Path(type_path) = ty else {
        return Err(Error::new_spanned(
            ty,
            "Could not infer iterator item type from field type. Use `#[new(into_iter = T)]` to specify the item type explicitly.",
        ));
    };

    let Some(segment) = type_path.path.segments.last() else {
        return Err(Error::new_spanned(
            ty,
            "Could not infer iterator item type from field type. Use `#[new(into_iter = T)]` to specify the item type explicitly.",
        ));
    };

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return Err(Error::new_spanned(
            ty,
            "Could not infer iterator item type from field type. Use `#[new(into_iter = T)]` to specify the item type explicitly.",
        ));
    };

    let Some(first) = args.args.first() else {
        return Err(Error::new_spanned(
            ty,
            "Could not infer iterator item type from field type. Use `#[new(into_iter = T)]` to specify the item type explicitly.",
        ));
    };

    let GenericArgument::Type(inner) = first else {
        return Err(Error::new_spanned(
            first,
            "Unsupported generic argument. Use `#[new(into_iter = T)]` to specify the iterator item type explicitly.",
        ));
    };

    Ok(inner.clone())
}
