extern crate proc_macro;

mod constructor;
mod new_enum;
mod new_struct;

use proc_macro2::{Ident, TokenStream};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Error, Generics};

// Inspired by a part of SeaORM: https://github.com/SeaQL/sea-orm/blob/master/sea-orm-macros/src/derives/active_model.rs
#[proc_macro_derive(New, attributes(new))]
pub fn derive_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    process_input(ident, data, generics, attrs)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn process_input(
    ident: Ident,
    data: Data,
    generics: Generics,
    attributes: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let new_type = find_type(&ident, &data)?;

    match new_type {
        NewType::Struct => new_struct::process_input(ident, data, generics, attributes),
        NewType::Enum => new_enum::process_input(ident, data, generics, attributes),
    }
}

enum NewType {
    Struct,
    Enum,
}

fn find_type(ident: &Ident, data: &Data) -> syn::Result<NewType> {
    match data {
        Data::Struct(_) => Ok(NewType::Struct),
        Data::Enum(_) => Ok(NewType::Enum),
        _ => Err(syn::Error::new_spanned(
            ident,
            "'New' can only be derived for structs and enums",
        )),
    }
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;
