use crate::ast::lit::LiteralKind;
use std::ops::{Add, Sub, Mul, Div};

/// Var represents a variable of linear type in the staging program
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var<T>(T);

impl Var<LiteralKind> {
    
    pub fn lit<U>(value: U) -> Self 
    where
        U: Into<LiteralKind>,
    {
        Var(value.into())
    }
}

impl_arith_for_var_lit!(Add, add, +);
impl_arith_for_var_lit!(Sub, sub, -);
impl_arith_for_var_lit!(Mul, mul, -);
impl_arith_for_var_lit!(Div, div, -);
impl_arith_for_var_small!(Add<i32>, add, i32, i64, LiteralKind::I32Literal, LiteralKind::I64Literal, +);
impl_arith_for_var_small!(Add<u32>, add, u32, u64, LiteralKind::U32Literal, LiteralKind::U64Literal, +);
impl_arith_for_var_big!(Add<i64>, add, i64, i32, LiteralKind::I32Literal, LiteralKind::I64Literal, +);
impl_arith_for_var_big!(Add<u64>, add, u64, u32, LiteralKind::U32Literal, LiteralKind::U64Literal, +);
impl_arith_for_var_float_small!(Add<f32>, add, +);
impl_arith_for_var_float_big!(Add<f64>, add, +);
impl_arith_for_var_small!(Sub<i32>, sub, i32, i64, LiteralKind::I32Literal, LiteralKind::I64Literal, -);
impl_arith_for_var_small!(Sub<u32>, sub, u32, u64, LiteralKind::U32Literal, LiteralKind::U64Literal, -);
impl_arith_for_var_big!(Sub<i64>, sub, i64, i32, LiteralKind::I32Literal, LiteralKind::I64Literal, -);
impl_arith_for_var_big!(Sub<u64>, sub, u64, u32, LiteralKind::U32Literal, LiteralKind::U64Literal, -);
impl_arith_for_var_float_small!(Sub<f32>, sub, -);
impl_arith_for_var_float_big!(Sub<f64>, sub, -);
impl_arith_for_var_small!(Mul<i32>, mul, i32, i64, LiteralKind::I32Literal, LiteralKind::I64Literal, *);
impl_arith_for_var_small!(Mul<u32>, mul, u32, u64, LiteralKind::U32Literal, LiteralKind::U64Literal, *);
impl_arith_for_var_big!(Mul<i64>, mul, i64, i32, LiteralKind::I32Literal, LiteralKind::I64Literal, *);
impl_arith_for_var_big!(Mul<u64>, mul, u64, u32, LiteralKind::U32Literal, LiteralKind::U64Literal, *);
impl_arith_for_var_float_small!(Mul<f32>, mul, *);
impl_arith_for_var_float_big!(Mul<f64>, mul, *);
impl_arith_for_var_small!(Div<i32>, div, i32, i64, LiteralKind::I32Literal, LiteralKind::I64Literal, /);
impl_arith_for_var_small!(Div<u32>, div, u32, u64, LiteralKind::U32Literal, LiteralKind::U64Literal, /);
impl_arith_for_var_big!(Div<i64>, div, i64, i32, LiteralKind::I32Literal, LiteralKind::I64Literal, /);
impl_arith_for_var_big!(Div<u64>, div, u64, u32, LiteralKind::U32Literal, LiteralKind::U64Literal, /);
impl_arith_for_var_float_small!(Div<f32>, div, /);
impl_arith_for_var_float_big!(Div<f64>, div, /);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_var_lit_add() {
        let v1 = Var::lit(1);
        let v2 = Var::lit(2);
        let v3 = v1 + v2;
        assert_eq!(Var::lit(3), v3);
        let v4 = Var::lit(4);
        assert_eq!(Var::lit(8), v4 + 4);
    }

    #[test]
    #[should_panic]
    fn test_var_lit_incompat() {
        let v1 = Var::lit(true);
        let v2 = Var::lit(2);
        let _ = v1 + v2;
    }
}