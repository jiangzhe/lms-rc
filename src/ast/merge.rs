use super::{Expr, Type, TypeInference};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Merge {
    builder: Box<Expr>,
    value: Box<Expr>,
}

impl TypeInference for Merge {
    fn ty(&self) -> Type {
        todo!()
    }
}
