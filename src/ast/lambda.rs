use super::{Expr, Parameter, Type, TypeInference};

/// Lambda represents an anonymous function
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lambda {
    pub(super) params: Vec<Parameter>,
    pub(super) body: Box<Expr>,
}

impl TypeInference for Lambda {
    fn ty(&self) -> Type {
        todo!()
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_str("Lambda(")?;
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            p.fmt(f)?;
        }
        write!(f, "->{})", self.body)
    }
}
