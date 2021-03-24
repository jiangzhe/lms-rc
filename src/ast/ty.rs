use super::*;
use crate::sym::Symbol;
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
    Bool(Bool),
    U8(U8),
    U32(U32),
    I32(I32),
    U64(U64),
    I64(I64),
    F32(F32),
    F64(F64),
    Str(Str),
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
            Type::Bool(_)
            | Type::U8(_)
            | Type::U32(_)
            | Type::I32(_)
            | Type::U64(_)
            | Type::I64(_)
            | Type::F32(_)
            | Type::F64(_)
            | Type::Str(_) => true,
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
            Type::Bool(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_u8(&self) -> bool {
        match self {
            Type::U8(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_u32(&self) -> bool {
        match self {
            Type::U32(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_i32(&self) -> bool {
        match self {
            Type::I32(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        match self {
            Type::U64(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_i64(&self) -> bool {
        match self {
            Type::I64(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_f32(&self) -> bool {
        match self {
            Type::F32(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        match self {
            Type::F64(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_str(&self) -> bool {
        match self {
            Type::Str(_) => true,
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

impl TypeInference for Type {
    fn ty(&self) -> Type {
        self.clone()
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Bool(_) => f.write_str("Bool"),
            Type::U8(_) => f.write_str("U8"),
            Type::U32(_) => f.write_str("U32"),
            Type::I32(_) => f.write_str("I32"),
            Type::U64(_) => f.write_str("U64"),
            Type::I64(_) => f.write_str("I64"),
            Type::F32(_) => f.write_str("F32"),
            Type::F64(_) => f.write_str("F64"),
            Type::Str(_) => f.write_str("Str"),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bool {}
#[allow(non_upper_case_globals)]
pub const Bool: Bool = Bool {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U8 {}
pub const U8: U8 = U8 {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U32 {}
pub const U32: U32 = U32 {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I32 {}
pub const I32: I32 = I32 {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U64 {}
pub const U64: U64 = U64 {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I64 {}
pub const I64: I64 = I64 {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct F32 {}
pub const F32: F32 = F32 {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct F64 {}
pub const F64: F64 = F64 {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Str {}
#[allow(non_upper_case_globals)]
pub const Str: Str = Str {};

impl_from_for_type!(U8, Type::U8);
impl_from_for_type!(U32, Type::U32);
impl_from_for_type!(I32, Type::I32);
impl_from_for_type!(U64, Type::U64);
impl_from_for_type!(I64, Type::I64);
impl_from_for_type!(F32, Type::F32);
impl_from_for_type!(F64, Type::F64);
impl_from_for_type!(Str, Type::Str);

/// Marker trait for builder
pub trait BuilderType {}

impl BuilderType for AppenderType {}
impl BuilderType for MergerType {}
impl BuilderType for DictMergerType {}
impl BuilderType for GroupMergerType {}
impl BuilderType for VecMergerType {}
