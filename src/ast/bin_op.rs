use super::{Expr, Type, TypeInference};

/// Types of binary operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOpType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    Xor,
    Max,
    Min,
    Pow,
}

derive_display!(BinOpType);

/// A binary operation on two expressions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinOp {
    pub(super) op_ty: BinOpType,
    pub(super) left: Box<Expr>,
    pub(super) right: Box<Expr>,
}

impl BinOp {
    pub fn add(left: Expr, right: Expr) -> Self {
        BinOp {
            op_ty: BinOpType::Add,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn sub(left: Expr, right: Expr) -> Self {
        BinOp {
            op_ty: BinOpType::Subtract,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn mul(left: Expr, right: Expr) -> Self {
        BinOp {
            op_ty: BinOpType::Multiply,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn div(left: Expr, right: Expr) -> Self {
        BinOp {
            op_ty: BinOpType::Divide,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn eq(left: Expr, right: Expr) -> Self {
        BinOp {
            op_ty: BinOpType::Equal,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn ne(left: Expr, right: Expr) -> Self {
        BinOp {
            op_ty: BinOpType::NotEqual,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl TypeInference for BinOp {
    fn ty(&self) -> Type {
        match self.op_ty {
            BinOpType::Equal
            | BinOpType::NotEqual
            | BinOpType::LessThan
            | BinOpType::LessThanOrEqual
            | BinOpType::GreaterThan
            | BinOpType::GreaterThanOrEqual
            | BinOpType::LogicalAnd
            | BinOpType::LogicalOr => Type::Bool,
            // any other operator, infer type of left operand
            _ => self.left.ty(),
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}, {})", self.op_ty, self.left, self.right)
    }
}
