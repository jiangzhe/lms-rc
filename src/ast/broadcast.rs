use super::{Expr, Type, TypeInference, VectorType};

/// A broadcast value acting like a vector with identical items.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Broadcast {
    pub(super) value: Box<Expr>,
}

impl TypeInference for Broadcast {
    fn ty(&self) -> Type {
        Type::Vector(VectorType {
            item_ty: Box::new(self.value.ty()),
        })
    }
}

impl std::fmt::Display for Broadcast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Broadcast({})", self.value)
    }
}
