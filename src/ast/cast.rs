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

impl std::fmt::Display for Cast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cast<{}>({})", self.ty, self.value)
    }
}
