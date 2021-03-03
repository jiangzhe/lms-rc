use crate::def::DefEx;
use crate::exp::{Exp, Sym};
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Plus<T>(pub Exp<T>, pub Exp<T>);

impl<T> DefEx for Plus<T> where T: Debug + Clone + 'static {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn syms(&self) -> Vec<Sym> {
        match (&self.0, &self.1) {
            (Exp::Sym(x), Exp::Sym(y)) => vec![x.clone(), y.clone()],
            (Exp::Sym(x), _) => vec![x.clone()],
            (_, Exp::Sym(y)) => vec![y.clone()],
            _ => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Minus<T>(pub Exp<T>, pub Exp<T>);

impl<T> DefEx for Minus<T> where T: Debug + Clone + 'static {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn syms(&self) -> Vec<Sym> {
        match (&self.0, &self.1) {
            (Exp::Sym(x), Exp::Sym(y)) => vec![x.clone(), y.clone()],
            (Exp::Sym(x), _) => vec![x.clone()],
            (_, Exp::Sym(y)) => vec![y.clone()],
            _ => vec![],
        } 
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Times<T>(pub Exp<T>, pub Exp<T>);

impl<T> DefEx for Times<T> where T: Debug + Clone + 'static {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn syms(&self) -> Vec<Sym> {
        match (&self.0, &self.1) {
            (Exp::Sym(x), Exp::Sym(y)) => vec![x.clone(), y.clone()],
            (Exp::Sym(x), _) => vec![x.clone()],
            (_, Exp::Sym(y)) => vec![y.clone()],
            _ => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Divides<T>(pub Exp<T>, pub Exp<T>);

impl<T> DefEx for Divides<T> where T: Debug + Clone + 'static {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn syms(&self) -> Vec<Sym> {
        match (&self.0, &self.1) {
            (Exp::Sym(x), Exp::Sym(y)) => vec![x.clone(), y.clone()],
            (Exp::Sym(x), _) => vec![x.clone()],
            (_, Exp::Sym(y)) => vec![y.clone()],
            _ => vec![],
        } 
    }
}