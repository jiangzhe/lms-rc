use super::{Builder, Expr, Iter, Merge, Type, TypeInference};

/// For represents a loop in parallel.
///
/// The usage of For already be coupled with some builder.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct For {
    iters: Vec<Iter>,
    builder: Box<Expr>,
    func: Box<Expr>,
}

impl TypeInference for For {
    fn ty(&self) -> Type {
        self.builder.ty()
    }
}

impl Builder for For {
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
            builder: Box::new(Expr::For(self)),
            value: Box::new(value),
        }
    }

    fn eval_type(&self) -> Type {
        self.ty().eval()
    }
}
