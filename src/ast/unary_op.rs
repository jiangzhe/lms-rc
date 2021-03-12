use crate::ast::expr::Expr;
use crate::ast::ty::{Type, TypeInference};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOpKind {
    Exp,
    Log,
    Sqrt,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    Sinh,
    Cosh,
    Tanh,
    Erf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryOp {
    kind: UnaryOpKind,
    value: Box<Expr>,
}

impl TypeInference for UnaryOp {
    fn ty(&self) -> Type {
        todo!()
    }
}
