macro_rules! impl_num_var {
    ($ty:ty, $litf:ident, $exprf:ident, $checkf:ident) => {
        impl Var<$ty> {
            /// Create a new var with literal.
            pub fn $litf(value: $ty) -> Self {
                Var::new(Expr::Literal(value.into()))
            }

            /// Create a new var with an expression.
            #[inline]
            pub fn $exprf(expr: Expr) -> Self {
                assert!(
                    expr.ty().$checkf(),
                    "Imcompatible type {:?} to construct a {:?} var",
                    expr.ty(),
                    stringify!($ty)
                );
                Var::new(expr)
            }

            /// Equality check on two vars and returns a bool var
            pub fn eq(self, other: Self) -> Var<bool> {
                Var::new(Expr::BinOp(BinOp::eq(self.expr, other.expr)))
            }

            /// Non-equality check on two vars, and returns a bool var
            pub fn ne(self, other: Self) -> Var<bool> {
                Var::new(Expr::BinOp(BinOp::ne(self.expr, other.expr)))
            }
        }
    };
}

macro_rules! impl_from_for_lit {
    ($ty:ty, $path:path) => {
        impl From<$ty> for Literal {
            fn from(src: $ty) -> Self {
                $path(src)
            }
        }
    };
}

macro_rules! impl_from_for_lit_expr {
    ($ty:ty, $path:path) => {
        impl From<$ty> for Expr {
            fn from(src: $ty) -> Self {
                Expr::Literal($path(src))
            }
        }
    };
}

macro_rules! impl_from_for_type {
    ($ty:ty, $path:path) => {
        impl From<$ty> for Type {
            fn from(src: $ty) -> Self {
                $path(src)
            }
        }
    };
}

macro_rules! impl_arith_for_var_num {
    ($opty:ty, $opgty:ty, $builtinty:ty, $opf:ident, $op:tt, $asf:ident, $litf:ident, $exprf:ident, $binopf:path) => {
        impl $opty for Var<$builtinty> {
            type Output = Self;

            fn $opf(self, other: Self) -> Self {
                match (self.expr, other.expr) {
                    (Expr::Literal(v0), Expr::Literal(v1)) => {
                        let v = v0.$asf().unwrap() $op v1.$asf().unwrap();
                        Var::$litf(v)
                    }
                    (v0, v1) => {
                        Var::$exprf(Expr::BinOp($binopf(v0, v1)))
                    }
                }
            }
        }

        impl $opgty for Var<$builtinty> {
            type Output = Self;

            fn $opf(self, other: $builtinty) -> Self {
                match self.expr {
                    Expr::Literal(v0) => {
                        let v = v0.$asf().unwrap() $op other;
                        Var::$litf(v)
                    }
                    v0 => {
                        let v1 = other.into();
                        Var::$exprf(Expr::BinOp($binopf(v0, Expr::Literal(v1))))
                    }
                }
            }
        }
    }
}

macro_rules! impl_static_type {
    ($ty:ty, $path:path) => {
        impl StaticType for $ty {
            fn ty() -> Type {
                $path
            }
        }
    };
}

macro_rules! impl_dynamic_type {
    ($ty:ty, $path:path) => {
        impl DynamicType for $ty {
            fn ty(self) -> Type {
                $path(self)
            }
        }
    };
}

macro_rules! derive_display {
    ($ty:ty) => {
        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }
    };
}
