use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Fields, Generics};

use crate::constructor::{field_options, generator, lint_extractor, main_options, ItemKind};

pub(crate) fn process_input(
    ident: Ident,
    fields: Fields,
    generics: Generics,
    attributes: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let options = main_options::collect(&attributes, ItemKind::Struct)?;
    let plan = field_options::collect(&fields, options.constant)?;

    let constructor_name = options.constructor_prefix;
    let constructor = generator::create_constructor(
        plan,
        &options.visibility,
        options.constant,
        quote!(#constructor_name),
        quote!(Self),
    );

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let lint_attributes = lint_extractor::collect(&attributes);

    Ok(quote!(
        #(#lint_attributes)*
        #[automatically_derived]
        impl #impl_generics #ident #type_generics #where_clause {
            #constructor
        }
    ))
}
