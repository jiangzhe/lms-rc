use crate::def::{Def, DefEx};
use crate::exp::Sym;
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

/// Trait of statement
///
/// It extends Any to allow downcast to
/// actual type, and define as_any() to
/// allow unwrap as dyn Any
pub trait StmtEx: Any + Debug {
    fn as_any(&self) -> &dyn Any;

    fn lhs(&self) -> Sym;

    fn rhs(&self) -> &dyn DefEx;
}

/// Statement
///
/// Links symbols and definitions
#[derive(Debug, Clone)]
pub enum Stmt<T> {
    // equivalent to TP case class in scala-lms
    Assign(Assign<T>),
}

impl<T: 'static + Clone + Debug> StmtEx for Stmt<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn lhs(&self) -> Sym {
        self.sym()
    }

    fn rhs(&self) -> &dyn DefEx {
        self.def()
    }
}

impl<T: 'static + Clone + Debug> Stmt<T> {
    pub fn assign(sym: Sym, rhs: Def<T>) -> Self {
        let a = Assign {
            sym,
            def: Rc::new(rhs),
            _marker: PhantomData,
        };
        Stmt::Assign(a)
    }

    pub fn sym(&self) -> Sym {
        match self {
            Stmt::Assign(a) => a.sym.clone(),
        }
    }

    pub fn def(&self) -> &dyn DefEx {
        match self {
            Stmt::Assign(a) => &*a.def,
        }
    }

    pub fn defines(&self, sym: Sym) -> Option<&Def<T>> {
        match self {
            Stmt::Assign(a) if a.sym == sym => a.def.as_any().downcast_ref::<Def<T>>(),
            Stmt::Assign(_) => None,
        }
    }

    pub fn defines_rhs(&self, rhs: &Def<T>) -> Option<&Sym>
    where
        T: PartialEq,
    {
        match self {
            Stmt::Assign(a) => {
                if let Some(d) = a.def.as_any().downcast_ref::<Def<T>>() {
                    if d == rhs {
                        return Some(&a.sym);
                    }
                }
                None
            }
        }
    }
}

#[derive(Clone)]
pub struct Assign<T> {
    pub sym: Sym,
    pub def: Rc<dyn DefEx>,
    _marker: PhantomData<T>,
}

impl<T: Debug> Debug for Assign<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}={:?}", self.sym, self.def)
    }
}
