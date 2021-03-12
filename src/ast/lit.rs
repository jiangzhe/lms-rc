use super::{ScalarKind, Type, TypeInference};

/// Literal represents constant values already known at compile time
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Literal {
    Bool(bool),
    U8(u8),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(u32),
    F64(u64),
    String(String),
}

impl Literal {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Literal::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Literal::U8(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Literal::I32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Literal::U32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Literal::I64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Literal::U64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Literal::F32(v) => Some(f32::from_bits(*v)),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Literal::F64(v) => Some(f64::from_bits(*v)),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Literal::String(v) => Some(v.clone()),
            _ => None,
        }
    }
}

impl_from_for_lit!(bool, Literal::Bool);
impl_from_for_lit!(u8, Literal::U8);
impl_from_for_lit!(i32, Literal::I32);
impl_from_for_lit!(i64, Literal::I64);
impl_from_for_lit!(u32, Literal::U32);
impl_from_for_lit!(u64, Literal::U64);
impl_from_for_lit!(String, Literal::String);

/// special conversion between f32 and Literal
impl From<f32> for Literal {
    fn from(src: f32) -> Self {
        Literal::F32(f32::to_bits(src))
    }
}

/// special conversion between f64 and Literal
impl From<f64> for Literal {
    fn from(src: f64) -> Self {
        Literal::F64(f64::to_bits(src))
    }
}

impl TypeInference for Literal {
    fn ty(&self) -> Type {
        match self {
            Literal::Bool(_) => Type::Scalar(ScalarKind::Bool),
            Literal::U8(_) => Type::Scalar(ScalarKind::U8),
            Literal::I32(_) => Type::Scalar(ScalarKind::I32),
            Literal::I64(_) => Type::Scalar(ScalarKind::I64),
            Literal::U32(_) => Type::Scalar(ScalarKind::U32),
            Literal::U64(_) => Type::Scalar(ScalarKind::U64),
            Literal::F32(_) => Type::Scalar(ScalarKind::F32),
            Literal::F64(_) => Type::Scalar(ScalarKind::F64),
            Literal::String(_) => Type::Scalar(ScalarKind::String),
        }
    }
}
