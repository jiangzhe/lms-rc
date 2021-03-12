use super::{Expr, ScalarKind, Type, TypeInference};

/// Get length of a vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Length(Box<Expr>);

impl TypeInference for Length {
    fn ty(&self) -> Type {
        Type::Scalar(ScalarKind::U64)
    }
}
