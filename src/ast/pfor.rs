use super::{Expr, Iter, Type, TypeInference};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct For {
    iters: Vec<Iter>,
    builder: Box<Expr>,
    func: Box<Expr>,
}

impl TypeInference for For {
    fn ty(&self) -> Type {
        todo!()
    }
}
