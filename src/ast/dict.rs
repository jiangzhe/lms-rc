use super::{Type, TypeInference};

/// Dict type contains key type and value type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictType {
    pub(crate) key_ty: Box<Type>,
    pub(crate) value_ty: Box<Type>,
}

impl_from_for_type!(DictType, Type::Dict);

impl std::fmt::Display for DictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.key_ty, self.value_ty)
    }
}

/// A new Dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dict {
    pub(crate) key_ty: Type,
    pub(crate) value_ty: Type,
}

impl TypeInference for Dict {
    fn ty(&self) -> Type {
        Type::Dict(DictType {
            key_ty: Box::new(self.key_ty.clone()),
            value_ty: Box::new(self.value_ty.clone()),
        })
    }
}

impl std::fmt::Display for Dict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewDict<{}, {}>", self.key_ty, self.value_ty)
    }
}
