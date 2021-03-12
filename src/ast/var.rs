use super::*;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

/// Var represents a variable of linear type in the staging program
///
/// Var is generic over its actual value.
///
/// Scalar operations defined on Var<ScalarKind>
///
/// Builder operations defined on Var<BuilderKind>
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var<T>(T, Expr);

impl Var<ScalarKind> {
    /// Create a new var of scalar kind assigned a literal.
    pub fn lit<V>(value: V) -> Self
    where
        V: Into<Literal>,
    {
        let lit = value.into();
        let ty = lit.ty();
        Var(ty.scalar().unwrap(), Expr::Literal(lit))
    }

    /// Create a new var of scalar kind associated with an expression.
    /// Caller should ensure the consistency between kind and expr
    #[inline]
    pub fn scalar(kind: ScalarKind, expr: Expr) -> Self {
        Var(kind, expr)
    }
}

impl Var<AppenderKind> {
    pub fn appender() -> Self {
        todo!()
    }
}

impl Not for Var<ScalarKind> {
    type Output = Self;

    fn not(self) -> Self::Output {
        match (self.0, self.1) {
            (ScalarKind::Bool, Expr::Literal(Literal::Bool(v))) => Var::lit(v),
            (
                ScalarKind::Bool,
                Expr::UnaryOp(UnaryOp {
                    kind: UnaryOpKind::Not,
                    value,
                }),
            ) => Var::scalar(ScalarKind::Bool, *value),
            (ScalarKind::Bool, other) => {
                Var::scalar(ScalarKind::Bool, Expr::UnaryOp(UnaryOp::not(other)))
            }
            (ty, _) => panic!("incompatible type[{:?}] on Not operation", ty),
        }
    }
}

impl Neg for Var<ScalarKind> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match (self.0, self.1) {
            (ScalarKind::I32, Expr::Literal(Literal::I32(v))) => Var::lit(-v),
            (
                ScalarKind::I32,
                Expr::UnaryOp(UnaryOp {
                    kind: UnaryOpKind::Neg,
                    value,
                }),
            ) => Var::scalar(ScalarKind::I32, *value),
            (ScalarKind::I32, other) => {
                Var::scalar(ScalarKind::I32, Expr::UnaryOp(UnaryOp::neg(other)))
            }
            (ty, _) => panic!("incompatible type[{:?}] on Neg operation", ty),
        }
    }
}

impl_arith_for_var_scalar!(Add, add, +);
impl_arith_for_var_scalar!(Sub, sub, -);
impl_arith_for_var_scalar!(Mul, mul, *);
impl_arith_for_var_scalar!(Div, div, /);

impl_arith_for_var_builtin!(Add< u8>,  add,  u8,  as_u8,  ScalarKind::U8, +);
impl_arith_for_var_builtin!(Add<i32>, add, i32, as_i32, ScalarKind::I32, +);
impl_arith_for_var_builtin!(Add<u32>, add, u32, as_u32, ScalarKind::U32, +);
impl_arith_for_var_builtin!(Add<i64>, add, i64, as_i64, ScalarKind::I64, +);
impl_arith_for_var_builtin!(Add<u64>, add, u64, as_u64, ScalarKind::U64, +);
impl_arith_for_var_builtin!(Add<f32>, add, f32, as_f32, ScalarKind::F32, +);
impl_arith_for_var_builtin!(Add<f64>, add, f64, as_f64, ScalarKind::F64, +);

impl_arith_for_var_builtin!(Sub< u8>, sub,  u8,  as_u8,  ScalarKind::U8, -);
impl_arith_for_var_builtin!(Sub<i32>, sub, i32, as_i32, ScalarKind::I32, -);
impl_arith_for_var_builtin!(Sub<u32>, sub, u32, as_u32, ScalarKind::U32, -);
impl_arith_for_var_builtin!(Sub<i64>, sub, i64, as_i64, ScalarKind::I64, -);
impl_arith_for_var_builtin!(Sub<u64>, sub, u64, as_u64, ScalarKind::U64, -);
impl_arith_for_var_builtin!(Sub<f32>, sub, f32, as_f32, ScalarKind::F32, -);
impl_arith_for_var_builtin!(Sub<f64>, sub, f64, as_f64, ScalarKind::F64, -);

impl_arith_for_var_builtin!(Mul< u8>, mul,  u8,  as_u8,  ScalarKind::U8, *);
impl_arith_for_var_builtin!(Mul<i32>, mul, i32, as_i32, ScalarKind::I32, *);
impl_arith_for_var_builtin!(Mul<u32>, mul, u32, as_u32, ScalarKind::U32, *);
impl_arith_for_var_builtin!(Mul<i64>, mul, i64, as_i64, ScalarKind::I64, *);
impl_arith_for_var_builtin!(Mul<u64>, mul, u64, as_u64, ScalarKind::U64, *);
impl_arith_for_var_builtin!(Mul<f32>, mul, f32, as_f32, ScalarKind::F32, *);
impl_arith_for_var_builtin!(Mul<f64>, mul, f64, as_f64, ScalarKind::F64, *);

impl_arith_for_var_builtin!(Div< u8>, div,  u8,  as_u8,  ScalarKind::U8, /);
impl_arith_for_var_builtin!(Div<i32>, div, i32, as_i32, ScalarKind::I32, /);
impl_arith_for_var_builtin!(Div<u32>, div, u32, as_u32, ScalarKind::U32, /);
impl_arith_for_var_builtin!(Div<i64>, div, i64, as_i64, ScalarKind::I64, /);
impl_arith_for_var_builtin!(Div<u64>, div, u64, as_u64, ScalarKind::U64, /);
impl_arith_for_var_builtin!(Div<f32>, div, f32, as_f32, ScalarKind::F32, /);
impl_arith_for_var_builtin!(Div<f64>, div, f64, as_f64, ScalarKind::F64, /);

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
