use super::{Bool, F32, F64, I32, I64, U32, U64, U8};

pub trait ScalarType {
    fn scalar_repr(&self) -> &'static str;
}

impl_scalar_type!(Bool, "i1");
impl_scalar_type!(U8, "i8");
impl_scalar_type!(U32, "i32");
impl_scalar_type!(I32, "i32");
impl_scalar_type!(U64, "i64");
impl_scalar_type!(I64, "i64");
impl_scalar_type!(F32, "f32");
impl_scalar_type!(F64, "f64");
