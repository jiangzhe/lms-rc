use super::{Expr, Type, TypeInference, U64};

/// Get length of a vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Length(pub(crate) Box<Expr>);

impl TypeInference for Length {
    fn ty(&self) -> Type {
        Type::U64(U64)
    }
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length({})", self.0)
    }
}
