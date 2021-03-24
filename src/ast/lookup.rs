use super::{Expr, Type, TypeInference};

/// Lookup value in dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookup {
    pub(crate) data: Box<Expr>,
    pub(crate) index: Box<Expr>,
}

impl TypeInference for Lookup {
    fn ty(&self) -> Type {
        todo!()
    }
}

impl std::fmt::Display for Lookup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lookup({}, {})", self.data, self.index)
    }
}
