use super::{Bool, Expr, Str, Type, TypeInference, F32, F64, I32, I64, U32, U64, U8, BinOpType};
use crate::Result;
use std::cmp::Ordering;

/// Literal represents constant values already known at compile time
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Literal {
    Bool(bool),
    U8(u8),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(u32),
    F64(u64),
    Str(String),
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
            Literal::Str(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn apply_bin_op(&self, other: &Self, op_ty: BinOpType) -> Result<Self> {
        if self.ty() != other.ty() {
            return Err(compile_err!("incompatible types [{}, {}] for binary operation {}", self.ty(), other.ty(), op_ty))
        }
        match op_ty {
            BinOpType::Add => try_add(self, other),
            BinOpType::Subtract => try_sub(self, other),
            BinOpType::Multiply => try_mul(self, other),
            BinOpType::Divide => try_div(self, other),
            BinOpType::Modulo => try_rem(self, other),
            BinOpType::Equal => try_eq(self, other),
            BinOpType::NotEqual => try_ne(self, other),
            BinOpType::GreaterThan => {
                let r = try_cmp(self, other)?;
                Ok(Literal::Bool(r == Ordering::Greater))
            }
            BinOpType::GreaterThanOrEqual => {
                let r = try_cmp(self, other)?;
                Ok(Literal::Bool(r == Ordering::Greater || r == Ordering::Equal))
            }
            BinOpType::LessThan => {
                let r = try_cmp(self, other)?;
                Ok(Literal::Bool(r == Ordering::Less))
            }
            BinOpType::LessThanOrEqual => {
                let r = try_cmp(self, other)?;
                Ok(Literal::Bool(r == Ordering::Less || r == Ordering::Equal))
            }
            BinOpType::LogicalAnd => try_logical_and(self, other),
            BinOpType::LogicalOr => try_logical_or(self, other),
            BinOpType::BitwiseAnd => try_bit_and(self, other),
            BinOpType::BitwiseOr => try_bit_or(self, other),
            BinOpType::Xor => try_bit_xor(self, other),
            BinOpType::Max => try_max(self, other),
            BinOpType::Min => try_min(self, other),
        }
    }
}

impl_from_for_lit!(bool, Literal::Bool);
impl_from_for_lit!(u8, Literal::U8);
impl_from_for_lit!(i32, Literal::I32);
impl_from_for_lit!(i64, Literal::I64);
impl_from_for_lit!(u32, Literal::U32);
impl_from_for_lit!(u64, Literal::U64);
impl_from_for_lit!(String, Literal::Str);

impl_from_for_lit_expr!(bool, Literal::Bool);
impl_from_for_lit_expr!(u8, Literal::U8);
impl_from_for_lit_expr!(i32, Literal::I32);
impl_from_for_lit_expr!(i64, Literal::I64);
impl_from_for_lit_expr!(u32, Literal::U32);
impl_from_for_lit_expr!(u64, Literal::U64);
impl_from_for_lit_expr!(String, Literal::Str);

try_eq_for_lit!(try_eq, ==);
try_eq_for_lit!(try_ne, !=);

fn try_cmp(this: &Literal, that: &Literal) -> Result<Ordering> {
    let r = match (this, that) {
        (Literal::Bool(v0), Literal::Bool(v1)) => v0.cmp(v1),
        (Literal::U8(v0), Literal::U8(v1)) => v0.cmp(v1),
        (Literal::U32(v0), Literal::U32(v1)) => v0.cmp(v1),
        (Literal::I32(v0), Literal::I32(v1)) => v0.cmp(v1),
        (Literal::U64(v0), Literal::U64(v1)) => v0.cmp(v1),
        (Literal::I64(v0), Literal::I64(v1)) => v0.cmp(v1),
        (Literal::F32(v0), Literal::F32(v1)) => {
            let v0 = f32::from_bits(*v0);
            let v1 = f32::from_bits(*v1);
            v0.partial_cmp(&v1).ok_or_else(|| compile_err!("float values[{} and {}] fail cmp operation", v0, v1))?
        }
        (Literal::F64(v0), Literal::F64(v1)) => {
            let v0 = f64::from_bits(*v0);
            let v1 = f64::from_bits(*v1);
            v0.partial_cmp(&v1).ok_or_else(|| compile_err!("float values[{} and {}] fail cmp operation", v0, v1))?
        }
        (Literal::Str(v0), Literal::Str(v1)) => v0.cmp(v1),
        (s, o) => return Err(compile_err!("incompatible types [{} and {}] in cmp operation", s.ty(), o.ty())),
    };
    Ok(r)
}

try_maxmin_for_lit!(try_max, >);
try_maxmin_for_lit!(try_min, <);

try_logical_for_lit!(try_logical_and, &&);
try_logical_for_lit!(try_logical_or, ||);

try_arith_for_num_lit!(try_add, +);
try_arith_for_num_lit!(try_sub, -);
try_arith_for_num_lit!(try_mul, *);
try_arith_for_num_lit!(try_div, /);
try_arith_for_num_lit!(try_rem, %);

try_bitop_for_num_lit!(try_bit_and, &);
try_bitop_for_num_lit!(try_bit_or, |);
try_bitop_for_num_lit!(try_bit_xor, ^);

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
            Literal::Bool(_) => Type::Bool(Bool),
            Literal::U8(_) => Type::U8(U8),
            Literal::I32(_) => Type::I32(I32),
            Literal::I64(_) => Type::I64(I64),
            Literal::U32(_) => Type::U32(U32),
            Literal::U64(_) => Type::U64(U64),
            Literal::F32(_) => Type::F32(F32),
            Literal::F64(_) => Type::F64(F64),
            Literal::Str(_) => Type::Str(Str),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Bool(v) => v.fmt(f),
            Literal::U8(v) => v.fmt(f),
            Literal::I32(v) => v.fmt(f),
            Literal::I64(v) => v.fmt(f),
            Literal::U32(v) => v.fmt(f),
            Literal::U64(v) => v.fmt(f),
            Literal::F32(v) => v.fmt(f),
            Literal::F64(v) => v.fmt(f),
            Literal::Str(v) => v.fmt(f),
        }
    }
}
