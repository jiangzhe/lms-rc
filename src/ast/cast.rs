use super::{Expr, Type, TypeInference};

/// Cast an expression to a certain type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cast {
    ty: Type,
    value: Box<Expr>,
}

impl TypeInference for Cast {
    fn ty(&self) -> Type {
        self.ty.clone()
    }
}
