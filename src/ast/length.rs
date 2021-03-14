use super::{Expr, Type, TypeInference};

/// Get length of a vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Length(Box<Expr>);

impl TypeInference for Length {
    fn ty(&self) -> Type {
        Type::U64
    }
}
