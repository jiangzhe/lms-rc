use super::{Expr, Type, TypeInference};

/// Evaluation on builder.
///
/// e.g. evaluating a Appender will return a vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eval(pub(crate) Box<Expr>);

impl TypeInference for Eval {
    fn ty(&self) -> Type {
        self.0.ty().eval()
    }
}

impl std::fmt::Display for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Eval({})", self.0)
    }
}
