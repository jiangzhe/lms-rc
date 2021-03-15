use super::Type;

/// Function with specified argument types and return type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    args_ty: Vec<Type>,
    ret_ty: Box<Type>,
}

impl std::fmt::Display for FunctionType {
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
