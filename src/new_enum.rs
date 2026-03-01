use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, Attribute, Generics, Variant};

use crate::constructor::{field_options, generator, lint_extractor, main_options, ItemKind};

pub(crate) fn process_input(
    ident: Ident,
    variants: Punctuated<Variant, Comma>,
    generics: Generics,
    attributes: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let options = main_options::collect(&attributes, ItemKind::Enum)?;

    if variants.is_empty() {
        return Ok(quote!());
    }

    let mut constructors = Vec::new();

    for variant in variants {
        let plan = field_options::collect(&variant.fields, options.constant)?;

        let snake_case = variant.ident.to_string().to_snake_case();
        let constructor_name = if let Some(prefix) = &options.constructor_prefix {
            format_ident!("{}_{}", prefix, snake_case)
        } else {
            format_ident!("{}", snake_case)
        };
        let variant_ident = &variant.ident;

        let constructor = generator::create_constructor(
            plan,
            &options.visibility,
            options.constant,
            quote!(#constructor_name),
            quote!(Self::#variant_ident),
        );

        constructors.push(constructor);
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let lint_attributes = lint_extractor::collect(&attributes);

    Ok(quote! {
        #(#lint_attributes)*
        #[automatically_derived]
        impl #impl_generics #ident #type_generics #where_clause {
            #(#constructors)*
        }
    })
}
