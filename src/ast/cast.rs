use crate::ast::expr::Expr;
use crate::ast::scalar::ScalarKind;
use crate::ast::ty::{Type, TypeInference};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cast {
    kind: ScalarKind,
    value: Box<Expr>,
}

impl TypeInference for Cast {
    fn ty(&self) -> Type {
        Type::Scalar(self.kind)
    }
}
