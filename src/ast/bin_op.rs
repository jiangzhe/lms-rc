use crate::ast::expr::Expr;
use crate::ast::scalar::ScalarKind;
use crate::ast::ty::{Type, TypeInference};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOpKind {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinOp {
    kind: BinOpKind,
    left: Box<Expr>,
    right: Box<Expr>,
}

impl TypeInference for BinOp {
    fn ty(&self) -> Type {
        match self.kind {
            BinOpKind::Add
            | BinOpKind::Subtract
            | BinOpKind::Multiply
            | BinOpKind::Divide
            | BinOpKind::Modulo => self.left.ty(),
            BinOpKind::Equal
            | BinOpKind::NotEqual
            | BinOpKind::LessThan
            | BinOpKind::LessThanOrEqual
            | BinOpKind::GreaterThan
            | BinOpKind::GreaterThanOrEqual
            | BinOpKind::LogicalAnd
            | BinOpKind::LogicalOr => Type::Scalar(ScalarKind::Bool),
            BinOpKind::BitwiseAnd | BinOpKind::BitwiseOr | BinOpKind::Xor => self.left.ty(),
            BinOpKind::Max | BinOpKind::Min | BinOpKind::Pow => self.left.ty(),
        }
    }
}
