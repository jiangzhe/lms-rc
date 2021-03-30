use super::{Expr, Type, TypeInference};

/// Vector represents a variable-length list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorType {
    pub(crate) item_ty: Box<Type>,
}

impl_from_for_type!(VectorType, Type::Vector);

impl std::fmt::Display for VectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.item_ty)
    }
}

/// A new Vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vector {
    // item_ty is necessary if vector is empty.
    pub(crate) item_ty: Type,
    pub(crate) items: Vec<Expr>,
}

impl TypeInference for Vector {
    fn ty(&self) -> Type {
        Type::Vector(VectorType {
            item_ty: Box::new(self.item_ty.clone()),
        })
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "NewVector<{}>", self.item_ty)?;
        f.write_char('(')?;
        for (i, e) in self.items.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            e.fmt(f)?;
        }
        f.write_char(')')
    }
}
