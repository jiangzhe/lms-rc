use super::{Expr, Type, TypeInference};

/// Lookup value in dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookup {
    data: Box<Expr>,
    index: Box<Expr>,
}

impl TypeInference for Lookup {
    fn ty(&self) -> Type {
        todo!()
    }
}
