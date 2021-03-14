use super::*;

/// Type classifies each value and defines its behavior.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// A scalar.
    // Scalar(ScalarType),
    Bool,
    U8,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    String,
    /// A variable-length vector.
    Vector(VectorType),
    /// A dictionary mapping keys to values.
    Dict(DictType),
    /// An ordered tuple.
    Tuple(TupleType),
    /// A function with a list of arguments and return type.
    Function(FunctionType),
    /// A mutable builder to append item to a list.
    Appender(AppenderType),
    /// A builder that constructs a scalar.
    Merger(MergerType),
    /// A builder that creates a dictionary.
    DictMerger(DictMergerType),
    /// A builder that creates groups.
    GroupMerger(GroupMergerType),
    /// A builder that constructs a vector by updating elements.
    VecMerger(VecMergerType),
    /// An unknown type, used only before type inference.
    Unknown,
}

impl Type {
    #[inline]
    pub fn is_scalar(&self) -> bool {
        match self {
            Type::Bool
            | Type::U8
            | Type::U32
            | Type::I32
            | Type::U64
            | Type::I64
            | Type::F32
            | Type::F64
            | Type::String => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        match self {
            Type::Bool => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_u8(&self) -> bool {
        match self {
            Type::U8 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_u32(&self) -> bool {
        match self {
            Type::U32 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_i32(&self) -> bool {
        match self {
            Type::I32 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        match self {
            Type::U64 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_i64(&self) -> bool {
        match self {
            Type::I64 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_f32(&self) -> bool {
        match self {
            Type::F32 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        match self {
            Type::F64 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        match self {
            Type::String => true,
            _ => false,
        }
    }

    #[inline]
    pub fn vector(self) -> VectorType {
        match self {
            Type::Vector(vk) => vk,
            _ => panic!("{:?} is not vector", self),
        }
    }

    #[inline]
    pub fn is_vector(&self) -> bool {
        match self {
            Type::Vector(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn dict(self) -> DictType {
        match self {
            Type::Dict(dk) => dk,
            _ => panic!("{:?} is not dict", self),
        }
    }

    #[inline]
    pub fn is_dict(&self) -> bool {
        match self {
            Type::Dict(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn appender(self) -> AppenderType {
        match self {
            Type::Appender(ak) => ak,
            _ => panic!("{:?} is not appender", self),
        }
    }

    #[inline]
    pub fn is_appender(&self) -> bool {
        match self {
            Type::Appender(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn merger(self) -> MergerType {
        match self {
            Type::Merger(mk) => mk,
            _ => panic!("{:?} is not merger", self),
        }
    }

    #[inline]
    pub fn is_merger(&self) -> bool {
        match self {
            Type::Merger(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn dictmerger(self) -> DictMergerType {
        match self {
            Type::DictMerger(dmk) => dmk,
            _ => panic!("{:?} is not dictmerger", self),
        }
    }

    #[inline]
    pub fn is_dictmerger(&self) -> bool {
        match self {
            Type::DictMerger(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn groupmerger(self) -> GroupMergerType {
        match self {
            Type::GroupMerger(gmk) => gmk,
            _ => panic!("{:?} is not groupmerger", self),
        }
    }

    #[inline]
    pub fn is_groupmerger(&self) -> bool {
        match self {
            Type::GroupMerger(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn vecmerger(self) -> VecMergerType {
        match self {
            Type::VecMerger(vmk) => vmk,
            _ => panic!("{:?} is not vecmerger", self),
        }
    }

    #[inline]
    pub fn is_vecmerger(&self) -> bool {
        match self {
            Type::VecMerger(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn tuple(self) -> TupleType {
        match self {
            Type::Tuple(tk) => tk,
            _ => panic!("{:?} is not tuple", self),
        }
    }

    #[inline]
    pub fn is_tuple(&self) -> bool {
        match self {
            Type::Tuple(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn function(self) -> FunctionType {
        match self {
            Type::Function(fk) => fk,
            _ => panic!("{:?} is not function", self),
        }
    }

    #[inline]
    pub fn is_function(&self) -> bool {
        match self {
            Type::Function(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_unknown(&self) -> bool {
        match self {
            Type::Unknown => true,
            _ => false,
        }
    }

    /// Returns the result type for eval
    #[inline]
    pub fn eval(self) -> Type {
        match self {
            Type::Appender(AppenderType { item_ty })
            | Type::VecMerger(VecMergerType { item_ty, .. }) => {
                Type::Vector(VectorType { item_ty })
            }
            Type::Merger(MergerType { item_ty, .. }) => *item_ty,
            Type::DictMerger(DictMergerType {
                key_ty, value_ty, ..
            }) => Type::Dict(DictType { key_ty, value_ty }),
            Type::GroupMerger(GroupMergerType { key_ty, value_ty }) => {
                let list_ty = Type::Vector(VectorType { item_ty: value_ty });
                Type::Dict(DictType {
                    key_ty,
                    value_ty: Box::new(list_ty),
                })
            }
            _ => panic!("{:?} cannot be evaluated", self),
        }
    }

    /// Returns the item type for merge
    pub fn merge(self) -> Type {
        match self {
            Type::Appender(AppenderType { item_ty })
            | Type::VecMerger(VecMergerType { item_ty, .. })
            | Type::Merger(MergerType { item_ty, .. }) => *item_ty,
            Type::DictMerger(DictMergerType {
                key_ty, value_ty, ..
            })
            | Type::GroupMerger(GroupMergerType { key_ty, value_ty }) => {
                Type::Tuple(TupleType(vec![*key_ty, *value_ty]))
            }
            _ => panic!("{:?} cannot be merged", self),
        }
    }
}
