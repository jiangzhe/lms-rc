use super::{DynamicType, StaticType, Symbol, Type};

/// Parameter represents the input parameter of a function/lambda.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub(super) sym: Symbol,
    pub(super) ty: Type,
}

impl Parameter {
    pub fn new(sym: Symbol, ty: Type) -> Self {
        Parameter { sym, ty }
    }

    pub fn new_static<T>(sym: Symbol) -> Self
    where
        T: StaticType,
    {
        let ty = T::ty();
        Parameter { sym, ty }
    }

    pub fn new_dynamic<T>(sym: Symbol, ty: T) -> Self
    where
        T: DynamicType,
    {
        let ty = ty.ty();
        Parameter { sym, ty }
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parameter<{}>({})", self.ty, self.sym)
    }
}
