use crate::ast::scalar::ScalarKind;
use crate::ast::ty::{Type, TypeInference};

// restrict numeric types
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Literal {
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(u32),
    F64(u64),
    String(String),
}

impl_from_for_lit!(bool, Literal::Bool);
impl_from_for_lit!(i32, Literal::I32);
impl_from_for_lit!(i64, Literal::I64);
impl_from_for_lit!(u32, Literal::U32);
impl_from_for_lit!(u64, Literal::U64);
impl_from_for_lit!(String, Literal::String);

impl From<f32> for Literal {
    fn from(src: f32) -> Self {
        Literal::F32(f32::to_bits(src))
    }
}

impl From<f64> for Literal {
    fn from(src: f64) -> Self {
        Literal::F64(f64::to_bits(src))
    }
}

impl TypeInference for Literal {
    fn ty(&self) -> Type {
        match self {
            Literal::Bool(_) => Type::Scalar(ScalarKind::Bool),
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
