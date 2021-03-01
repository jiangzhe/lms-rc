use crate::exp::{Exp, Sym};
use crate::block::Block;
use std::fmt::Formatter;
use std::any::Any;

pub trait Codegen {

    fn emit_source<T>(args: &[Sym], block: Block<T>, class_name: String, out: &mut Formatter<'_>) -> Vec<(Sym, Box<dyn Any>)>;

    fn emit_source1<T, U, F>(f: F, class_name: String, out: &mut Formatter<'_>) -> Vec<(Sym, Box<dyn Any>)> 
    where 
        F: Fn(Exp<T>) -> Exp<U>, 
    {
        // let s = Exp::fresh();
        todo!()
    }
}
