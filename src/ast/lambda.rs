use super::{Expr, Symbol, Type, TypeInference};

/// LambdaType specifies argument types and return type of a lambda
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaType {
    args_ty: Vec<Type>,
    ret_ty: Box<Type>,
}

impl std::fmt::Display for LambdaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('(')?;
        for (i, arg) in self.args_ty.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            arg.fmt(f)?;
        }
        write!(f, ")->{}", self.ret_ty)
    }
}

/// Lambda represents an anonymous function
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lambda {
    pub(super) params: Vec<Symbol>,
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
        f.write_str("|")?;
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            p.fmt(f)?;
        }
        write!(f, "|->{{{}}}", self.body)
    }
}
