use super::{Builder, Expr, Type, TypeInference};

/// Merge operation on builder.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Merge {
    pub(crate) builder: Box<Expr>,
    pub(crate) value: Box<Expr>,
}

impl TypeInference for Merge {
    fn ty(&self) -> Type {
        self.builder.ty()
    }
}

impl Builder for Merge {
    fn merge<T>(self, item: T) -> Merge
    where
        T: Into<Expr>,
    {
        let value: Expr = item.into();
        let merge_ty = self.ty().merge();
        assert!(
            merge_ty == value.ty(),
            "Incompatible types[{:?} and {:?}] on merge operation",
            merge_ty,
            value.ty()
        );
        Merge {
            builder: Box::new(Expr::Merge(self)),
            value: Box::new(value),
        }
    }

    fn eval_type(&self) -> Type {
        self.ty().eval()
    }
}
