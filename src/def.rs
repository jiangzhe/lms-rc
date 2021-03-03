use crate::exp::Sym;
use crate::def_arith::{Plus, Minus, Times, Divides};
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
    Minus(Minus<T>),
    Times(Times<T>),
    Divides(Divides<T>),
}

impl<T: Debug> Debug for Def<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Def::Nil => write!(f, "Nil"),
            Def::Plus(plus) => plus.fmt(f),
            Def::Minus(minus) => minus.fmt(f),
            Def::Times(times) => times.fmt(f),
            Def::Divides(divides) => divides.fmt(f),
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
            Def::Minus(minus) => minus.syms(),
            Def::Times(times) => times.syms(),
            Def::Divides(divides) => divides.syms(),
        }
    }
}
