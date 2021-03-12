use super::{Expr, Parameter, Type, TypeInference};

/// Lambda represents an anonymous function
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lambda {
    params: Vec<Parameter>,
    body: Box<Expr>,
}

impl TypeInference for Lambda {
    fn ty(&self) -> Type {
        todo!()
    }
}
