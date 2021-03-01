//! Arithmetic operations defined on Exp<T>
use crate::exp::Exp;
use crate::exp::ExpOrDef;
use crate::unit::PlusUnit;
use crate::def::Def;
use crate::def_arith::{Plus, Times};
use std::any::Any;
use std::ops::{Add, Mul};
use std::fmt::Debug;

impl<T> Add for Exp<T>
where
    T: Add<Output = T> + PlusUnit<T> + Debug + PartialEq + Clone + Any,
{
    type Output = ExpOrDef<T>;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Exp::Const(x), Exp::Const(y)) => ExpOrDef::Exp(Exp::Const(x + y)),
            (x, Exp::Const(y)) if y == T::UNIT => {
                ExpOrDef::Exp(x)
            }
            (Exp::Const(x), y) if x == T::UNIT => {
                ExpOrDef::Exp(y)
            }
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
    T: Add<Output = T> + PlusUnit<T> + Debug + PartialEq + Clone + Any,
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
                // x.1.to_atomic(Def::Times(times))
                ExpOrDef::Def(Def::Times(times))
            }
            (x, Exp::Sym(y)) => {
                let times = Times(x, Exp::Sym(y.clone()));
                // y.1.to_atomic(Def::Times(times))
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

#[cfg(test)]
mod tests {
    use crate::ctx::Context;
    use super::*;
    #[test]
    fn test_const_plus_exp() {
        let mut ctx = Context::new();

        let c3 = ctx.eval({
            let c1 = Exp::Const(1);
            let c2 = Exp::Const(2);
            c1 + c2
        });
        assert_eq!(Exp::Const(3), c3);
    }

    #[test]
    fn test_sym_const_plus_exp() {
        let mut ctx = Context::new();

        let r = ctx.dry_run(|_, input: Exp<i32>| {
            input + 1
        });
        println!("{:?}", ctx.find_stmt_by_sym(&r.as_sym().cloned().unwrap()));
    }
}
