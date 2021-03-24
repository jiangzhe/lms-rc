use super::{Expr, Type, TypeInference};

/// Iteration on data with optional start and end expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Iter {
    pub(crate) data: Box<Expr>,
    pub(crate) start: Option<Box<Expr>>,
    pub(crate) end: Option<Box<Expr>>,
}

impl TypeInference for Iter {
    fn ty(&self) -> Type {
        todo!()
    }
}

impl std::fmt::Display for Iter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_str("Iter(")?;
        self.data.fmt(f)?;
        match (self.start.as_ref(), self.end.as_ref()) {
            (None, None) => f.write_char(')'),
            (Some(s), None) => write!(f, "_, {})", s),
            (None, Some(e)) => write!(f, "{}, _)", e),
            (Some(s), Some(e)) => write!(f, "{}, {})", s, e),
        }
    }
}
