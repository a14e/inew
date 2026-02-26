use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Data, DataEnum, Generics};

use crate::{
    constructor::{generator, linter, options, plan},
    ItemKind,
};

pub(crate) fn process_input(
    ident: Ident,
    data: Data,
    generics: Generics,
    attributes: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let Data::Enum(DataEnum { variants, .. }) = data else {
        unreachable!("Input should already be validated");
    };

    let options = options::collect(&attributes, ItemKind::Enum)?;
    let mut constructors = Vec::new();

    for variant in variants {
        let plan = plan::build(&variant.fields, options.constant)?;

        let prefix = &options.constructor_name;
        let snake_case = variant.ident.to_string().to_snake_case();
        let constructor_name = if options.no_prefix {
            format_ident!("{}", snake_case)
        } else {
            format_ident!("{}_{}", prefix, snake_case)
        };
        let variant_ident = &variant.ident;

        let constructor = generator::generate_constructor(
            &plan,
            &options.visibility,
            &options.constant_keyword,
            &constructor_name,
            &quote!(Self::#variant_ident),
        );

        constructors.push(constructor);
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let lint_attributes = linter::collect_attributes(&attributes);

    Ok(quote! {
        #(#lint_attributes)*
        #[automatically_derived]
        impl #impl_generics #ident #type_generics #where_clause {
            #(#constructors)*
        }
    })
}
