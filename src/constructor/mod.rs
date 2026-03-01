pub(crate) mod field_options;
pub(crate) mod generator;
pub(crate) mod lint_extractor;
pub(crate) mod main_options;

use proc_macro2::Span;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ItemKind {
    Struct,
    Enum,
}

pub(crate) struct BoolArgument {
    pub value: bool,
    pub span: Span,
}

impl BoolArgument {
    fn new(value: bool, span: Span) -> Self {
        Self { value, span }
    }
    
    fn value(&self) -> bool {
        self.value
    }
}
