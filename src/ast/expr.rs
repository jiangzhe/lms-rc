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
    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Expr::Symbol(sym) => Some(sym),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Expr::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn as_lit(&self) -> Option<&Literal> {
        match self {
            Expr::Literal(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn is_lit(&self) -> bool {
        match self {
            Expr::Literal(_) => true,
            _ => false,
        }
    }

    /// Apply given transformer to all children.
    ///
    /// If returns true, the transformation take effect on any of child.
    /// Otherwise, no effect.
    pub fn apply_children<F>(&mut self, f: &mut F) -> Result<bool>
    where
        F: ExprTransformer,
    {
        let r = match self {
            Expr::Broadcast(bc) => f.transform_expr(bc.value.as_mut())?,
            Expr::BinOp(BinOp { left, right, .. }) => {
                let mut r = f.transform_expr(left.as_mut())?;
                r |= f.transform_expr(right.as_mut())?;
                r
            }
            Expr::UnaryOp(UnaryOp { value, .. }) => f.transform_expr(value.as_mut())?,
            Expr::Cast(Cast { value, .. }) => f.transform_expr(value.as_mut())?,
            Expr::GetField(GetField { tuple, .. }) => f.transform_expr(tuple.as_mut())?,
            Expr::Length(Length(value)) => f.transform_expr(value.as_mut())?,
            Expr::Lookup(Lookup { data, index }) => {
                let mut r = f.transform_expr(data.as_mut())?;
                r |= f.transform_expr(index.as_mut())?;
                r
            }
            Expr::IfThenElse(IfThenElse { i, t, e }) => {
                let mut r = f.transform_expr(i.as_mut())?;
                r |= f.transform_expr(t.as_mut())?;
                r |= f.transform_expr(e.as_mut())?;
                r
            }
            Expr::For(For {
                iters,
                builder,
                func,
            }) => {
                let mut r = false;
                for it in iters {
                    r |= f.transform_expr(it.data.as_mut())?;
                    if let Some(start) = it.start.as_mut() {
                        r |= f.transform_expr(start.as_mut())?;
                    }
                    if let Some(end) = it.end.as_mut() {
                        r |= f.transform_expr(end.as_mut())?;
                    }
                }
                r |= f.transform_expr(builder.as_mut())?;
                r |= f.transform_expr(func.as_mut())?;
                r
            }
            Expr::Merge(Merge { builder, value }) => {
                let mut r = f.transform_expr(builder.as_mut())?;
                r |= f.transform_expr(value.as_mut())?;
                r
            }
            Expr::Lambda(lambda) => f.transform_lambda(lambda)?,
            Expr::NewVector(NewVector { items, .. }) => {
                let mut r = false;
                for it in items {
                    r |= f.transform_expr(it)?;
                }
                r
            }
            Expr::Eval(Eval(value)) => f.transform_expr(value.as_mut())?,
            // below expressions do not have children
            Expr::Symbol(_)
            | Expr::Literal(_)
            | Expr::NewDict(_)
            | Expr::NewAppender(_)
            | Expr::NewMerger(_)
            | Expr::NewDictMerger(_)
            | Expr::NewGroupMerger(_)
            | Expr::NewVecMerger(_) => false,
        };
        Ok(r)
    }

    pub fn traverse_children<F>(&self, f: &mut F) -> Result<()>
    where
        F: ExprVisitor,
    {
        match self {
            Expr::Broadcast(bc) => {
                f.visit_expr(bc.value.as_ref())?;
            }
            Expr::BinOp(BinOp { left, right, .. }) => {
                f.visit_expr(left.as_ref())?;
                f.visit_expr(right.as_ref())?;
            }
            Expr::UnaryOp(UnaryOp { value, .. }) => {
                f.visit_expr(value.as_ref())?;
            }
            Expr::Cast(Cast { value, .. }) => {
                f.visit_expr(value.as_ref())?;
            }
            Expr::GetField(GetField { tuple, .. }) => {
                f.visit_expr(tuple.as_ref())?;
            }
            Expr::Length(Length(value)) => {
                f.visit_expr(value.as_ref())?;
            }
            Expr::Lookup(Lookup { data, index }) => {
                f.visit_expr(data.as_ref())?;
                f.visit_expr(index.as_ref())?;
            }
            Expr::IfThenElse(IfThenElse { i, t, e }) => {
                f.visit_expr(i.as_ref())?;
                f.visit_expr(t.as_ref())?;
                f.visit_expr(e.as_ref())?;
            }
            Expr::For(For {
                iters,
                builder,
                func,
            }) => {
                for it in iters {
                    f.visit_expr(it.data.as_ref())?;
                    if let Some(start) = it.start.as_ref() {
                        f.visit_expr(start.as_ref())?;
                    }
                    if let Some(end) = it.end.as_ref() {
                        f.visit_expr(end.as_ref())?;
                    }
                }
                f.visit_expr(builder.as_ref())?;
                f.visit_expr(func.as_ref())?;
            }
            Expr::Merge(Merge { builder, value }) => {
                f.visit_expr(builder.as_ref())?;
                f.visit_expr(value.as_ref())?;
            }
            Expr::Lambda(lambda) => {
                f.visit_lambda(lambda)?;
            }
            Expr::NewVector(NewVector { items, .. }) => {
                for it in items {
                    f.visit_expr(it)?;
                }
            }
            Expr::Eval(Eval(value)) => {
                f.visit_expr(value.as_ref())?;
            }
            // below expressions do not have children
            Expr::Symbol(_)
            | Expr::Literal(_)
            | Expr::NewDict(_)
            | Expr::NewAppender(_)
            | Expr::NewMerger(_)
            | Expr::NewDictMerger(_)
            | Expr::NewGroupMerger(_)
            | Expr::NewVecMerger(_) => (),
        };
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
