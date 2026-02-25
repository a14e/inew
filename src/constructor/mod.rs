pub(crate) mod generator;
pub(crate) mod linter;
pub(crate) mod options;
pub(crate) mod plan;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ItemKind {
    Struct,
    Enum,
}
