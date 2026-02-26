use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, ToTokens};
use syn::{
    meta::ParseNestedMeta, punctuated::Punctuated, token::Comma, Error, Expr, Field, Fields,
    GenericArgument, PathArguments, Token, Type, TypePath, TypeTuple,
};

pub(crate) struct ConstructorPlan {
    pub field_datas: Vec<FieldData>,
    pub shape: VariantShape,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum VariantShape {
    Unit,
    Tuple,
    Struct,
}

pub(crate) struct FieldData {
    pub name: Ident,
    pub field_type: Type,
    pub default: DefaultValue,
    pub optional: bool,
    pub into: bool,
    pub into_iter: Option<Type>,
}

enum IntoIterKind {
    Inferred,
    Explicit(Box<Type>),
}

pub(crate) enum DefaultValue {
    None,
    Unit,
    PhantomData,
    Trait,
    CustomExpression(TokenStream),
}

pub(crate) fn build(fields: &Fields, is_const: bool) -> syn::Result<ConstructorPlan> {
    let punctuated = extract_punctuated_fields(fields);
    let field_datas = collect_field_datas(&punctuated)?;

    if is_const {
        for field in &field_datas {
            if matches!(field.default, DefaultValue::Trait) {
                return Err(Error::new_spanned(
                    &field.name,
                    "`default` cannot be used in constant constructors.",
                ));
            }

            if field.optional && matches!(field.default, DefaultValue::None) {
                return Err(Error::new_spanned(
                &field.name,
                "`optional` without an explicit `default = ...` cannot be used in constant constructors.",
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
    Ok(ConstructorPlan { field_datas, shape })
}

fn is_auto_default_type(ty: &Type) -> bool {
    is_phantom_data(ty) || matches!(ty, Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty())
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
    let (default, optional, into, into_iter_kind) = collect_field_options(field)?;
    let into_iter = match into_iter_kind {
        Some(IntoIterKind::Inferred) => Some(extract_into_iter_type(&ty)?),
        Some(IntoIterKind::Explicit(value)) => Some(*value),
        None => None,
    };

    Ok(FieldData {
        name: ident,
        field_type: ty,
        default,
        optional,
        into,
        into_iter,
    })
}

fn collect_field_options(
    field: &Field,
) -> syn::Result<(DefaultValue, bool, bool, Option<IntoIterKind>)> {
    let mut default_value = DefaultValue::None;
    let mut explicit_default = false;
    let mut optional = false;
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
            field_options_parser(
                meta,
                &mut default_value,
                &mut explicit_default,
                &mut optional,
                &mut into,
                &mut into_iter,
            )
        })?;

        if !has_options {
            return Err(Error::new_spanned(
                attribute,
                "Expected an argument in `#[new(...)]` attribute.",
            ));
        }
    }

    detect_automatic_defaults(&mut default_value, field);
    check_invalid_field_options(
        field,
        &default_value,
        explicit_default,
        optional,
        into,
        &into_iter,
    )?;
    Ok((default_value, optional, into, into_iter))
}

fn field_options_parser(
    meta: ParseNestedMeta<'_>,
    default_value: &mut DefaultValue,
    explicit_default: &mut bool,
    optional: &mut bool,
    into: &mut bool,
    into_iter: &mut Option<IntoIterKind>,
) -> syn::Result<()> {
    if meta.path.is_ident("option") {
        return Err(meta.error("Unknown argument `option`. Did you mean `optional`?"));
    }

    if meta.path.is_ident("default") {
        *explicit_default = true;
        return parse_default(meta, default_value);
    }

    if meta.path.is_ident("optional") {
        return parse_optional(meta, optional);
    }

    if meta.path.is_ident("into") {
        return parse_into(meta, into);
    }

    if meta.path.is_ident("into_iter") {
        return parse_into_iter(meta, into_iter);
    }

    Err(meta.error("Unknown argument in `#[new(...)]` for field. Expected one of: `default`, `optional`, `into`, `into_iter`."))
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
    let value = meta.value()?;
    let default_expression: Expr = value.parse()?;
    *default_value = DefaultValue::CustomExpression(default_expression.into_token_stream());
    Ok(())
}

fn parse_optional(meta: ParseNestedMeta<'_>, optional: &mut bool) -> syn::Result<()> {
    if *optional {
        return Err(meta.error("`optional` specified more than once in `#[new(...)]`."));
    }

    // #[new(optional)]
    *optional = true;
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
    explicit_default: bool,
    optional: bool,
    into: bool,
    into_iter: &Option<IntoIterKind>,
) -> syn::Result<()> {
    if matches!(default_value, DefaultValue::Trait) && optional {
        return Err(Error::new_spanned(
            field,
            "Parameterless `default` is not needed when `optional` is present. Remove `default`.",
        ));
    }

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

    if explicit_default && is_auto_default_type(&field.ty) {
        return Err(Error::new_spanned(
            field,
            "`default` can be skipped for fields that have automatic defaults (`()` or `PhantomData`).",
        ));
    }

    if optional && is_auto_default_type(&field.ty) {
        return Err(Error::new_spanned(
            field,
            "`optional` cannot be used on fields that have automatic defaults (`()` or `PhantomData`).",
        ));
    }

    if optional && into {
        return Err(Error::new_spanned(
            field,
            "`optional` cannot be combined with `into` in `#[new(...)]`.",
        ));
    }

    if optional && into_iter.is_some() {
        return Err(Error::new_spanned(
            field,
            "`optional` cannot be combined with `into_iter` in `#[new(...)]`.",
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

fn extract_into_iter_type(ty: &Type) -> syn::Result<Type> {
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
