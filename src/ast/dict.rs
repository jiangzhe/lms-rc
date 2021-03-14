use super::{Type, TypeInference};

/// Dict type contains key type and value type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictType {
    pub(crate) key_ty: Box<Type>,
    pub(crate) value_ty: Box<Type>,
}

impl_from_for_type!(DictType, Type::Dict);

/// A new Dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewDict {
    key_ty: Type,
    value_ty: Type,
}

impl TypeInference for NewDict {
    fn ty(&self) -> Type {
        Type::Dict(DictType {
            key_ty: Box::new(self.key_ty.clone()),
            value_ty: Box::new(self.value_ty.clone()),
        })
    }
}
