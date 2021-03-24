use super::{Expr, Type, TypeInference};

/// Get value from tuple by given index.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GetField {
    pub(crate) ty: Type,
    pub(crate) tuple: Box<Expr>,
    pub(crate) index: u32,
}

impl TypeInference for GetField {
    fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl std::fmt::Display for GetField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetField({}, {})", self.tuple, self.index)
    }
}
