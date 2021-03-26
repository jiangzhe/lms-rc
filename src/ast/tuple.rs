use super::{Expr, Type, TypeInference};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType(pub(crate) Vec<Type>);

impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('(')?;
        for (i, t) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            t.fmt(f)?;
        }
        f.write_char(')')
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewTuple(pub(crate) Vec<Expr>);

impl TypeInference for NewTuple {
    fn ty(&self) -> Type {
        Type::Tuple(TupleType(self.0.iter().map(|e| e.ty()).collect()))
    }
}

impl std::fmt::Display for NewTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('(')?;
        for (i, t) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            t.fmt(f)?;
        }
        f.write_char(')')
    }
}
