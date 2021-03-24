use super::{Expr, Type, TypeInference};

/// Different branches based on condition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfThenElse {
    pub(crate) i: Box<Expr>,
    pub(crate) t: Box<Expr>,
    pub(crate) e: Box<Expr>,
}

impl TypeInference for IfThenElse {
    fn ty(&self) -> Type {
        self.t.ty()
    }
}

impl std::fmt::Display for IfThenElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IfThenElse({}, {}, {})", self.i, self.t, self.e)
    }
}
