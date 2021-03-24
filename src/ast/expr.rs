use super::*;
use crate::sym::Symbol;
use crate::Result;
use enum_dispatch::enum_dispatch;

/// Expr represents an expression tree.
#[enum_dispatch(TypeInference)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// A literal expression.
    Literal(Literal),
    /// An identifier.
    Symbol(Symbol),
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

impl Expr {
    /// Apply given transformer to all children.
    pub fn apply_to_children<F>(&mut self, f: &mut F) -> Result<()>
    where
        F: Transformer<Expr> + Transformer<Symbol>,
    {
        match self {
            Expr::Broadcast(bc) => f.transform(bc.value.as_mut())?,
            Expr::BinOp(BinOp { left, right, .. }) => {
                f.transform(left.as_mut())?;
                f.transform(right.as_mut())?;
            }
            Expr::UnaryOp(UnaryOp { value, .. }) => f.transform(value.as_mut())?,
            Expr::Cast(Cast { value, .. }) => f.transform(value.as_mut())?,
            Expr::GetField(GetField { tuple, .. }) => f.transform(tuple.as_mut())?,
            Expr::Length(Length(value)) => f.transform(value.as_mut())?,
            Expr::Lookup(Lookup { data, index }) => {
                f.transform(data.as_mut())?;
                f.transform(index.as_mut())?;
            }
            Expr::IfThenElse(IfThenElse { i, t, e }) => {
                f.transform(i.as_mut())?;
                f.transform(t.as_mut())?;
                f.transform(e.as_mut())?;
            }
            Expr::For(For {
                iters,
                builder,
                func,
            }) => {
                for it in iters {
                    f.transform(it.data.as_mut())?;
                    if let Some(start) = it.start.as_mut() {
                        f.transform(start.as_mut())?;
                    }
                    if let Some(end) = it.end.as_mut() {
                        f.transform(end.as_mut())?;
                    }
                }
                f.transform(builder.as_mut())?;
                f.transform(func.as_mut())?;
            }
            Expr::Merge(Merge { builder, value }) => {
                f.transform(builder.as_mut())?;
                f.transform(value.as_mut())?;
            }
            Expr::Lambda(Lambda { params, body }) => {
                for p in params {
                    f.transform(p)?;
                }
                f.transform(body.as_mut())?;
            }
            Expr::NewVector(NewVector { items, .. }) => {
                for it in items {
                    f.transform(it)?;
                }
            }
            Expr::Eval(Eval(value)) => f.transform(value.as_mut())?,
            Expr::Symbol(_)
            | Expr::Literal(_)
            | Expr::NewDict(_)
            | Expr::NewAppender(_)
            | Expr::NewMerger(_)
            | Expr::NewDictMerger(_)
            | Expr::NewGroupMerger(_)
            | Expr::NewVecMerger(_) => (),
        }

        Ok(())
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(lit) => lit.fmt(f),
            Expr::Symbol(sym) => sym.fmt(f),
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
