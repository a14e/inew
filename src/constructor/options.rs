use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, parenthesized, token::Paren, Attribute, Error, LitBool, LitStr, Token,
};

use crate::ItemKind;

pub(crate) struct MainOptions {
    pub visibility: TokenStream,
    pub constructor_name: Ident,
    pub no_prefix: bool,
    pub constant: bool,
    pub constant_keyword: TokenStream,
}

enum Visibility {
    Private,
    Public,
    Crate,
    Super,
    InSelf,
    In(TokenStream),
}

pub(crate) fn collect(attributes: &[Attribute], item_kind: ItemKind) -> syn::Result<MainOptions> {
    let mut visibility = None;
    let mut rename = None;
    let mut no_prefix = None;
    let mut constant = None;

    let mut seen_new_attribute = false;

    for attribute in attributes {
        if !attribute.path().is_ident("new") {
            continue;
        }

        if seen_new_attribute {
            return Err(Error::new_spanned(
                attribute,
                "Multiple `#[new(...)]` attributes are not allowed.",
            ));
        }

        seen_new_attribute = true;
        let mut has_options = false;

        attribute.parse_nested_meta(|meta| {
            has_options = true;
            main_options_parser(
                meta,
                &mut visibility,
                &mut no_prefix,
                &mut rename,
                &mut constant,
                item_kind,
            )
        })?;

        if !has_options {
            return Err(Error::new_spanned(
                attribute,
                "`#[new]` requires at least one argument (e.g. `#[new(pub)]` or `#[new(rename = \"foo\")]`).",
            ));
        }

        check_invalid_main_options(attribute, &rename, &no_prefix)?;
    }

    let visibility = visibility.unwrap_or_else(|| {
        #[cfg(feature = "public-default")]
        {
            Visibility::Public
        }
        #[cfg(not(feature = "public-default"))]
        {
            Visibility::Private
        }
    });
    let visibility_token = create_visibility_token(&visibility);

    let constructor_name = rename.unwrap_or_else(|| Ident::new("new", Span::call_site()));
    let no_prefix = no_prefix.unwrap_or(false);

    let constant_result = constant.unwrap_or(false);
    let constant_keyword = if constant_result {
        quote!(const)
    } else {
        quote!()
    };

    Ok(MainOptions {
        visibility: visibility_token,
        constructor_name,
        no_prefix,
        constant: constant_result,
        constant_keyword,
    })
}

fn main_options_parser(
    meta: ParseNestedMeta<'_>,
    visibility: &mut Option<Visibility>,
    no_prefix: &mut Option<bool>,
    rename: &mut Option<Ident>,
    constant: &mut Option<bool>,
    item_kind: ItemKind,
) -> syn::Result<()> {
    if meta.path.is_ident("public") {
        return Err(meta.error("Unknown argument `public`. Did you mean `pub`?"));
    }

    if meta.path.is_ident("constant") {
        return Err(meta.error("Unknown argument `constant`. Did you mean `const`?"));
    }

    if meta.path.is_ident("pub") {
        return parse_pub(meta, visibility);
    }

    if meta.path.is_ident("rename") {
        return parse_rename(meta, rename);
    }

    if meta.path.is_ident("no_prefix") {
        return parse_no_prefix(meta, no_prefix, item_kind);
    }

    if meta.path.is_ident("const") {
        return parse_const(meta, constant);
    }

    Err(unknown_argument_error(&meta, item_kind))
}

fn parse_pub(meta: ParseNestedMeta<'_>, visibility: &mut Option<Visibility>) -> syn::Result<()> {
    if visibility.is_some() {
        return Err(meta.error("`pub` specified more than once in `#[new(...)]`."));
    }

    // #[new(pub(...))]
    if meta.input.peek(Paren) {
        let content;
        parenthesized!(content in meta.input);

        if content.is_empty() {
            return Err(
                meta.error("Empty `pub()` is not valid. Use `pub` or `pub(<restriction>)`.")
            );
        }

        if content.peek(Token![crate]) {
            content.parse::<Token![crate]>()?;

            if !content.is_empty() {
                return Err(meta.error("Unexpected tokens after `crate` in `pub(...)`."));
            }

            *visibility = Some(Visibility::Crate);
            return Ok(());
        }

        if content.peek(Token![super]) {
            content.parse::<Token![super]>()?;

            if !content.is_empty() {
                return Err(meta.error("Unexpected tokens after `super` in `pub(...)`."));
            }

            *visibility = Some(Visibility::Super);
            return Ok(());
        }

        if content.peek(Token![self]) {
            content.parse::<Token![self]>()?;

            if !content.is_empty() {
                return Err(meta.error("Unexpected tokens after `self` in `pub(...)`."));
            }

            *visibility = Some(Visibility::InSelf);
            return Ok(());
        }

        if content.peek(Token![in]) {
            content.parse::<Token![in]>()?;
            let path: syn::Path = content
                .parse()
                .map_err(|_| meta.error("Expected path after `in` in `pub(in <path>)`."))?;

            if !content.is_empty() {
                return Err(meta.error("Unexpected tokens after `in <path>` in `pub(...)`."));
            }

            *visibility = Some(Visibility::In(path.into_token_stream()));
            return Ok(());
        }

        return Err(
                meta.error("Invalid visibility inside `pub(...)`. Expected `crate`, `super`, `self`, or `in <path>`."),
            );
    }

    // #[new(pub = ...)]
    if meta.input.peek(Token![=]) {
        meta.input.parse::<Token![=]>()?;

        if !meta.input.peek(LitBool) {
            return Err(meta.error("Expected boolean literal after `pub =`."));
        }

        let lit_bool = meta.input.parse::<LitBool>()?;
        *visibility = Some(if lit_bool.value {
            Visibility::Public
        } else {
            Visibility::Private
        });
        return Ok(());
    }

    // #[new(pub)]
    *visibility = Some(Visibility::Public);
    Ok(())
}

fn parse_rename(meta: ParseNestedMeta<'_>, rename: &mut Option<Ident>) -> syn::Result<()> {
    if rename.is_some() {
        return Err(meta.error("`rename` specified more than once in `#[new(...)]`."));
    }

    if !meta.input.peek(Token![=]) {
        return Err(meta.error("Expected `rename = \"name\"`."));
    }

    // #[new(rename = ...)]
    meta.input.parse::<Token![=]>()?;

    if !meta.input.peek(LitStr) {
        return Err(meta.error("Expected string literal after `rename =`."));
    }

    let lit_str = meta.input.parse::<LitStr>()?;
    let name = lit_str.value();

    if syn::parse_str::<Ident>(&name).is_err() {
        return Err(Error::new_spanned(
            lit_str,
            "`rename` must be a valid Rust identifier.",
        ));
    }

    *rename = Some(Ident::new(&name, lit_str.span()));
    Ok(())
}

fn parse_no_prefix(
    meta: ParseNestedMeta<'_>,
    no_prefix: &mut Option<bool>,
    item_kind: ItemKind,
) -> syn::Result<()> {
    if item_kind == ItemKind::Struct {
        return Err(meta.error("`no_prefix` is only supported on enums."));
    }

    if no_prefix.is_some() {
        return Err(meta.error("`no_prefix` specified more than once."));
    }

    // #[new(no_prefix)]
    if !meta.input.peek(Token![=]) {
        *no_prefix = Some(true);
        return Ok(());
    }

    // #[new(no_prefix = ...)]
    meta.input.parse::<Token![=]>()?;

    if !meta.input.peek(LitBool) {
        return Err(meta.error("Expected boolean literal after `no_prefix =`."));
    }

    let lit_bool = meta.input.parse::<LitBool>()?;

    *no_prefix = Some(lit_bool.value);
    Ok(())
}

fn parse_const(meta: ParseNestedMeta<'_>, constant: &mut Option<bool>) -> syn::Result<()> {
    if constant.is_some() {
        return Err(meta.error("`const` specified more than once in `#[new(...)]`."));
    }

    // #[new(const)]
    if !meta.input.peek(Token![=]) {
        *constant = Some(true);
        return Ok(());
    }

    // #[new(const = ...)]
    meta.input.parse::<Token![=]>()?;

    if !meta.input.peek(LitBool) {
        return Err(meta.error("Expected boolean literal after `const =`."));
    }

    let lit_bool = meta.input.parse::<LitBool>()?;

    *constant = Some(lit_bool.value);
    Ok(())
}

fn unknown_argument_error(meta: &ParseNestedMeta<'_>, item_kind: ItemKind) -> syn::Error {
    match item_kind {
        ItemKind::Enum => {
            meta.error("Unknown argument. Expected one of: `pub`, `rename`, `no_prefix`, `const`.")
        }
        ItemKind::Struct => {
            meta.error("Unknown argument. Expected one of: `pub`, `rename`, `const`.")
        }
    }
}

fn check_invalid_main_options(
    attribute: &Attribute,
    rename: &Option<Ident>,
    no_prefix: &Option<bool>,
) -> syn::Result<()> {
    if rename.is_some() && no_prefix.is_some() {
        return Err(Error::new_spanned(
            attribute,
            "`rename` cannot be combined with `no_prefix` in `#[new(...)]`.",
        ));
    }

    Ok(())
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
