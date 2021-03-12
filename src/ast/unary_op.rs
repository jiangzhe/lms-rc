use super::{Expr, ScalarKind, Type, TypeInference};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOpKind {
    // Invert a boolean expression
    Not,
    // Negates a numerical expression
    Neg,
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
    pub kind: UnaryOpKind,
    pub value: Box<Expr>,
}

impl UnaryOp {
    pub fn not(value: Expr) -> Self {
        assert!(
            value.ty() == Type::Scalar(ScalarKind::Bool),
            "incompatible type on Not operator"
        );
        UnaryOp {
            kind: UnaryOpKind::Not,
            value: Box::new(value),
        }
    }

    pub fn neg(value: Expr) -> Self {
        let ty = value.ty();
        assert!(
            ty == Type::Scalar(ScalarKind::I32) || ty == Type::Scalar(ScalarKind::I64),
            "incompatible type on Neg operator"
        );
        UnaryOp {
            kind: UnaryOpKind::Neg,
            value: Box::new(value),
        }
    }
}

impl TypeInference for UnaryOp {
    fn ty(&self) -> Type {
        todo!()
    }
}
