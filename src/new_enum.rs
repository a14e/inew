use proc_macro2::{Ident, TokenStream};
use syn::{Attribute, Data, Error, Generics};

pub(crate) fn process_input(
    ident: Ident,
    _data: Data,
    _generics: Generics,
    _attributes: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    Err(Error::new_spanned(ident, "Unimplemented for now"))
}
