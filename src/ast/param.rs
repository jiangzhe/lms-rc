use crate::ast::sym::Symbol;
use crate::ast::ty::Type;

/// Parameter represents the input parameter of a function/lambda.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub name: Symbol,
    pub ty: Type,
}
