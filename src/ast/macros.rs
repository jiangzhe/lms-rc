macro_rules! impl_num_var {
    ($ty:ty, $rty:ty, $litf:ident, $exprf:ident, $checkf:ident) => {
        impl Var<$ty> {
            /// Create a new var with literal.
            pub fn $litf(value: $rty) -> Self {
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

macro_rules! try_eq_for_lit {
    ($f:ident, $op:tt) => {
        fn $f(this: &Literal, that: &Literal) -> Result<Literal> {
            let r = match (this, that) {
                (Literal::Bool(v0), Literal::Bool(v1)) => v0 $op v1,
                (Literal::U8(v0), Literal::U8(v1)) => v0 $op v1,
                (Literal::U32(v0), Literal::U32(v1)) => v0 $op v1,
                (Literal::I32(v0), Literal::I32(v1)) => v0 $op v1,
                (Literal::U64(v0), Literal::U64(v1)) => v0 $op v1,
                (Literal::I64(v0), Literal::I64(v1)) => v0 $op v1,
                (Literal::F32(v0), Literal::F32(v1)) => v0 $op v1,
                (Literal::F64(v0), Literal::F64(v1)) => v0 $op v1,
                (Literal::Str(v0), Literal::Str(v1)) => v0 $op v1,
                (s, o) => return Err(compile_err!("incompatible types [{} and {}] in eq/ne operation", s.ty(), o.ty())),
            };
            Ok(Literal::Bool(r))
        }
    }
}

macro_rules! try_maxmin_for_lit {
    ($f:ident, $op:tt) => {
        fn $f(this: &Literal, that: &Literal) -> Result<Literal> {
            let r = match (this, that) {
                (Literal::U8(v0), Literal::U8(v1)) => if v0 $op v1 { this.clone() } else { that.clone() },
                (Literal::U32(v0), Literal::U32(v1)) => if v0 $op v1 { this.clone() } else { that.clone() },
                (Literal::I32(v0), Literal::I32(v1)) => if v0 $op v1 { this.clone() } else { that.clone() },
                (Literal::U64(v0), Literal::U64(v1)) => if v0 $op v1 { this.clone() } else { that.clone() },
                (Literal::I64(v0), Literal::I64(v1)) => if v0 $op v1 { this.clone() } else { that.clone() },
                (Literal::F32(v0), Literal::F32(v1)) => {
                    let v0 = f32::from_bits(*v0);
                    let v1 = f32::from_bits(*v1);
                    if v0 $op v1 { this.clone() } else { that.clone() }
                }
                (Literal::F64(v0), Literal::F64(v1)) => {
                    let v0 = f64::from_bits(*v0);
                    let v1 = f64::from_bits(*v1);
                    if v0 $op v1 { this.clone() } else { that.clone() }
                }
                (Literal::Str(v0), Literal::Str(v1)) => if v0 $op v1 { this.clone() } else { that.clone() },
                (s, o) => return Err(compile_err!("incompatible types [{} and {}] in {} operation", s.ty(), o.ty(), stringify!($f))),
            };
            Ok(r)
        }
    }
}

macro_rules! try_logical_for_lit {
    ($f:ident, $op:tt) => {
        fn $f(this: &Literal, that: &Literal) -> Result<Literal> {
            let r = match (this, that) {
                (Literal::Bool(v0), Literal::Bool(v1)) => *v0 $op *v1,
                (s, o) => return Err(compile_err!("incompatible types [{} and {}] in {} operation", s.ty(), o.ty(), stringify!($f))),
            };
            Ok(Literal::Bool(r))
        }
    }
}

macro_rules! try_arith_for_num_lit {
    ($f:ident, $op:tt) => {
        fn $f(this: &Literal, that: &Literal) -> Result<Literal> {
            let r = match (this, that) {
                (Literal::U8(left), Literal::U8(right)) => Literal::U8(left $op right),
                (Literal::U32(left), Literal::U32(right)) => Literal::U32(left $op right),
                (Literal::I32(left), Literal::I32(right)) => Literal::I32(left $op right),
                (Literal::U64(left), Literal::U64(right)) => Literal::U64(left $op right),
                (Literal::I64(left), Literal::I64(right)) => Literal::I64(left $op right),
                (Literal::F32(left), Literal::F32(right)) => {
                    let left = f32::from_bits(*left);
                    let right = f32::from_bits(*right);
                    let r = left $op right;
                    Literal::F32(f32::to_bits(r))
                }
                (Literal::F64(left), Literal::F64(right)) => {
                    let left = f64::from_bits(*left);
                    let right = f64::from_bits(*right);
                    let r = left $op right;
                    Literal::F64(f64::to_bits(r))
                }
                (s, o) => return Err(compile_err!("incompatible types [{}, {}] for {} operation", s.ty(), o.ty(), stringify!($ty))),
            };
            Ok(r)
        }
    }
}

macro_rules! try_bitop_for_num_lit {
    ($f:ident, $op:tt) => {
        fn $f(this: &Literal, that: &Literal) -> Result<Literal> {
            let r = match (this, that) {
                (Literal::U8(v0), Literal::U8(v1)) => Literal::U8(v0 $op v1),
                (Literal::U32(v0), Literal::U32(v1)) => Literal::U32(v0 $op v1),
                (Literal::I32(v0), Literal::I32(v1)) => Literal::I32(v0 $op v1),
                (Literal::U64(v0), Literal::U64(v1)) => Literal::U64(v0 $op v1),
                (Literal::I64(v0), Literal::I64(v1)) => Literal::I64(v0 $op v1),
                (s, o) => return Err(compile_err!("incompatible types [{} and {}] in {} operation", s.ty(), o.ty(), stringify!($f))),
            };
            Ok(r)
        }
    }
}

macro_rules! impl_arith_for_var_num {
    ($opty:ty, $oplty:ty, $oprty:ty, $vty:ty, $builtinty:ty, $opf:ident, $op:tt, $asf:ident, $litf:ident, $exprf:ident, $binopf:path) => {
        impl $opty for $vty {
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

        impl $oplty for $builtinty {
            type Output = $vty;

            fn $opf(self, other: $vty) -> $vty {
                match other.expr {
                    Expr::Literal(v1) => {
                        let v = self $op v1.$asf().unwrap();
                        Var::$litf(v)
                    }
                    v1 => {
                        let v0 = Var::$litf(self);
                        Var::$exprf(Expr::BinOp($binopf(v0.expr, v1)))
                    }
                }
            }
        }

        impl $oprty for $vty {
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

macro_rules! derive_display {
    ($ty:ty) => {
        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }
    };
}

macro_rules! impl_scalar_type {
    ($ty:ty, $lit:literal) => {
        impl ScalarType for $ty {
            fn scalar_repr(&self) -> &'static str {
                $lit
            }
        }
    };
}
