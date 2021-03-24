use crate::sym::Symbol;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StmtIter {
    pub(super) data: Symbol,
    pub(super) start: Option<Symbol>,
    pub(super) end: Option<Symbol>,
}
