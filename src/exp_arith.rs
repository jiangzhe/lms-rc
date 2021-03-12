//! Arithmetic operations defined on Exp<T>
use crate::def::Def;
use crate::def_arith::{Divides, Minus, Plus, Times};
use crate::exp::Exp;
use crate::exp::ExpOrDef;
use std::any::Any;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

impl<T> Add for Exp<T>
where
    T: Add<Output = T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Exp::Const(x), Exp::Const(y)) => ExpOrDef::Exp(Exp::Const(x + y)),
            (Exp::Sym(x), y) => {
                let plus = Plus(Exp::Sym(x.clone()), y);
                ExpOrDef::Def(Def::Plus(plus))
            }
            (x, Exp::Sym(y)) => {
                let plus = Plus(x, Exp::Sym(y.clone()));
                ExpOrDef::Def(Def::Plus(plus))
            }
        }
    }
}

impl<T> Add<T> for Exp<T>
where
    T: Add<Output = T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn add(self, other: T) -> Self::Output {
        match self {
            Exp::Const(x) => ExpOrDef::Exp(Exp::Const(x + other)),
            Exp::Sym(x) => {
                let plus = Plus(Exp::Sym(x.clone()), Exp::Const(other));
                ExpOrDef::Def(Def::Plus(plus))
            }
        }
    }
}

impl_num_add!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

impl<T> Sub for Exp<T>
where
    T: Sub<Output = T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Exp::Const(x), Exp::Const(y)) => ExpOrDef::Exp(Exp::Const(x - y)),
            (Exp::Sym(x), y) => {
                let minus = Minus(Exp::Sym(x.clone()), y);
                ExpOrDef::Def(Def::Minus(minus))
            }
            (x, Exp::Sym(y)) => {
                let minus = Minus(x, Exp::Sym(y.clone()));
                ExpOrDef::Def(Def::Minus(minus))
            }
        }
    }
}

impl<T> Sub<T> for Exp<T>
where
    T: Sub<Output = T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn sub(self, other: T) -> Self::Output {
        match self {
            Exp::Const(x) => ExpOrDef::Exp(Exp::Const(x - other)),
            Exp::Sym(x) => {
                let minus = Minus(Exp::Sym(x.clone()), Exp::Const(other));
                ExpOrDef::Def(Def::Minus(minus))
            }
        }
    }
}

impl_num_sub!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

impl<T> Mul for Exp<T>
where
    T: Mul<Output = T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Exp::Const(x), Exp::Const(y)) => ExpOrDef::Exp(Exp::Const(x * y)),
            (Exp::Sym(x), y) => {
                let times = Times(Exp::Sym(x.clone()), y);
                ExpOrDef::Def(Def::Times(times))
            }
            (x, Exp::Sym(y)) => {
                let times = Times(x, Exp::Sym(y.clone()));
                ExpOrDef::Def(Def::Times(times))
            }
        }
    }
}

impl<T> Mul<T> for Exp<T>
where
    T: Mul<Output = T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn mul(self, other: T) -> Self::Output {
        match self {
            Exp::Const(x) => ExpOrDef::Exp(Exp::Const(x * other)),
            Exp::Sym(s) => {
                let times = Times(Exp::Sym(s.clone()), Exp::Const(other));
                ExpOrDef::Def(Def::Times(times))
            }
        }
    }
}

impl_num_mul!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

impl<T> Div for Exp<T>
where
    T: Div<Output = T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Exp::Const(x), Exp::Const(y)) => ExpOrDef::Exp(Exp::Const(x / y)),
            (Exp::Sym(x), y) => {
                let divides = Divides(Exp::Sym(x.clone()), y);
                ExpOrDef::Def(Def::Divides(divides))
            }
            (x, Exp::Sym(y)) => {
                let divides = Divides(x, Exp::Sym(y.clone()));
                ExpOrDef::Def(Def::Divides(divides))
            }
        }
    }
}

impl<T> Div<T> for Exp<T>
where
    T: Div<Output = T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn div(self, other: T) -> Self::Output {
        match self {
            Exp::Const(x) => ExpOrDef::Exp(Exp::Const(x / other)),
            Exp::Sym(s) => {
                let divides = Divides(Exp::Sym(s), Exp::Const(other));
                ExpOrDef::Def(Def::Divides(divides))
            }
        }
    }
}

impl_num_div!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctx::Context;
    #[test]
    fn test_const_arith() {
        let mut ctx = Context::new();

        let v1 = ctx.eval(Exp::Const(1) + Exp::Const(2));
        assert_eq!(Exp::Const(3), v1);
        let v2 = ctx.eval(Exp::Const(2) - Exp::Const(1));
        assert_eq!(Exp::Const(1), v2);
        let v3 = ctx.eval(Exp::Const(2) * Exp::Const(2));
        assert_eq!(Exp::Const(4), v3);
        let v4 = ctx.eval(Exp::Const(6) / Exp::Const(2));
        assert_eq!(Exp::Const(3), v4);

        let v1 = ctx.eval(Exp::Const(1) + 2);
        assert_eq!(Exp::Const(3), v1);
        let v2 = ctx.eval(Exp::Const(2) - 1);
        assert_eq!(Exp::Const(1), v2);
        let v3 = ctx.eval(Exp::Const(2) * 2);
        assert_eq!(Exp::Const(4), v3);
        let v4 = ctx.eval(Exp::Const(6) / 2);
        assert_eq!(Exp::Const(3), v4);

        let v1 = ctx.eval(1 + Exp::Const(2));
        assert_eq!(Exp::Const(3), v1);
        let v2 = ctx.eval(2 - Exp::Const(1));
        assert_eq!(Exp::Const(1), v2);
        let v3 = ctx.eval(2 * Exp::Const(2));
        assert_eq!(Exp::Const(4), v3);
        let v4 = ctx.eval(6 / Exp::Const(2));
        assert_eq!(Exp::Const(3), v4);
    }
}
