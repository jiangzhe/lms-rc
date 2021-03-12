use super::{Expr, Type, TypeInference};

/// A broadcast value acting like a vector with identical items.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Broadcast(Box<Expr>);

impl TypeInference for Broadcast {
    fn ty(&self) -> Type {
        self.0.ty()
    }
}
