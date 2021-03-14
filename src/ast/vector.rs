use super::{Expr, Type, TypeInference};

/// Vector represents a variable-length list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorType {
    pub(crate) item_ty: Box<Type>,
}

impl_from_for_type!(VectorType, Type::Vector);

/// A new Vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewVector {
    // item_ty is necessary if vector is empty.
    pub(crate) item_ty: Type,
    pub(crate) items: Vec<Expr>,
}

impl TypeInference for NewVector {
    fn ty(&self) -> Type {
        Type::Vector(VectorType {
            item_ty: Box::new(self.item_ty.clone()),
        })
    }
}
