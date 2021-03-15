use super::*;
use enum_dispatch::enum_dispatch;

/// Expr represents an expression tree.
#[enum_dispatch(TypeInference)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// A literal expression.
    Literal(Literal),
    /// Broadcasts a scalar into a vector.
    Broadcast(Broadcast),
    /// Applies a binary operator to child expressions.
    BinOp(BinOp),
    /// Applies a unary operator to child expressions.
    UnaryOp(UnaryOp),
    /// Cast a scalar expression to another type.
    Cast(Cast),
    /// Access a tuple field at given index.
    GetField(GetField),
    /// Get the length of a vector as an u64.
    Length(Length),
    /// Lookup a value in Dict.
    Lookup(Lookup),
    /// Evaluate different branch based on condition.
    IfThenElse(IfThenElse),
    /// Update a builder in parallel by itearating over data.
    For(For),
    /// Update a builder value, returning a new builder.
    Merge(Merge),
    /// An expression representing a function.
    Lambda(Lambda),
    /// Construct a new vector.
    NewVector(NewVector),
    /// Construct a new dictionary.
    NewDict(NewDict),
    /// Construct a new appender.
    NewAppender(NewAppender),
    /// Construct a new merger.
    NewMerger(NewMerger),
    /// Construct a new dictmerger.
    NewDictMerger(NewDictMerger),
    /// Construct a new groupmerger.
    NewGroupMerger(NewGroupMerger),
    /// Construct a new vecmerger.
    NewVecMerger(NewVecMerger),
    /// Consume a builder and return its result
    Eval(Eval),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(lit) => lit.fmt(f),
            Expr::Broadcast(bc) => bc.fmt(f),
            Expr::BinOp(bo) => bo.fmt(f),
            Expr::UnaryOp(uo) => uo.fmt(f),
            Expr::Cast(c) => c.fmt(f),
            Expr::GetField(gf) => gf.fmt(f),
            Expr::Length(len) => len.fmt(f),
            Expr::Lookup(lkp) => lkp.fmt(f),
            Expr::IfThenElse(ite) => ite.fmt(f),
            Expr::For(fr) => fr.fmt(f),
            Expr::Merge(mg) => mg.fmt(f),
            Expr::Lambda(lmd) => lmd.fmt(f),
            Expr::NewVector(nv) => nv.fmt(f),
            Expr::NewDict(nd) => nd.fmt(f),
            Expr::NewAppender(na) => na.fmt(f),
            Expr::NewMerger(nm) => nm.fmt(f),
            Expr::NewDictMerger(ndm) => ndm.fmt(f),
            Expr::NewGroupMerger(ngm) => ngm.fmt(f),
            Expr::NewVecMerger(nvm) => nvm.fmt(f),
            Expr::Eval(ev) => ev.fmt(f),
        }
    }
}
