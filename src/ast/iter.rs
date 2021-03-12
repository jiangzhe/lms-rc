use super::{Expr, Type, TypeInference};

/// Iteration on data with optional start and end expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Iter {
    pub data: Box<Expr>,
    pub start: Option<Box<Expr>>,
    pub end: Option<Box<Expr>>,
}

impl TypeInference for Iter {
    fn ty(&self) -> Type {
        todo!()
    }
}
