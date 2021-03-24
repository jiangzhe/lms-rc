mod uniquify;

use crate::ast::{Builder, Expr, Merge, Type, TypeInference};

/// Symbol represents a named variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub(crate) name: String,
    pub(crate) ty: Type,
    pub(crate) id: u32,
}

impl Symbol {
    #[inline]
    pub fn named<S: Into<String>, T: Into<Type>>(name: S, ty: T) -> Self {
        Self::new(name, ty, 0)
    }

    #[inline]
    pub fn unamed<T: Into<Type>>(ty: T) -> Self {
        Self::named("_", ty)
    }

    #[inline]
    pub fn new<S: Into<String>, T: Into<Type>>(name: S, ty: T, id: u32) -> Self {
        Symbol {
            name: name.into(),
            ty: ty.into(),
            id,
        }
    }
}

impl TypeInference for Symbol {
    fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl Builder for Symbol {
    fn merge<T>(self, item: T) -> Merge
    where
        T: Into<Expr>,
    {
        assert!(
            self.ty().is_builder(),
            "Non-bulder type[{:?}] on merge operation",
            self.ty()
        );
        let value: Expr = item.into();
        Merge {
            builder: Box::new(Expr::Symbol(self)),
            value: Box::new(value),
        }
    }

    fn eval_type(&self) -> Type {
        self.ty().eval()
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", self.name, self.id)
    }
}
