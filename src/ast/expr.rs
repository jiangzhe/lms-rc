use crate::ast::ty::Type;
use crate::ast::sym::Symbol;
use crate::ast::scalar::ScalarKind;
use crate::ast::lit::LiteralKind;
use crate::ast::bin_op::BinOpKind;
use crate::ast::unary_op::UnaryOpKind;
use crate::ast::iter::Iter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub ty: Type,
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprKind {
    Literal(LiteralKind),
    // Ident(Symbol),
    Not(Symbol),
    Negate(Symbol),
    Broadcast(Symbol),
    BinOp {
        kind: BinOpKind,
        left: Symbol,
        right: Symbol,
    },
    UnaryOp {
        kind: UnaryOpKind,
        data: Symbol,
    },
    Cast {
        data: Symbol,
        kind: ScalarKind,
    },
    GetField {
        data: Symbol,
        index: u32,
    },
    Length(Symbol),
    Lookup {
        data: Symbol,
        index: Symbol,
    },
    If {
        cond: Symbol,
        on_true: Symbol,
        on_false: Symbol,
    },
    For {
        iter: Iter,
        builder: Symbol,
        func: Symbol,
    },
    Merge {
        builder: Symbol,
        value: Symbol,
    },
    Lambda {
        params: Vec<Symbol>,
        body: Symbol,
    },
}