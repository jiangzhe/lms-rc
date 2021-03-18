use super::*;
use enum_dispatch::enum_dispatch;

/// Enable type inference on any expression.
///
/// The type inference is executed on the program staging,
/// before the code generation and compilation.
#[enum_dispatch]
pub trait TypeInference {
    fn ty(&self) -> Type;
}

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
    /// A lambda with a list of arguments and return type.
    Lambda(LambdaType),
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
    pub fn is_builder(&self) -> bool {
        match self {
            Type::Appender(_)
            | Type::Merger(_)
            | Type::DictMerger(_)
            | Type::GroupMerger(_)
            | Type::VecMerger(_) => true,
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
    pub fn lambda(self) -> LambdaType {
        match self {
            Type::Lambda(lk) => lk,
            _ => panic!("{:?} is not lambda", self),
        }
    }

    #[inline]
    pub fn is_lambda(&self) -> bool {
        match self {
            Type::Lambda(_) => true,
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
    ///
    /// This method only returns valid type that can be merged
    /// if self is builder type, otherwise it will panic.
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Bool => f.write_str("Bool"),
            Type::U8 => f.write_str("U8"),
            Type::U32 => f.write_str("U32"),
            Type::I32 => f.write_str("I32"),
            Type::U64 => f.write_str("U64"),
            Type::I64 => f.write_str("I64"),
            Type::F32 => f.write_str("F32"),
            Type::F64 => f.write_str("F64"),
            Type::String => f.write_str("String"),
            Type::Vector(v) => v.fmt(f),
            Type::Dict(d) => d.fmt(f),
            Type::Tuple(t) => t.fmt(f),
            Type::Lambda(lmd) => lmd.fmt(f),
            Type::Appender(a) => a.fmt(f),
            Type::Merger(m) => m.fmt(f),
            Type::DictMerger(dm) => dm.fmt(f),
            Type::GroupMerger(gm) => gm.fmt(f),
            Type::VecMerger(vm) => vm.fmt(f),
            Type::Unknown => f.write_str("Unknown"),
        }
    }
}

/// Base trait for static type information
pub trait StaticType {
    fn ty() -> Type;
}

impl_static_type!(u8, Type::U8);
impl_static_type!(u32, Type::U32);
impl_static_type!(i32, Type::I32);
impl_static_type!(u64, Type::U64);
impl_static_type!(i64, Type::I64);
impl_static_type!(f32, Type::F32);
impl_static_type!(f64, Type::F64);
impl_static_type!(String, Type::String);

/// Base trait for dynamic type information
pub trait DynamicType {
    fn ty(self) -> Type;
}

impl_dynamic_type!(VectorType, Type::Vector);
impl_dynamic_type!(DictType, Type::Dict);
impl_dynamic_type!(TupleType, Type::Tuple);
impl_dynamic_type!(LambdaType, Type::Lambda);
impl_dynamic_type!(AppenderType, Type::Appender);
impl_dynamic_type!(MergerType, Type::Merger);
impl_dynamic_type!(DictMergerType, Type::DictMerger);
impl_dynamic_type!(GroupMergerType, Type::GroupMerger);
impl_dynamic_type!(VecMergerType, Type::VecMerger);
