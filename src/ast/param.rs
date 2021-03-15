use crate::ast::sym::Symbol;
use crate::ast::ty::Type;

/// Parameter represents the input parameter of a function/lambda.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub(super) name: Symbol,
    pub(super) ty: Type,
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parameter({})", self.name)
    }
}
