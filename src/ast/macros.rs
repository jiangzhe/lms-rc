macro_rules! impl_arith_for_var_scalar {
    ($ty:ty, $func:ident, $op:tt) => {

        impl $ty for Var<ScalarKind> {
            type Output = Self;

            fn $func(self, other: Self) -> Self::Output {
                match (self.0, other.0) {
                    // unsigned integer
                    (ScalarKind::U8, ScalarKind::U8) => {
                        match (self.1, other.1) {
                            (Expr::Literal(v0), Expr::Literal(v1)) => {
                                let v = v0.as_u8().unwrap() $op v1.as_u8().unwrap();
                                Var::lit(v)
                            }
                            (v0, v1) => {
                                Var::scalar(ScalarKind::U8, Expr::BinOp(BinOp::add(v0, v1)))
                            }
                        }
                    }
                    (ScalarKind::U32, ScalarKind::U32) => {
                        match (self.1, other.1) {
                            (Expr::Literal(v0), Expr::Literal(v1)) => {
                                let v = v0.as_u32().unwrap() $op v1.as_u32().unwrap();
                                Var::lit(v)
                            }
                            (v0, v1) => {
                                Var::scalar(ScalarKind::U32, Expr::BinOp(BinOp::add(v0, v1)))
                            }
                        }
                    }
                    (ScalarKind::U64, ScalarKind::U64) => {
                        match (self.1, other.1) {
                            (Expr::Literal(v0), Expr::Literal(v1)) => {
                                let v = v0.as_u64().unwrap() $op v1.as_u64().unwrap();
                                Var::lit(v)
                            }
                            (v0, v1) => {
                                Var::scalar(ScalarKind::U64, Expr::BinOp(BinOp::add(v0, v1)))
                            }
                        }
                    }
                    // signed integer
                    (ScalarKind::I32, ScalarKind::I32) => {
                        match (self.1, other.1) {
                            (Expr::Literal(v0), Expr::Literal(v1)) => {
                                let v = v0.as_i32().unwrap() $op v1.as_i32().unwrap();
                                Var::lit(v)
                            }
                            (v0, v1) => {
                                Var::scalar(ScalarKind::I32, Expr::BinOp(BinOp::add(v0, v1)))
                            }
                        }
                    }
                    (ScalarKind::I64, ScalarKind::I64) => {
                        match (self.1, other.1) {
                            (Expr::Literal(v0), Expr::Literal(v1)) => {
                                let v = v0.as_i64().unwrap() $op v1.as_i64().unwrap();
                                Var::lit(v)
                            }
                            (v0, v1) => {
                                Var::scalar(ScalarKind::I64, Expr::BinOp(BinOp::add(v0, v1)))
                            }
                        }
                    }
                    // floating number
                    (ScalarKind::F32, ScalarKind::F32) => {
                        match (self.1, other.1) {
                            (Expr::Literal(v0), Expr::Literal(v1)) => {
                                let v = v0.as_f32().unwrap() $op v1.as_f32().unwrap();
                                Var::lit(v)
                            }
                            (v0, v1) => {
                                Var::scalar(ScalarKind::I32, Expr::BinOp(BinOp::add(v0, v1)))
                            }
                        }
                    }
                    (ScalarKind::F64, ScalarKind::F64) => {
                        match (self.1, other.1) {
                            (Expr::Literal(v0), Expr::Literal(v1)) => {
                                let v = v0.as_f64().unwrap() $op v1.as_f64().unwrap();
                                Var::lit(v)
                            }
                            (v0, v1) => {
                                Var::scalar(ScalarKind::I64, Expr::BinOp(BinOp::add(v0, v1)))
                            }
                        }
                    }
                    (k0, k1) => panic!("incompatible types[{:?} & {:?}] in {} operation", k0, k1, stringify!($ty)),
                }
            }
        }
    }
}

macro_rules! impl_arith_for_var_builtin {
    ($ty:ty, $func:ident, $rty:ty, $asfunc:ident, $p1:path, $op:tt) => {
        impl $ty for Var<ScalarKind> {
            type Output = Self;

            fn $func(self, other: $rty) -> Self::Output {
                match self.0 {
                    $p1 => match self.1 {
                        Expr::Literal(f0) => {
                            let r = f0.$asfunc().unwrap() + other;
                            return Var::lit(r);
                        }
                        _ => (),
                    },
                    _ => (),
                }
                panic!(
                    "incompatible types[{:?} & {:?}] in {} operation",
                    self,
                    other,
                    stringify!($ty)
                )
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
