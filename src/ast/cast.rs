use super::{Expr, ScalarKind, Type, TypeInference};

/// Cast an expression to a certain type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cast {
    kind: ScalarKind,
    value: Box<Expr>,
}

impl TypeInference for Cast {
    fn ty(&self) -> Type {
        Type::Scalar(self.kind)
    }
}
