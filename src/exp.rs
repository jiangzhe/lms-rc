use crate::ctx::{Context, WithCtx};
use crate::def::Def;
use std::fmt::Debug;

/// Generic expression
///
/// It contains two variants, constant and symbol. 
/// Symbol is numbered with global identifier,
/// and registered with Context. It also embeds the
/// context inside for future comuputation. 
#[derive(Clone, PartialEq)]
pub enum Exp<T> {
    Const(T),
    Sym(Sym),
}

impl<T: Debug> Debug for Exp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Const(c) => c.fmt(f),
            Exp::Sym(s) => s.fmt(f),
        }
    }
}

impl<T> Exp<T> {
    pub fn cst(value: T) -> Self {
        Exp::Const(value)
    }

    pub fn as_cst(&self) -> Option<&T> {
        match self {
            Exp::Const(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_sym(&self) -> Option<&Sym> {
        match self {
            Exp::Sym(sym) => Some(sym),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sym(pub(super) usize);

/// Represent a intermediate value of T
///
/// this value can be converted to Exp by calling
/// context.eval method, which will register the
/// definition with a fresh symbol.
#[derive(Clone)]
pub enum ExpOrDef<T> {
    Exp(Exp<T>),
    Def(Def<T>)
}

impl<T: Debug + Clone + PartialEq + 'static> WithCtx for ExpOrDef<T> {
    type Target = Exp<T>;

    fn with(self, ctx: &mut Context) -> Self::Target {
        match self {
            ExpOrDef::Exp(e) => e,
            ExpOrDef::Def(d) => {
                let stmt = ctx.find_or_create_stmt(d);
                Exp::Sym(stmt.lhs())
            }
        }
    }
}