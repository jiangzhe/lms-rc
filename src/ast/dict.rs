use super::{Expr, Type, TypeInference};

/// Dict type contains key type and value type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictKind(Box<Type>, Box<Type>);

/// A new Dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewDict(Box<Expr>);

impl TypeInference for NewDict {
    fn ty(&self) -> Type {
        todo!()
    }
}
