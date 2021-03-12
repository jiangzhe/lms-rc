use super::{Expr, Type, TypeInference};

/// Evaluation on builder.
///
/// e.g. evaluating a Appender will return a vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eval(Box<Expr>);

impl TypeInference for Eval {
    fn ty(&self) -> Type {
        todo!()
    }
}
