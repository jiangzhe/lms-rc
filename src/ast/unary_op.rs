use super::{Expr, Type, TypeInference};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOpType {
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

derive_display!(UnaryOpType);

/// Unary operation on single expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryOp {
    pub(super) op_ty: UnaryOpType,
    pub(super) value: Box<Expr>,
}

impl UnaryOp {
    pub fn not(value: Expr) -> Self {
        assert!(
            value.ty() == Type::Bool,
            "incompatible type on Not operator"
        );
        UnaryOp {
            op_ty: UnaryOpType::Not,
            value: Box::new(value),
        }
    }

    pub fn neg(value: Expr) -> Self {
        let ty = value.ty();
        assert!(
            ty == Type::I32 || ty == Type::I64,
            "incompatible type on Neg operator"
        );
        UnaryOp {
            op_ty: UnaryOpType::Neg,
            value: Box::new(value),
        }
    }
}

impl TypeInference for UnaryOp {
    fn ty(&self) -> Type {
        match self.op_ty {
            UnaryOpType::Not => Type::Bool,
            UnaryOpType::Neg => self.value.ty(),
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.op_ty, self.value)
    }
}
