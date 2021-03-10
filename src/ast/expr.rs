use crate::ast::ty::Type;
use crate::ast::sym::Symbol;
use crate::ast::scalar::ScalarKind;
use crate::ast::lit::LiteralKind;
use crate::ast::bin_op::BinOpKind;
use crate::ast::unary_op::UnaryOpKind;
use crate::ast::iter::Iter;
use crate::ast::param::Parameter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub ty: Type,
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprKind {
    Literal(LiteralKind),
    // Ident(Symbol),
    Not(Box<Expr>),
    Negate(Box<Expr>),
    Broadcast(Box<Expr>),
    BinOp {
        kind: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        kind: UnaryOpKind,
        data: Box<Expr>,
    },
    Cast {
        data: Box<Expr>,
        kind: ScalarKind,
    },
    GetField {
        data: Box<Expr>,
        index: u32,
    },
    Length(Box<Expr>),
    Lookup {
        data: Box<Expr>,
        index: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        on_true: Box<Expr>,
        on_false: Box<Expr>,
    },
    For {
        // todo: Vec<Iter>?
        iter: Iter,
        builder: Box<Expr>,
        func: Box<Expr>,
    },
    Merge {
        builder: Box<Expr>,
        value: Box<Expr>,
    },
    Lambda {
        params: Vec<Parameter>,
        body: Box<Expr>,
    },
    MakeVector(Vec<Symbol>),
}