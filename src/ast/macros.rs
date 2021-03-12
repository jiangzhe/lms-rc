macro_rules! impl_arith_for_var_lit {
    ($ty:ty, $func:ident, $op:tt) => {
        impl $ty for Var<Literal> {
            type Output = Self;

            fn $func(self, other: Self) -> Self::Output {
                match (self.0, other.0) {
                    // floating number
                    (Literal::F32(f0), Literal::F32(f1)) => {
                        let r = f32::from_bits(f0) $op f32::from_bits(f1);
                        Var(Literal::F32(f32::to_bits(r)))
                    }
                    (Literal::F32(f0), Literal::F64(f1)) => {
                        let r = f32::from_bits(f0) as f64 $op f64::from_bits(f1);
                        Var(Literal::F64(f64::to_bits(r)))
                    }
                    (Literal::F64(f0), Literal::F32(f1)) => {
                        let r = f64::from_bits(f0) $op f32::from_bits(f1) as f64;
                        Var(Literal::F64(f64::to_bits(r)))
                    },
                    (Literal::F64(f0), Literal::F64(f1)) => {
                        let r = f64::from_bits(f0) $op f64::from_bits(f1);
                        Var(Literal::F64(f64::to_bits(r)))
                    },
                    // signed integers
                    (Literal::I32(i0), Literal::I32(i1)) => {
                        let r = i0 $op i1;
                        Var(Literal::I32(r))
                    }
                    (Literal::I32(i0), Literal::I64(i1)) => {
                        let r = i0 as i64 $op i1;
                        Var(Literal::I64(r))
                    }
                    (Literal::I64(i0), Literal::I32(i1)) => {
                        let r = i0 $op i1 as i64;
                        Var(Literal::I64(r))
                    }
                    (Literal::I64(i0), Literal::I64(i1)) => {
                        let r = i0 $op i1;
                        Var(Literal::I64(r))
                    }
                    // unsigned integers
                    (Literal::U32(u0), Literal::U32(u1)) => {
                        let r = u0 $op u1;
                        Var(Literal::U32(r))
                    }
                    (Literal::U32(u0), Literal::U64(u1)) => {
                        let r = u0 as u64 $op u1;
                        Var(Literal::U64(r))
                    }
                    (Literal::U64(u0), Literal::U32(u1)) => {
                        let r = u0 $op u1 as u64;
                        Var(Literal::U64(r))
                    }
                    (Literal::U64(u0), Literal::U64(u1)) => {
                        let r = u0 $op u1;
                        Var(Literal::U64(r))
                    }
                    (l0, l1) => panic!("incompatible types[{:?} & {:?}] in {} operation", l0, l1, stringify!($ty)),
                }
            }
        }
    }
}

macro_rules! impl_arith_for_var_builtin {
    ($ty:ty, $func:ident, $rty:ty, $p1:path, $op:tt) => {
        impl $ty for Var<Literal> {
            type Output = Self;

            fn $func(self, other: $rty) -> Self::Output {
                match self.0 {
                    $p1(f0) => {
                        let r = f0 $op other;
                        Var($p1(r))
                    }
                    _ => panic!("incompatible types[{:?} & {:?}] in {} operation", self, other, stringify!($ty)),
                }
            }
        }
    }
}

macro_rules! impl_arith_for_var_float {
    ($ty1:ty, $ty2:ty, $func:ident, $op:tt) => {
        impl $ty1 for Var<Literal> {
            type Output = Self;

            fn $func(self, other: f32) -> Self::Output {
                match self.0 {
                    Literal::F32(f0) => {
                        let r = f32::from_bits(f0) $op other;
                        Var(Literal::F32(f32::to_bits(r)))
                    }
                    Literal::F64(f0) => {
                        let r = f64::from_bits(f0) $op other as f64;
                        Var(Literal::F64(f64::to_bits(r)))
                    }
                    _ => panic!("incompatible types[{:?} & {:?}] in {} operation", self, other, stringify!($ty1)),
                }
            }
        }

        impl $ty2 for Var<Literal> {
            type Output = Self;

            fn $func(self, other: f64) -> Self::Output {
                match self.0 {
                    Literal::F32(f0) => {
                        let r = f32::from_bits(f0) as f64 $op other;
                        Var(Literal::F64(f64::to_bits(r)))
                    }
                    Literal::F64(f0) => {
                        let r = f64::from_bits(f0) $op other;
                        Var(Literal::F64(f64::to_bits(r)))
                    }
                    _ => panic!("incompatible types[{:?} & {:?}] in {} operation", self, other, stringify!($ty2)),
                }
            }
        }
    }
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
