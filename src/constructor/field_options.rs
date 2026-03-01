use proc_macro2::{Ident, Span};
use syn::{
    meta::ParseNestedMeta, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, Expr,
    Field, Fields, GenericArgument, LitBool, PathArguments, Token, Type, TypePath, TypeTuple,
};

use crate::constructor::BoolArgument;

pub(crate) struct ConstructorPlan {
    pub shape: VariantShape,
    pub fields: Vec<FieldData>,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum VariantShape {
    Unit,
    Tuple,
    Struct,
}

pub(crate) struct FieldData {
    pub name: Option<Ident>,
    pub field_type: Type,
    pub options: FieldOptions,
}

pub(crate) struct FieldOptions {
    pub default: Option<DefaultValue>,
    pub optional: bool,
    pub into: bool,
    pub into_iter: Option<Type>,
}

impl From<FieldArguments> for FieldOptions {
    fn from(value: FieldArguments) -> Self {
        Self {
            default: value.default.map(|argument| argument.value),
            optional: value.optional.as_ref().is_some_and(BoolArgument::value),
            into: value.into.as_ref().is_some_and(BoolArgument::value),
            into_iter: value.into_iter,
        }
    }
}

pub(crate) enum DefaultValue {
    Unit,
    PhantomData,
    /// #[new(default)]
    Trait,
    /// #[new(default = ...)]
    Custom(Expr),
}

#[derive(Default)]
struct FieldArguments {
    default: Option<DefaultArgument>,
    optional: Option<BoolArgument>,
    into: Option<BoolArgument>,
    into_iter: Option<Type>,
}

struct DefaultArgument {
    value: DefaultValue,
    span: Span,
}

impl DefaultArgument {
    fn new(value: DefaultValue, span: Span) -> Self {
        Self { value, span }
    }
}

pub(crate) fn collect(fields: &Fields, constant: bool) -> syn::Result<ConstructorPlan> {
    let punctuated = extract_punctuated_fields(fields);

    Ok(ConstructorPlan {
        fields: collect_field_datas(&punctuated, constant)?,
        shape: extract_shape(fields),
    })
}

fn extract_punctuated_fields(fields: &Fields) -> Punctuated<Field, Comma> {
    match fields {
        Fields::Named(found) => found.named.clone(),
        Fields::Unnamed(found) => found.unnamed.clone(),
        Fields::Unit => Default::default(),
    }
}

fn collect_field_datas(
    fields: &Punctuated<Field, Comma>,
    constant: bool,
) -> syn::Result<Vec<FieldData>> {
    fields
        .iter()
        .map(|field| collect_field_data(field, constant))
        .collect()
}

fn collect_field_data(field: &Field, constant: bool) -> syn::Result<FieldData> {
    Ok(FieldData {
        name: field.ident.clone(),
        field_type: field.ty.clone(),
        options: collect_field_options(field, constant)?,
    })
}

fn collect_field_options(field: &Field, constant: bool) -> syn::Result<FieldOptions> {
    let mut seen_new_attribute = false;
    let mut arguments = FieldArguments::default();

    for attribute in &field.attrs {
        if !attribute.path().is_ident("new") {
            continue;
        }

        if seen_new_attribute {
            return Err(Error::new_spanned(
                attribute,
                "Multiple field `#[new(...)]` attributes are not allowed.",
            ));
        }

        seen_new_attribute = true;

        let mut has_options = false;

        attribute.parse_nested_meta(|meta| {
            has_options = true;
            field_options_parser(meta, &mut arguments, &field.ty, constant)
        })?;

        if !has_options {
            return Err(Error::new_spanned(
                attribute,
                "Field `#[new]` requires at least one argument (e.g. `#[new(optional)]` or `#[new(default = 123)]`).",
            ));
        }
    }

    detect_automatic_defaults(&mut arguments.default, &field.ty);
    check_invalid_field_options(field, &arguments, constant)?;

    Ok(arguments.into())
}

fn field_options_parser(
    meta: ParseNestedMeta<'_>,
    arguments: &mut FieldArguments,
    field_type: &Type,
    constant: bool,
) -> syn::Result<()> {
    if meta.path.is_ident("option") {
        return Err(meta.error("Unknown argument `option`. Did you mean `optional`?"));
    }

    if meta.path.is_ident("default") {
        return parse_default(meta, &mut arguments.default, constant);
    }

    if meta.path.is_ident("optional") {
        return parse_optional(meta, &mut arguments.optional);
    }

    if meta.path.is_ident("into") {
        return parse_into(meta, &mut arguments.into, constant);
    }

    if meta.path.is_ident("into_iter") {
        return parse_into_iter(meta, &mut arguments.into_iter, field_type, constant);
    }

    Err(meta.error("Unknown argument in field `#[new(...)]`. Expected one of: `default`, `optional`, `into`, `into_iter`."))
}

fn parse_default(
    meta: ParseNestedMeta<'_>,
    default_value: &mut Option<DefaultArgument>,
    constant: bool,
) -> syn::Result<()> {
    if default_value.is_some() {
        return Err(meta.error("`default` specified more than once in field `#[new(...)]`."));
    }

    let span = meta.path.span();

    // #[new(default)]
    if !meta.input.peek(Token![=]) {
        if constant {
            return Err(
                meta.error("Parameterless `default` cannot be used in constant constructors.")
            );
        }

        *default_value = Some(DefaultArgument::new(DefaultValue::Trait, span));
        return Ok(());
    }

    // #[new(default = ...)]
    let value = meta.value()?;
    let default_expression = value.parse::<Expr>()?;
    *default_value = Some(DefaultArgument::new(
        DefaultValue::Custom(default_expression),
        span,
    ));
    Ok(())
}

fn parse_optional(
    meta: ParseNestedMeta<'_>,
    optional: &mut Option<BoolArgument>,
) -> syn::Result<()> {
    if optional.is_some() {
        return Err(meta.error("`optional` specified more than once in field `#[new(...)]`."));
    }

    let span = meta.path.span();

    // #[new(optional)]
    if !meta.input.peek(Token![=]) {
        *optional = Some(BoolArgument::new(true, span));
        return Ok(());
    }

    // #[new(optional = ...)]
    meta.input.parse::<Token![=]>()?;

    if !meta.input.peek(LitBool) {
        return Err(meta.error("Expected boolean literal after `optional =`."));
    }

    let lit_bool = meta.input.parse::<LitBool>()?;

    *optional = Some(BoolArgument::new(lit_bool.value, span));
    Ok(())
}

fn parse_into(
    meta: ParseNestedMeta<'_>,
    into: &mut Option<BoolArgument>,
    constant: bool,
) -> syn::Result<()> {
    if into.is_some() {
        return Err(meta.error("`into` specified more than once in field `#[new(...)]`."));
    }

    let span = meta.path.span();

    // #[new(into)]
    if !meta.input.peek(Token![=]) {
        if constant {
            return Err(meta.error("`into` cannot be used in constant constructors."));
        }

        *into = Some(BoolArgument::new(true, span));
        return Ok(());
    }

    // #[new(into = ...)]
    meta.input.parse::<Token![=]>()?;

    if !meta.input.peek(LitBool) {
        return Err(meta.error("Expected boolean literal after `into =`."));
    }

    let lit_bool = meta.input.parse::<LitBool>()?;

    if constant && lit_bool.value {
        return Err(meta.error("`into` cannot be used in constant constructors."));
    }

    *into = Some(BoolArgument::new(lit_bool.value, span));
    Ok(())
}

fn parse_into_iter(
    meta: ParseNestedMeta<'_>,
    into_iter: &mut Option<Type>,
    field_type: &Type,
    constant: bool,
) -> syn::Result<()> {
    if into_iter.is_some() {
        return Err(meta.error("`into_iter` specified more than once in field `#[new(...)]`."));
    }

    if constant {
        return Err(meta.error("`into_iter` cannot be used in constant constructors."));
    }

    // #[new(into_iter)]
    if !meta.input.peek(Token![=]) {
        let ty = extract_into_iter_type(field_type)?;
        *into_iter = Some(ty);
        return Ok(());
    }

    // #[new(into_iter = ...)]
    meta.input.parse::<Token![=]>()?;
    let ty = meta.input.parse::<Type>()?;

    *into_iter = Some(ty);
    Ok(())
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

fn detect_automatic_defaults(default: &mut Option<DefaultArgument>, field_type: &Type) {
    if default.is_some() {
        return;
    }

    let span = field_type.span();

    if is_phantom_data(field_type) {
        *default = Some(DefaultArgument::new(DefaultValue::PhantomData, span));
        return;
    }

    let Type::Tuple(TypeTuple { elems, .. }) = &field_type else {
        return;
    };

    if elems.is_empty() {
        *default = Some(DefaultArgument::new(DefaultValue::Unit, span));
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
    arguments: &FieldArguments,
    constant: bool,
) -> syn::Result<()> {
    let default_trait = matches!(
        arguments.default,
        Some(DefaultArgument {
            value: DefaultValue::Trait,
            ..
        })
    );
    let is_optional = arguments.optional.as_ref().is_some_and(BoolArgument::value);

    if default_trait && is_optional {
        if let Some(default) = &arguments.default {
            return Err(Error::new(
                default.span,
                "Parameterless `default` is not needed when `optional` is true. Remove `default`.",
            ));
        }
    }

    let explicit_default = matches!(
        arguments.default,
        Some(DefaultArgument {
            value: DefaultValue::Trait | DefaultValue::Custom(..),
            ..
        })
    );

    if constant && is_optional && !explicit_default {
        if let Some(optional) = &arguments.optional {
            return Err(Error::new(
                optional.span,
                "`optional` without an explicit `default = ...` cannot be used in constant constructors.",
            ));
        }
    }

    let is_into = arguments.into.as_ref().is_some_and(BoolArgument::value);

    if explicit_default && is_into {
        if let Some(default) = &arguments.default {
            return Err(Error::new(
                default.span,
                "`default` cannot be combined with `into` in field `#[new(...)]`.",
            ));
        }
    }

    let is_into_inter = arguments.into_iter.is_some();

    if explicit_default && is_into_inter {
        if let Some(default) = &arguments.default {
            return Err(Error::new(
                default.span,
                "`default` cannot be combined with `into_iter` in field `#[new(...)]`.",
            ));
        }
    }

    let is_auto_default = is_auto_default_type(&field.ty);

    if explicit_default && is_auto_default {
        return Err(Error::new_spanned(
            field,
            "`default` can be skipped for fields that have automatic defaults (`()` or `PhantomData`).",
        ));
    }

    if is_optional && is_auto_default {
        return Err(Error::new_spanned(
                field,
                "`optional` cannot be used on fields that have automatic defaults (`()` or `PhantomData`).",
            ));
    }

    if is_optional && is_into {
        if let Some(optional) = &arguments.optional {
            return Err(Error::new(
                optional.span,
                "`optional` cannot be combined with `into` in field `#[new(...)]`.",
            ));
        }
    }

    if is_optional && is_into_inter {
        if let Some(optional) = &arguments.optional {
            return Err(Error::new(
                optional.span,
                "`optional` cannot be combined with `into_iter` in field `#[new(...)]`.",
            ));
        }
    }

    if is_into && is_into_inter {
        if let Some(into) = &arguments.into {
            return Err(Error::new(
                into.span,
                "`into` cannot be combined with `into_iter` in field `#[new(...)]`.",
            ));
        }
    }

    Ok(())
}

fn is_auto_default_type(ty: &Type) -> bool {
    is_phantom_data(ty) || matches!(ty, Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty())
}

fn extract_shape(fields: &Fields) -> VariantShape {
    match fields {
        Fields::Unit => VariantShape::Unit,
        Fields::Unnamed(_) => VariantShape::Tuple,
        Fields::Named(_) => VariantShape::Struct,
    }
}
