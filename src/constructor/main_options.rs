use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{
    meta::ParseNestedMeta, parenthesized, spanned::Spanned, token::Paren, Attribute, Error,
    LitBool, LitStr, Token,
};

use crate::constructor::{BoolArgument, ItemKind};

pub(crate) struct MainOptions {
    pub visibility: Visibility,
    pub constructor_prefix: Option<Ident>,
    pub constant: bool,
}

impl From<MainArguments> for MainOptions {
    fn from(value: MainArguments) -> Self {
        let no_prefix = value.no_prefix.as_ref().is_some_and(BoolArgument::value);

        let constructor_prefix = if no_prefix {
            None
        } else {
            match value.rename {
                Some(rename) => Some(rename),
                None => Some(Ident::new("new", Span::call_site())),
            }
        };

        Self {
            visibility: value.visibility.into(),
            constructor_prefix,
            constant: value.constant.as_ref().is_some_and(BoolArgument::value),
        }
    }
}

pub(crate) enum Visibility {
    Private,
    Public,
    Crate,
    Super,
    InSelf,
    In(TokenStream),
}

impl From<Option<PubArgument>> for Visibility {
    fn from(value: Option<PubArgument>) -> Self {
        match value {
            Some(PubArgument::Public) => Visibility::Public,
            Some(PubArgument::Explicit(is_public)) => {
                if is_public {
                    Visibility::Public
                } else {
                    Visibility::Private
                }
            }
            Some(PubArgument::Crate) => Visibility::Crate,
            Some(PubArgument::Super) => Visibility::Super,
            Some(PubArgument::InSelf) => Visibility::InSelf,
            Some(PubArgument::In(path)) => Visibility::In(path),
            None => {
                #[cfg(feature = "public-default")]
                {
                    Visibility::Public;
                }
                #[cfg(not(feature = "public-default"))]
                {
                    Visibility::Private
                }
            }
        }
    }
}

#[derive(Default)]
struct MainArguments {
    visibility: Option<PubArgument>,
    rename: Option<Ident>,
    no_prefix: Option<BoolArgument>,
    constant: Option<BoolArgument>,
}

enum PubArgument {
    /// #[new(pub)]
    Public,
    /// #[new(pub = ...)]
    Explicit(bool),
    /// #[new(pub(crate))]
    Crate,
    /// #[new(pub(super))]
    Super,
    /// #[new(pub(self))]
    InSelf,
    /// #[new(pub(in ...))]
    In(TokenStream),
}

pub(crate) fn collect(attributes: &[Attribute], item_kind: ItemKind) -> syn::Result<MainOptions> {
    let mut seen_new_attribute = false;
    let mut arguments = MainArguments::default();

    for attribute in attributes {
        if !attribute.path().is_ident("new") {
            continue;
        }

        if seen_new_attribute {
            return Err(Error::new_spanned(
                attribute,
                "Multiple main `#[new(...)]` attributes are not allowed.",
            ));
        }

        seen_new_attribute = true;
        let mut has_options = false;

        attribute.parse_nested_meta(|meta| {
            has_options = true;
            main_options_parser(meta, &mut arguments, item_kind)
        })?;

        if !has_options {
            return Err(Error::new_spanned(
                attribute,
                "Main `#[new]` requires at least one argument (e.g. `#[new(pub)]` or `#[new(rename = \"name\")]`).",
            ));
        }

        check_invalid_main_arguments(&arguments)?;
    }

    Ok(arguments.into())
}

fn main_options_parser(
    meta: ParseNestedMeta<'_>,
    arguments: &mut MainArguments,
    item_kind: ItemKind,
) -> syn::Result<()> {
    if meta.path.is_ident("public") {
        return Err(meta.error("Unknown argument `public`. Did you mean `pub`?"));
    }

    if meta.path.is_ident("constant") {
        return Err(meta.error("Unknown argument `constant`. Did you mean `const`?"));
    }

    if meta.path.is_ident("pub") {
        return parse_pub(meta, &mut arguments.visibility);
    }

    if meta.path.is_ident("rename") {
        return parse_rename(meta, &mut arguments.rename, item_kind);
    }

    if meta.path.is_ident("no_prefix") {
        return parse_no_prefix(meta, &mut arguments.no_prefix, item_kind);
    }

    if meta.path.is_ident("const") {
        return parse_const(meta, &mut arguments.constant);
    }

    Err(unknown_argument_error(&meta, item_kind))
}

fn parse_pub(meta: ParseNestedMeta<'_>, visibility: &mut Option<PubArgument>) -> syn::Result<()> {
    if visibility.is_some() {
        return Err(meta.error("`pub` specified more than once in main `#[new(...)]`."));
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

            *visibility = Some(PubArgument::Crate);
            return Ok(());
        }

        if content.peek(Token![super]) {
            content.parse::<Token![super]>()?;

            if !content.is_empty() {
                return Err(meta.error("Unexpected tokens after `super` in `pub(...)`."));
            }

            *visibility = Some(PubArgument::Super);
            return Ok(());
        }

        if content.peek(Token![self]) {
            content.parse::<Token![self]>()?;

            if !content.is_empty() {
                return Err(meta.error("Unexpected tokens after `self` in `pub(...)`."));
            }

            *visibility = Some(PubArgument::InSelf);
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

            *visibility = Some(PubArgument::In(path.into_token_stream()));
            return Ok(());
        }

        return Err(
                meta.error("Unknown visibility inside `pub(...)`. Expected `crate`, `super`, `self`, or `in <path>`."),
            );
    }

    // #[new(pub = ...)]
    if meta.input.peek(Token![=]) {
        meta.input.parse::<Token![=]>()?;

        if !meta.input.peek(LitBool) {
            return Err(meta.error("Expected boolean literal after `pub =`."));
        }

        let lit_bool = meta.input.parse::<LitBool>()?;
        *visibility = Some(PubArgument::Explicit(lit_bool.value));
        return Ok(());
    }

    // #[new(pub)]
    *visibility = Some(PubArgument::Public);
    Ok(())
}

fn parse_rename(
    meta: ParseNestedMeta<'_>,
    rename: &mut Option<Ident>,
    item_kind: ItemKind,
) -> syn::Result<()> {
    if rename.is_some() {
        return Err(meta.error("`rename` specified more than once in main `#[new(...)]`."));
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

    if name.is_empty() {
        if item_kind == ItemKind::Enum {
            return Err(meta.error("Empty `rename` is not allowed. If you want enum constructors without prefixes, replace it with `no_prefix`."));
        } else {
            return Err(meta.error("Expected non-empty string literal after `rename =`."));
        }
    }

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
    no_prefix: &mut Option<BoolArgument>,
    item_kind: ItemKind,
) -> syn::Result<()> {
    if item_kind == ItemKind::Struct {
        return Err(meta.error("`no_prefix` is only supported on enums."));
    }

    if no_prefix.is_some() {
        return Err(meta.error("`no_prefix` specified more than once in main `#[new(...)]`."));
    }

    let span = meta.path.span();

    // #[new(no_prefix)]
    if !meta.input.peek(Token![=]) {
        *no_prefix = Some(BoolArgument::new(true, span));
        return Ok(());
    }

    // #[new(no_prefix = ...)]
    meta.input.parse::<Token![=]>()?;

    if !meta.input.peek(LitBool) {
        return Err(meta.error("Expected boolean literal after `no_prefix =`."));
    }

    let lit_bool = meta.input.parse::<LitBool>()?;

    *no_prefix = Some(BoolArgument::new(lit_bool.value, span));
    Ok(())
}

fn parse_const(meta: ParseNestedMeta<'_>, constant: &mut Option<BoolArgument>) -> syn::Result<()> {
    if constant.is_some() {
        return Err(meta.error("`const` specified more than once in main `#[new(...)]`."));
    }

    let span = meta.path.span();

    // #[new(const)]
    if !meta.input.peek(Token![=]) {
        *constant = Some(BoolArgument::new(true, span));
        return Ok(());
    }

    // #[new(const = ...)]
    meta.input.parse::<Token![=]>()?;

    if !meta.input.peek(LitBool) {
        return Err(meta.error("Expected boolean literal after `const =`."));
    }

    let lit_bool = meta.input.parse::<LitBool>()?;

    *constant = Some(BoolArgument::new(lit_bool.value, span));
    Ok(())
}

fn unknown_argument_error(meta: &ParseNestedMeta<'_>, item_kind: ItemKind) -> Error {
    match item_kind {
        ItemKind::Enum => {
            meta.error("Unknown enum main `#[new(...)]` argument. Expected one of: `pub`, `rename`, `no_prefix`, `const`.")
        }
        ItemKind::Struct => {
            meta.error("Unknown struct main `#[new(...)]` argument. Expected one of: `pub`, `rename`, `const`.")
        }
    }
}

fn check_invalid_main_arguments(arguments: &MainArguments) -> syn::Result<()> {
    let no_prefix = arguments
        .no_prefix
        .as_ref()
        .is_some_and(BoolArgument::value);

    if arguments.rename.is_some() && no_prefix {
        if let Some(no_prefix) = &arguments.no_prefix {
            return Err(Error::new(
                no_prefix.span,
                "`rename` cannot be combined with `no_prefix` in main `#[new(...)]`.",
            ));
        }
    }

    Ok(())
}
