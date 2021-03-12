use super::{Expr, Type, TypeInference};

/// Different branches based on condition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfThenElse {
    i: Box<Expr>,
    t: Box<Expr>,
    e: Box<Expr>,
}

impl TypeInference for IfThenElse {
    fn ty(&self) -> Type {
        self.t.ty()
    }
}
