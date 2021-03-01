use crate::exp::Sym;
use crate::def_arith::{Plus, Times};
use std::any::Any;
use std::fmt::Debug;

/// Generic definition
///
/// This is the root trait of all definitions.
pub trait DefEx: Debug + Any {

    /// as_any enables downcasting to actual type
    fn as_any(&self) -> &dyn Any;

    /// extract symbols from definition
    fn syms(&self) -> Vec<Sym>;
}

#[derive(PartialEq, Clone)]
pub enum Def<T> {
    Nil,
    Plus(Plus<T>),
    Times(Times<T>),
}

impl<T: Debug> Debug for Def<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Def::Nil => write!(f, "Nil"),
            Def::Plus(plus) => plus.fmt(f),
            Def::Times(times) => times.fmt(f),
        }
    }
}

impl<T: Debug + Clone + 'static> DefEx for Def<T> {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn syms(&self) -> Vec<Sym> {
        match self {
            Def::Nil => vec![],
            Def::Plus(plus) => plus.syms(),
            Def::Times(times) => times.syms(),
        }
    }
}
