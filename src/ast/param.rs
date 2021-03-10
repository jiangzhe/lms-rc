use crate::ast::sym::Symbol;
use crate::ast::ty::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub name: Symbol,
    pub ty: Type,
}