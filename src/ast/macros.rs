
macro_rules! impl_arith_for_var_lit {
    ($ty:ty, $func:ident, $op:tt) => {
        impl $ty for Var<LiteralKind> {
            type Output = Self;
        
            fn $func(self, other: Self) -> Self::Output {
                match (self.0, other.0) {
                    // floating number
                    (LiteralKind::F32Literal(f0), LiteralKind::F32Literal(f1)) => {
                        let r = f32::from_bits(f0) $op f32::from_bits(f1);
                        Var(LiteralKind::F32Literal(f32::to_bits(r)))
                    }
                    (LiteralKind::F32Literal(f0), LiteralKind::F64Literal(f1)) => {
                        let r = f32::from_bits(f0) as f64 $op f64::from_bits(f1);
                        Var(LiteralKind::F64Literal(f64::to_bits(r)))
                    }
                    (LiteralKind::F64Literal(f0), LiteralKind::F32Literal(f1)) => {
                        let r = f64::from_bits(f0) $op f32::from_bits(f1) as f64;
                        Var(LiteralKind::F64Literal(f64::to_bits(r)))
                    },
                    (LiteralKind::F64Literal(f0), LiteralKind::F64Literal(f1)) => {
                        let r = f64::from_bits(f0) $op f64::from_bits(f1);
                        Var(LiteralKind::F64Literal(f64::to_bits(r)))
                    },
                    // signed integers
                    (LiteralKind::I32Literal(i0), LiteralKind::I32Literal(i1)) => {
                        let r = i0 $op i1;
                        Var(LiteralKind::I32Literal(r))
                    }
                    (LiteralKind::I32Literal(i0), LiteralKind::I64Literal(i1)) => {
                        let r = i0 as i64 $op i1;
                        Var(LiteralKind::I64Literal(r))
                    }
                    (LiteralKind::I64Literal(i0), LiteralKind::I32Literal(i1)) => {
                        let r = i0 $op i1 as i64;
                        Var(LiteralKind::I64Literal(r))
                    }
                    (LiteralKind::I64Literal(i0), LiteralKind::I64Literal(i1)) => {
                        let r = i0 $op i1;
                        Var(LiteralKind::I64Literal(r))
                    }
                    // unsigned integers
                    (LiteralKind::U32Literal(u0), LiteralKind::U32Literal(u1)) => {
                        let r = u0 $op u1;
                        Var(LiteralKind::U32Literal(r))
                    }
                    (LiteralKind::U32Literal(u0), LiteralKind::U64Literal(u1)) => {
                        let r = u0 as u64 $op u1;
                        Var(LiteralKind::U64Literal(r))
                    }
                    (LiteralKind::U64Literal(u0), LiteralKind::U32Literal(u1)) => {
                        let r = u0 $op u1 as u64;
                        Var(LiteralKind::U64Literal(r))
                    }
                    (LiteralKind::U64Literal(u0), LiteralKind::U64Literal(u1)) => {
                        let r = u0 $op u1;
                        Var(LiteralKind::U64Literal(r))
                    }
                    (l0, l1) => panic!("incompatible types[{:?} & {:?}] in {} operation", l0, l1, stringify!($ty)),
                }
            }
        }
    }
}

macro_rules! impl_arith_for_var_small {
    ($ty:ty, $func:ident, $rty:ty, $gty:ty, $p1:path, $p2:path, $op:tt) => {
        impl $ty for Var<LiteralKind> {
            type Output = Self;
        
            fn $func(self, other: $rty) -> Self::Output {
                match self.0 {
                    $p1(f0) => {
                        let r = f0 $op other;
                        Var($p1(r))
                    }
                    $p2(f0) => {
                        // let r = f64::from_bits(f0) + other as f64;
                        let r = f0 $op other as $gty;
                        Var($p2(r as $gty))
                    }
                    _ => panic!("incompatible types[{:?} & {:?}] in {} operation", self, other, stringify!($ty)),
                }
            }
        }
    }
}

macro_rules! impl_arith_for_var_big {
    ($ty:ty, $func:ident, $rty:ty, $gty:ty, $p1:path, $p2:path, $op:tt) => {
        impl $ty for Var<LiteralKind> {
            type Output = Self;
        
            fn $func(self, other: $rty) -> Self::Output {
                match self.0 {
                    $p1(f0) => {
                        let r = f0 as $rty $op other;
                        Var($p2(r))
                    }
                    $p2(f0) => {
                        // let r = f64::from_bits(f0) + other as f64;
                        let r = f0 $op other;
                        Var($p2(r))
                    }
                    _ => panic!("incompatible types[{:?} & {:?}] in {} operation", self, other, stringify!($ty)),
                }
            }
        }
    }
}

macro_rules! impl_arith_for_var_float_small {
    ($ty:ty, $func:ident, $op:tt) => {
        impl $ty for Var<LiteralKind> {
            type Output = Self;

            fn $func(self, other: f32) -> Self::Output {
                match self.0 {
                    LiteralKind::F32Literal(f0) => {
                        let r = f32::from_bits(f0) $op other;
                        Var(LiteralKind::F32Literal(f32::to_bits(r)))
                    }
                    LiteralKind::F64Literal(f0) => {
                        let r = f64::from_bits(f0) $op other as f64;
                        Var(LiteralKind::F64Literal(f64::to_bits(r)))
                    }
                    _ => panic!("incompatible types[{:?} & {:?}] in {} operation", self, other, stringify!($ty)),
                }
            }
        }
    }
}

macro_rules! impl_arith_for_var_float_big {
    ($ty:ty, $func:ident, $op:tt) => {
        impl $ty for Var<LiteralKind> {
            type Output = Self;

            fn $func(self, other: f64) -> Self::Output {
                match self.0 {
                    LiteralKind::F32Literal(f0) => {
                        let r = f32::from_bits(f0) as f64 $op other;
                        Var(LiteralKind::F64Literal(f64::to_bits(r)))
                    }
                    LiteralKind::F64Literal(f0) => {
                        let r = f64::from_bits(f0) $op other;
                        Var(LiteralKind::F64Literal(f64::to_bits(r)))
                    }
                    _ => panic!("incompatible types[{:?} & {:?}] in {} operation", self, other, stringify!($ty)),
                }
            }
        }
    }
}


macro_rules! impl_from_for_lit_kind {
    ($ty:ty, $path:path) => {
        impl From<$ty> for LiteralKind {
            fn from(src: $ty) -> Self {
                $path(src)
            }
        }        
    }
}