use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::constructor::{
    field_options::{ConstructorPlan, DefaultValue, FieldData, FieldOptions, VariantShape},
    main_options::Visibility,
};

pub(crate) fn create_constructor(
    plan: ConstructorPlan,
    visibility: &Visibility,
    constant: bool,
    constructor_name: TokenStream,
    self_ident: TokenStream,
) -> TokenStream {
    let defaults = build_default_expressions(&plan.fields);
    let is_named = plan.shape == VariantShape::Struct;
    let (parameters, pass_values) = build_constructor_parameters(&plan.fields, defaults, is_named);

    let visibility = create_visibility_token(visibility);
    let constant = create_constant_token(constant);

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

fn build_default_expressions(fields: &[FieldData]) -> Vec<Option<TokenStream>> {
    fields
        .iter()
        .map(|field_data| build_default_expression(field_data.options.default.as_ref()))
        .collect()
}

fn build_default_expression(default: Option<&DefaultValue>) -> Option<TokenStream> {
    use DefaultValue::*;

    let token = match default? {
        Unit => quote!(()),
        PhantomData => quote!(::core::marker::PhantomData),
        Trait => quote!(::core::default::Default::default()),
        Custom(expression) => quote!(#expression),
    };

    Some(token)
}

fn build_constructor_parameters(
    fields: &[FieldData],
    defaults: Vec<Option<TokenStream>>,
    is_named: bool,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut parameters = Vec::new();
    let mut values = Vec::new();

    for (index, (field, default)) in fields.iter().zip(defaults).enumerate() {
        let (parameter, pass_value) = build_constructor_parameter(field, index, default, is_named);

        if let Some(stream) = parameter {
            parameters.push(stream);
        }

        values.push(pass_value);
    }

    (parameters, values)
}

fn build_constructor_parameter(
    field: &FieldData,
    index: usize,
    default: Option<TokenStream>,
    is_named: bool,
) -> (Option<TokenStream>, TokenStream) {
    let FieldData {
        name,
        field_type,
        options:
            FieldOptions {
                optional,
                into,
                into_iter,
                ..
            },
    } = field;

    let field_name = name.clone().unwrap_or_else(|| format_ident!("_{}", index));

    match (default, optional) {
        (Some(expression), true) => {
            let parameter = quote!(#field_name: ::core::option::Option<#field_type>);

            let pass_value = if is_named {
                quote! {
                    #field_name: match #field_name {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => #expression,
                    }
                }
            } else {
                quote! {
                    match #field_name {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => #expression,
                    }
                }
            };

            (Some(parameter), pass_value)
        }
        (Some(expression), false) => {
            let pass_value = if is_named {
                quote!(#field_name: #expression)
            } else {
                quote!(#expression)
            };

            (None, pass_value)
        }
        (None, true) => {
            let parameter = quote!(#field_name: ::core::option::Option<#field_type>);

            let pass_value = if is_named {
                quote! {
                    #field_name: match #field_name {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => ::core::default::Default::default(),
                    }
                }
            } else {
                quote! {
                    match #field_name {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => ::core::default::Default::default(),
                    }
                }
            };

            (Some(parameter), pass_value)
        }
        (None, _) => {
            if *into {
                let parameter = quote!(#field_name: impl ::core::convert::Into<#field_type>);

                let pass_value = if is_named {
                    quote!(#field_name: #field_name.into())
                } else {
                    quote!(#field_name.into())
                };

                (Some(parameter), pass_value)
            } else if let Some(iter_type) = into_iter {
                let parameter =
                    quote!(#field_name: impl ::core::iter::IntoIterator<Item = #iter_type>);

                let pass_value = if is_named {
                    quote!(#field_name: #field_name.into_iter().collect())
                } else {
                    quote!(#field_name.into_iter().collect())
                };

                (Some(parameter), pass_value)
            } else {
                let parameter = quote!(#field_name: #field_type);

                let pass_value = if is_named {
                    quote!(#field_name: #field_name)
                } else {
                    quote!(#field_name)
                };

                (Some(parameter), pass_value)
            }
        }
    }
}

fn create_visibility_token(visibility: &Visibility) -> TokenStream {
    use Visibility::*;

    match visibility {
        Private => quote!(),
        Public => quote!(pub),
        Crate => quote!(pub(crate)),
        Super => quote!(pub(super)),
        InSelf => quote!(pub(self)),
        In(path) => quote!(pub(in #path)),
    }
}

fn create_constant_token(constant: bool) -> TokenStream {
    if constant {
        quote!(const)
    } else {
        quote!()
    }
}
