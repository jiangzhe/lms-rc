use crate::ast::sym::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Iter {
    pub data: Symbol,
    pub start: Option<Symbol>,
    pub end: Option<Symbol>,
}