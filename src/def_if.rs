use crate::block::Block;
use crate::def::DefEx;
use crate::exp::{Exp, Sym};
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct IfThenElse<T> {
    cond: Exp<bool>,
    then_block: Block<T>,
    else_block: Block<T>,
}

impl<T> DefEx for IfThenElse<T>
where
    T: Debug + Clone + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn syms(&self) -> Vec<Sym> {
        let mut ss = vec![];
        if let Exp::Sym(s) = &self.cond {
            ss.push(s.clone());
        }
        if let Exp::Sym(s) = &self.then_block.res {
            ss.push(s.clone());
        }
        if let Exp::Sym(s) = &self.else_block.res {
            ss.push(s.clone());
        }
        ss
    }
}
