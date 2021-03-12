use crate::ast::lit::Literal;
use std::ops::{Add, Div, Mul, Sub};

/// Var represents a variable of linear type in the staging program
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var<T>(T);

impl Var<Literal> {
    pub fn lit<U>(value: U) -> Self
    where
        U: Into<Literal>,
    {
        Var(value.into())
    }
}

impl_arith_for_var_lit!(Add, add, +);
impl_arith_for_var_lit!(Sub, sub, -);
impl_arith_for_var_lit!(Mul, mul, *);
impl_arith_for_var_lit!(Div, div, /);
impl_arith_for_var_builtin!(Add<i32>, add, i32, Literal::I32, +);
impl_arith_for_var_builtin!(Add<u32>, add, u32, Literal::U32, +);
impl_arith_for_var_builtin!(Add<i64>, add, i64, Literal::I64, +);
impl_arith_for_var_builtin!(Add<u64>, add, u64, Literal::U64, +);
impl_arith_for_var_float!(Add<f32>, Add<f64>, add, +);
impl_arith_for_var_builtin!(Sub<i32>, sub, i32, Literal::I32, -);
impl_arith_for_var_builtin!(Sub<u32>, sub, u32, Literal::U32, -);
impl_arith_for_var_builtin!(Sub<i64>, sub, i64, Literal::I64, -);
impl_arith_for_var_builtin!(Sub<u64>, sub, u64, Literal::U64, -);
impl_arith_for_var_float!(Sub<f32>, Sub<f64>, sub, -);
impl_arith_for_var_builtin!(Mul<i32>, mul, i32, Literal::I32, *);
impl_arith_for_var_builtin!(Mul<u32>, mul, u32, Literal::U32, *);
impl_arith_for_var_builtin!(Mul<i64>, mul, i64, Literal::I64, *);
impl_arith_for_var_builtin!(Mul<u64>, mul, u64, Literal::U64, *);
impl_arith_for_var_float!(Mul<f32>, Mul<f64>, mul, *);
impl_arith_for_var_builtin!(Div<i32>, div, i32, Literal::I32, /);
impl_arith_for_var_builtin!(Div<u32>, div, u32, Literal::U32, /);
impl_arith_for_var_builtin!(Div<i64>, div, i64, Literal::I64, /);
impl_arith_for_var_builtin!(Div<u64>, div, u64, Literal::U64, /);
impl_arith_for_var_float!(Div<f32>, Div<f64>, div, /);

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
