use super::{Expr, Type, TypeInference};

/// Get value from tuple by given index.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetField {
    ty: Type,
    tuple: Box<Expr>,
    index: u32,
}

impl TypeInference for GetField {
    fn ty(&self) -> Type {
        self.ty.clone()
    }
}
