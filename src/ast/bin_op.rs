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

/// A binary operation on two expressions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinOp {
    op_ty: BinOpType,
    left: Box<Expr>,
    right: Box<Expr>,
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
