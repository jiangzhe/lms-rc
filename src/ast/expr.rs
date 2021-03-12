use crate::ast::bin_op::BinOp;
use crate::ast::cast::Cast;
use crate::ast::get_field::GetField;
use crate::ast::lit::Literal;
use crate::ast::ty::{Type, TypeInference};
use crate::ast::unary_op::UnaryOp;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(TypeInference)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Literal(Literal),
    // Ident(Symbol),
    // Not(Box<Expr>),
    Not(Not),
    // Negate(Box<Expr>),
    Negate(Negate),
    // Broadcast(Box<Expr>),
    Broadcast(Broadcast),
    // BinOp {
    //     kind: BinOpKind,
    //     left: Box<Expr>,
    //     right: Box<Expr>,
    // },
    BinOp(BinOp),
    // UnaryOp {
    //     kind: UnaryOpKind,
    //     data: Box<Expr>,
    // },
    UnaryOp(UnaryOp),
    // Cast {
    //     data: Box<Expr>,
    //     kind: ScalarKind,
    // },
    Cast(Cast),
    // GetField {
    //     data: Box<Expr>,
    //     index: u32,
    // },
    GetField(GetField),
    // Length(Box<Expr>),

    // Lookup {
    //     data: Box<Expr>,
    //     index: Box<Expr>,
    // },
    // If {
    //     cond: Box<Expr>,
    //     on_true: Box<Expr>,
    //     on_false: Box<Expr>,
    // },
    // For {
    //     // todo: Vec<Iter>?
    //     iter: Iter,
    //     builder: Box<Expr>,
    //     func: Box<Expr>,
    // },
    // Merge {
    //     builder: Box<Expr>,
    //     value: Box<Expr>,
    // },
    // Lambda {
    //     params: Vec<Parameter>,
    //     body: Box<Expr>,
    // },
    // MakeVector(Vec<Symbol>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Not(Box<Expr>);

impl TypeInference for Not {
    fn ty(&self) -> Type {
        self.0.ty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Negate(Box<Expr>);

impl TypeInference for Negate {
    fn ty(&self) -> Type {
        self.0.ty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Broadcast(Box<Expr>);

impl TypeInference for Broadcast {
    fn ty(&self) -> Type {
        self.0.ty()
    }
}
