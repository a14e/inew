use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::constructor::plan::{ConstructorPlan, DefaultValue, FieldData, VariantShape};

pub(crate) fn generate_constructor(
    plan: &ConstructorPlan,
    visibility: &TokenStream,
    constant: &TokenStream,
    constructor_name: &Ident,
    self_ident: &TokenStream,
) -> TokenStream {
    let defaults = build_default_expressions(&plan.field_datas);
    let is_named = plan.shape == VariantShape::Struct;
    let (parameters, pass_values) =
        build_constructor_parameters(&plan.field_datas, defaults, is_named);

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

fn build_default_expressions(field_datas: &[FieldData]) -> Vec<Option<TokenStream>> {
    field_datas
        .iter()
        .map(|field_data| build_default_expression(&field_data.default))
        .collect()
}

fn build_default_expression(default: &DefaultValue) -> Option<TokenStream> {
    use DefaultValue::{CustomExpression, PhantomData, Trait, Unit};

    match default {
        DefaultValue::None => None,
        Unit => Some(quote!(())),
        PhantomData => Some(quote!(::core::marker::PhantomData)),
        Trait => Some(quote!(::core::default::Default::default())),
        CustomExpression(expression) => Some(quote!(#expression)),
    }
}

fn build_constructor_parameters(
    fields: &[FieldData],
    defaults: Vec<Option<TokenStream>>,
    is_named: bool,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut parameters = Vec::new();
    let mut values = Vec::new();

    for (field, default) in fields.iter().zip(defaults) {
        let (parameter, pass_value) = build_constructor_parameter(field, default, is_named);

        if let Some(stream) = parameter {
            parameters.push(stream);
        }

        values.push(pass_value);
    }

    (parameters, values)
}

fn build_constructor_parameter(
    field: &FieldData,
    default: Option<TokenStream>,
    is_named: bool,
) -> (Option<TokenStream>, TokenStream) {
    let FieldData {
        name,
        field_type,
        optional,
        into,
        into_iter,
        ..
    } = field;

    match (default, optional) {
        (Some(expression), true) => {
            let parameter = quote!(#name: ::core::option::Option<#field_type>);

            let pass_value = if is_named {
                quote! {
                    #name: match #name {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => #expression,
                    }
                }
            } else {
                quote! {
                    match #name {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => #expression,
                    }
                }
            };

            (Some(parameter), pass_value)
        }
        (Some(expression), false) => {
            let pass_value = if is_named {
                quote!(#name: #expression)
            } else {
                quote!(#expression)
            };

            (None, pass_value)
        }
        (None, true) => {
            let parameter = quote!(#name: ::core::option::Option<#field_type>);

            let pass_value = if is_named {
                quote! {
                    #name: match #name {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => ::core::default::Default::default(),
                    }
                }
            } else {
                quote! {
                    match #name {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => ::core::default::Default::default(),
                    }
                }
            };

            (Some(parameter), pass_value)
        }
        (None, _) => {
            if *into {
                let parameter = quote!(#name: impl ::core::convert::Into<#field_type>);

                let pass_value = if is_named {
                    quote!(#name: #name.into())
                } else {
                    quote!(#name.into())
                };

                (Some(parameter), pass_value)
            } else if let Some(iter_type) = into_iter {
                let parameter = quote!(#name: impl ::core::iter::IntoIterator<Item = #iter_type>);

                let pass_value = if is_named {
                    quote!(#name: #name.into_iter().collect())
                } else {
                    quote!(#name.into_iter().collect())
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
