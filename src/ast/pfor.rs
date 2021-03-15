use super::{Builder, Expr, Iter, Merge, Type, TypeInference};

/// For represents a loop in parallel.
///
/// The usage of For already be coupled with some builder.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct For {
    pub(super) iters: Vec<Iter>,
    pub(super) builder: Box<Expr>,
    pub(super) func: Box<Expr>,
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

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_str("For([")?;
        for (i, t) in self.iters.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            t.fmt(f)?;
        }
        write!(f, "], {}, {})", self.builder, self.func)
    }
}
