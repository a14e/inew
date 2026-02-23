use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DataStruct, Generics};

use crate::constructor;

pub(crate) fn process_input(
    ident: Ident,
    data: Data,
    generics: Generics,
    attributes: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let Data::Struct(DataStruct { fields, .. }) = data else {
        unreachable!("Input should already be validated");
    };

    let properties = constructor::collect_main_properties(&attributes)?;
    let plan = constructor::build_constructor_plan(&fields, properties.constant)?;

    let constructor = constructor::generate_constructor(
        &plan,
        &properties.visibility,
        &properties.constant_keyword,
        &properties.constructor_name,
        &quote!(Self),
    );

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    Ok(quote!(
        #[automatically_derived]
        impl #impl_generics #ident #type_generics #where_clause {
            #constructor
        }
    ))
}
