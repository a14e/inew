use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, punctuated::Punctuated, token::Comma, Attribute, Error, Expr, Field,
    Fields, GenericArgument, LitBool, LitStr, PathArguments, Token, Type, TypePath, TypeTuple,
};

pub(crate) struct MainProperties {
    pub visibility: TokenStream,
    pub constructor_name: Ident,
    pub constant: bool,
    pub constant_keyword: TokenStream,
}

pub(crate) struct ConstructorPlan {
    pub parameters: Vec<TokenStream>,
    pub pass_values: Vec<TokenStream>,
    pub shape: VariantShape,
}

#[derive(Debug)]
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
    Explicit(Type),
}

enum DefaultValue {
    None,
    Unit,
    PhantomData,
    Trait,
    CustomFunction(TokenStream),
}

pub(crate) fn collect_main_properties(attributes: &[Attribute]) -> syn::Result<MainProperties> {
    let mut public = None;
    let mut rename = None;
    let mut constant = None;

    let mut seen_new_attribute = false;

    for attribute in attributes {
        if !attribute.path().is_ident("new") {
            continue;
        }

        if seen_new_attribute {
            return Err(Error::new_spanned(
                attribute,
                "Multiple #[new(...)] attributes found. Ensure only one is present.",
            ));
        }

        seen_new_attribute = true;
        let mut has_arguments = false;

        attribute.parse_nested_meta(|meta| {
            has_arguments = true;
            main_properties_parser(meta, &mut public, &mut rename, &mut constant)
        })?;

        if !has_arguments {
            return Err(Error::new_spanned(
                attribute,
                "Expected at least one argument inside #[new(...)] attribute.",
            ));
        }
    }

    let visibility = if public.unwrap_or(true) {
        quote!(pub)
    } else {
        quote!()
    };
    let constructor_name = rename.unwrap_or_else(|| Ident::new("new", Span::call_site()));
    let constant_result = constant.unwrap_or(false);
    let constant_keyword = if constant_result {
        quote!(const)
    } else {
        quote!()
    };

    Ok(MainProperties {
        visibility,
        constructor_name,
        constant: constant_result,
        constant_keyword,
    })
}

fn main_properties_parser(
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

pub(crate) fn build_constructor_plan(
    fields: &Fields,
    is_const: bool,
) -> syn::Result<ConstructorPlan> {
    let punctuated = extract_punctuated_fields(&fields);
    let field_data = collect_field_datas(&punctuated)?;

    if is_const {
        for field in &field_data {
            if matches!(field.default, DefaultValue::Trait) {
                return Err(Error::new_spanned(
                    &field.name,
                    "'default' is not allowed in const constructors",
                ));
            }

            if field.into {
                return Err(Error::new_spanned(
                    &field.name,
                    "'into' is not allowed in const constructors",
                ));
            }

            if field.into_iter.is_some() {
                return Err(Error::new_spanned(
                    &field.name,
                    "'into_iter' is not allowed in const constructors",
                ));
            }
        }
    }

    let shape = extract_shape(&fields);
    let is_named = matches!(shape, VariantShape::Struct);
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
    let (default, into, into_iter) = read_field_settings(field)?;

    Ok(FieldData {
        name: ident,
        field_type: ty,
        default,
        into,
        into_iter,
    })
}

fn read_field_settings(field: &Field) -> syn::Result<(DefaultValue, bool, Option<IntoIterKind>)> {
    let mut default_value = DefaultValue::None;
    let mut into = false;
    let mut into_iter: Option<IntoIterKind> = None;

    let mut seen_new_attribute = false;

    for attribute in &field.attrs {
        if !attribute.path().is_ident("new") {
            continue;
        }

        if seen_new_attribute {
            return Err(Error::new_spanned(
                attribute,
                "Multiple #[new(...)] attributes found. Ensure the field has only one #[new(...)] attribute.",
            ));
        }

        seen_new_attribute = true;

        let mut has_arguments = false;

        attribute.parse_nested_meta(|meta| {
            has_arguments = true;
            field_settings_parser(meta, &mut default_value, &mut into, &mut into_iter)
        })?;

        if !has_arguments {
            return Err(Error::new_spanned(
                attribute,
                "Expected an argument in #[new(...)] attribute.",
            ));
        }
    }

    detect_automatic_defaults(&mut default_value, field);
    check_invalid_combinations(field, &default_value, into, &into_iter)?;
    Ok((default_value, into, into_iter))
}

fn field_settings_parser(
    meta: ParseNestedMeta<'_>,
    default_value: &mut DefaultValue,
    into: &mut bool,
    into_iter: &mut Option<IntoIterKind>,
) -> syn::Result<()> {
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

    if meta.path.is_ident("into") {
        if *into {
            return Err(meta.error("Duplicate 'into' key found in #[new(...)] attribute."));
        }

        *into = true;
        return Ok(());
    }

    if meta.path.is_ident("into_iter") {
        if into_iter.is_some() {
            return Err(meta.error("Duplicate 'into_iter' key found in #[new(...)] attribute."));
        }

        if !meta.input.peek(Token![=]) {
            *into_iter = Some(IntoIterKind::Inferred);
            return Ok(());
        }

        meta.input.parse::<Token![=]>()?;
        let ty: Type = meta.input.parse()?;

        *into_iter = Some(IntoIterKind::Explicit(ty));
        return Ok(());
    }

    Err(meta.error("Unknown argument found in #[new(...)] attribute."))
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

    let Some(last) = path.segments.last() else {
        return false;
    };

    return last.ident == "PhantomData";
}

fn check_invalid_combinations(
    field: &Field,
    default_value: &DefaultValue,
    into: bool,
    into_iter: &Option<IntoIterKind>,
) -> syn::Result<()> {
    if !matches!(default_value, DefaultValue::None) && into {
        return Err(Error::new_spanned(
            field,
            "'default' and 'into' cannot be combined in the same #[new(...)] attribute.",
        ));
    }

    if !matches!(default_value, DefaultValue::None) && into_iter.is_some() {
        return Err(Error::new_spanned(
            field,
            "'default' and 'into_iter' cannot be combined in the same #[new(...)] attribute.",
        ));
    }

    if into && into_iter.is_some() {
        return Err(Error::new_spanned(
            field,
            "'into' and 'into_iter' cannot be combined in the same #[new(...)] attribute.",
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
                    IntoIterKind::Explicit(ty) => ty,
                };

                let parameter = quote!(
                    #name: impl ::core::iter::IntoIterator<Item = #inner_type>
                );

                let pass_value = if is_named {
                    quote!(#name: #name.into_iter().collect())
                } else {
                    quote!(#name.into_iter().collect())
                };

                return Ok((Some(parameter), pass_value));
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
            "Cannot infer iterator item type. Use #[new(into_iter = T)] to specify the item type explicitly.",
        ));
    };

    let Some(segment) = type_path.path.segments.last() else {
        return Err(Error::new_spanned(
            ty,
            "Cannot infer iterator item type. Use #[new(into_iter = T)] to specify the item type explicitly.",
        ));
    };

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return Err(Error::new_spanned(
            ty,
            "Iterator item type could not be inferred. Use #[new(into_iter = T)] to specify the item type explicitly.",
        ));
    };

    let Some(first) = args.args.first() else {
        return Err(Error::new_spanned(
            ty,
            "Iterator item type could not be inferred. Use #[new(into_iter = T)] to specify the item type explicitly.",
        ));
    };

    let GenericArgument::Type(inner) = first else {
        return Err(Error::new_spanned(
            first,
            "Unsupported generic argument. Use #[new(into_iter = T)] to specify the iterator item type explicitly.",
        ));
    };

    Ok(inner.clone())
}

pub(crate) fn generate_constructor(
    plan: &ConstructorPlan,
    visibility: &TokenStream,
    constant: &TokenStream,
    constructor_name: &Ident,
    self_ident: &TokenStream,
) -> TokenStream {
    let parameters = &plan.parameters;
    let pass_values = &plan.pass_values;

    match &plan.shape {
        VariantShape::Unit => {
            quote! {
                #[must_use]
                #visibility #constant fn #constructor_name() -> Self {
                    #self_ident
                }
            }
        }
        VariantShape::Tuple => {
            quote! {
                #[must_use]
                #visibility #constant fn #constructor_name(#(#parameters),*) -> Self {
                    #self_ident(#(#pass_values),*)
                }
            }
        }
        VariantShape::Struct => {
            quote! {
                #[must_use]
                #visibility #constant fn #constructor_name(#(#parameters),*) -> Self {
                    #self_ident {
                        #(#pass_values),*
                    }
                }
            }
        }
    }
}

