use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DataStruct, Generics};

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
    let Data::Struct(DataStruct { fields, .. }) = data else {
        unreachable!("Input should already be validated");
    };

    let options = options::collect(&attributes, ItemKind::Struct)?;
    let plan = plan::build(&fields, options.constant)?;

    let constructor = generator::generate_constructor(
        &plan,
        &options.visibility,
        &options.constant_keyword,
        &options.constructor_name,
        &quote!(Self),
    );

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let lint_attributes = linter::collect_attributes(&attributes);

    Ok(quote!(
        #(#lint_attributes)*
        #[automatically_derived]
        impl #impl_generics #ident #type_generics #where_clause {
            #constructor
        }
    ))
}
