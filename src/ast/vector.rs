use super::{Expr, Type, TypeInference};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorKind(Box<Type>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewVector(Vec<Expr>);

impl TypeInference for NewVector {
    fn ty(&self) -> Type {
        todo!()
    }
}
