#[doc(hidden)]
macro_rules! impl_num_add {
    ($ty:ty) => {
        impl Add<$crate::exp::Exp<$ty>> for $ty {
            type Output = $crate::exp::ExpOrDef<$ty>;

            fn add(self, other: $crate::exp::Exp<$ty>) -> Self::Output {
                match other {
                    $crate::exp::Exp::Const(y) => $crate::exp::ExpOrDef::Exp($crate::exp::Exp::Const(self + y)),
                    $crate::exp::Exp::Sym(s) => {
                        let plus = $crate::def_arith::Plus($crate::exp::Exp::Const(self), $crate::exp::Exp::Sym(s));
                        $crate::exp::ExpOrDef::Def($crate::def::Def::Plus(plus))
                    }
                }
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_num_add!($ty);
        )*
    }
}

#[doc(hidden)]
macro_rules! impl_num_sub {
    ($ty:ty) => {
        impl Sub<$crate::exp::Exp<$ty>> for $ty {
            type Output = $crate::exp::ExpOrDef<$ty>;

            fn sub(self, other: $crate::exp::Exp<$ty>) -> Self::Output {
                match other {
                    $crate::exp::Exp::Const(y) => $crate::exp::ExpOrDef::Exp($crate::exp::Exp::Const(self - y)),
                    $crate::exp::Exp::Sym(s) => {
                        let minus = $crate::def_arith::Minus($crate::exp::Exp::Const(self), $crate::exp::Exp::Sym(s));
                        $crate::exp::ExpOrDef::Def($crate::def::Def::Minus(minus))
                    }
                }
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_num_sub!($ty);
        )*
    }
}

#[doc(hidden)]
macro_rules! impl_num_mul {
    ($ty:ty) => {
        impl Mul<$crate::exp::Exp<$ty>> for $ty {
            type Output = $crate::exp::ExpOrDef<$ty>;

            fn mul(self, other: $crate::exp::Exp<$ty>) -> Self::Output {
                match other {
                    $crate::exp::Exp::Const(y) => $crate::exp::ExpOrDef::Exp($crate::exp::Exp::Const(self * y)),
                    $crate::exp::Exp::Sym(s) => {
                        let times = $crate::def_arith::Times($crate::exp::Exp::Const(self), $crate::exp::Exp::Sym(s));
                        $crate::exp::ExpOrDef::Def($crate::def::Def::Times(times))
                    }
                }
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_num_mul!($ty);
        )*
    }
}

#[doc(hidden)]
macro_rules! impl_num_div {
    ($ty:ty) => {
        impl Div<$crate::exp::Exp<$ty>> for $ty {
            type Output = $crate::exp::ExpOrDef<$ty>;

            fn div(self, other: $crate::exp::Exp<$ty>) -> Self::Output {
                match other {
                    $crate::exp::Exp::Const(y) => $crate::exp::ExpOrDef::Exp($crate::exp::Exp::Const(self / y)),
                    $crate::exp::Exp::Sym(s) => {
                        let divides = $crate::def_arith::Divides($crate::exp::Exp::Const(self), $crate::exp::Exp::Sym(s));
                        $crate::exp::ExpOrDef::Def($crate::def::Def::Divides(divides))
                    }
                }
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_num_div!($ty);
        )*
    }
}
