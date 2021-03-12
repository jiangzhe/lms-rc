use crate::ast::expr::Expr;
use crate::ast::scalar::ScalarKind;
use crate::ast::ty::{Type, TypeInference};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetField {
    data: Box<Expr>,
    index: u32,
}

impl TypeInference for GetField {
    fn ty(&self) -> Type {
        todo!()
    }
}
