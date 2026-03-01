use syn::Attribute;

pub(crate) fn collect(attributes: &[Attribute]) -> Vec<Attribute> {
    attributes
        .iter()
        .filter(|attribute| {
            attribute.path().is_ident("allow")
                || attribute.path().is_ident("warn")
                || attribute.path().is_ident("deny")
                || attribute.path().is_ident("forbid")
        })
        .cloned()
        .collect()
}
