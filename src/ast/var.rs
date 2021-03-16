use super::*;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

/// Var represents a variable of linear type in the staging program.
///
/// Var is generic over its actual value.
///
/// Scalar operations defined on Var<ScalarType>.
///
/// Builder operations defined on Var<AppenderType>, Var<MergerType>, Var<DictMergerType>,
/// Var<GroupMergerType>, Var<VecMergerType>.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var<T> {
    pub(crate) expr: Expr,
    _marker: PhantomData<T>,
}

impl<T> Var<T> {
    /// Create a new var with given expression.
    ///
    /// This function is only for internal usage.
    /// The caller must make sure the parameter type
    /// is consistent with input expression.
    fn new(expr: Expr) -> Self {
        Var {
            expr,
            _marker: PhantomData,
        }
    }
}

impl<T> From<Var<T>> for Expr {
    fn from(src: Var<T>) -> Self {
        src.expr
    }
}

impl Var<bool> {
    /// Create a new var with a literal.
    pub fn lit_bool(value: bool) -> Self {
        Var::new(Expr::Literal(value.into()))
    }

    /// Create a new var with an expression.
    #[inline]
    pub fn expr_bool(expr: Expr) -> Self {
        assert!(
            expr.ty().is_bool(),
            "Imcompatible type {:?} to construct a {:?} var",
            expr.ty(),
            stringify!(BoolType)
        );
        Var::new(expr)
    }
}

impl TypeInference for Var<bool> {
    fn ty(&self) -> Type {
        Type::Bool
    }
}

impl Not for Var<bool> {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self.expr {
            Expr::Literal(Literal::Bool(v)) => Var::lit_bool(v),
            Expr::UnaryOp(UnaryOp {
                op_ty: UnaryOpType::Not,
                value,
                ..
            }) => Var::expr_bool(*value),
            other => Var::expr_bool(Expr::UnaryOp(UnaryOp::not(other))),
        }
    }
}

impl Var<String> {
    /// Create a new var with a literal.
    pub fn lit_string(value: String) -> Self {
        Var::new(Expr::Literal(value.into()))
    }

    /// Create a new var with an expression.
    #[inline]
    pub fn expr_string(expr: Expr) -> Self {
        assert!(
            expr.ty().is_string(),
            "Imcompatible type {:?} to construct a {:?} var",
            expr.ty(),
            stringify!(StringType)
        );
        Var::new(expr)
    }

    /// Equality check on two vars, and returns a bool var
    pub fn eq(self, other: Self) -> Var<bool> {
        Var::new(Expr::BinOp(BinOp::eq(self.expr, other.expr)))
    }

    /// Non-equality check on two vars, and returns a bool var
    pub fn ne(self, other: Self) -> Var<bool> {
        Var::new(Expr::BinOp(BinOp::ne(self.expr, other.expr)))
    }
}

impl TypeInference for Var<String> {
    fn ty(&self) -> Type {
        Type::String
    }
}

impl_num_var!(u8, lit_u8, expr_u8, is_u8, Type::U8);
impl_num_var!(u32, lit_u32, expr_u32, is_u32, Type::U32);
impl_num_var!(i32, lit_i32, expr_i32, is_i32, Type::I32);
impl_num_var!(u64, lit_u64, expr_u64, is_u64, Type::U64);
impl_num_var!(i64, lit_i64, expr_i64, is_i64, Type::I64);
impl_num_var!(f32, lit_f32, expr_f32, is_f32, Type::F32);
impl_num_var!(f64, lit_f64, expr_f64, is_f64, Type::F64);

impl_arith_for_var_num!(Add, Add<u8>, u8, add, +, as_u8, lit_u8, expr_u8, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<u8>, u8, sub, -, as_u8, lit_u8, expr_u8, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<u8>, u8, mul, *, as_u8, lit_u8, expr_u8, BinOp::mul);
impl_arith_for_var_num!(Div, Div<u8>, u8, div, /, as_u8, lit_u8, expr_u8, BinOp::div);

impl_arith_for_var_num!(Add, Add<u32>, u32, add, +, as_u32, lit_u32, expr_u32, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<u32>, u32, sub, -, as_u32, lit_u32, expr_u32, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<u32>, u32, mul, *, as_u32, lit_u32, expr_u32, BinOp::mul);
impl_arith_for_var_num!(Div, Div<u32>, u32, div, /, as_u32, lit_u32, expr_u32, BinOp::div);

impl_arith_for_var_num!(Add, Add<i32>, i32, add, +, as_i32, lit_i32, expr_i32, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<i32>, i32, sub, -, as_i32, lit_i32, expr_i32, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<i32>, i32, mul, *, as_i32, lit_i32, expr_i32, BinOp::mul);
impl_arith_for_var_num!(Div, Div<i32>, i32, div, /, as_i32, lit_i32, expr_i32, BinOp::div);

impl_arith_for_var_num!(Add, Add<u64>, u64, add, +, as_u64, lit_u64, expr_u64, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<u64>, u64, sub, -, as_u64, lit_u64, expr_u64, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<u64>, u64, mul, *, as_u64, lit_u64, expr_u64, BinOp::mul);
impl_arith_for_var_num!(Div, Div<u64>, u64, div, /, as_u64, lit_u64, expr_u64, BinOp::div);

impl_arith_for_var_num!(Add, Add<i64>, i64, add, +, as_i64, lit_i64, expr_i64, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<i64>, i64, sub, -, as_i64, lit_i64, expr_i64, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<i64>, i64, mul, *, as_i64, lit_i64, expr_i64, BinOp::mul);
impl_arith_for_var_num!(Div, Div<i64>, i64, div, /, as_i64, lit_i64, expr_i64, BinOp::div);

impl_arith_for_var_num!(Add, Add<f32>, f32, add, +, as_f32, lit_f32, expr_f32, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<f32>, f32, sub, -, as_f32, lit_f32, expr_f32, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<f32>, f32, mul, *, as_f32, lit_f32, expr_f32, BinOp::mul);
impl_arith_for_var_num!(Div, Div<f32>, f32, div, /, as_f32, lit_f32, expr_f32, BinOp::div);

impl_arith_for_var_num!(Add, Add<f64>, f64, add, +, as_f64, lit_f64, expr_f64, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<f64>, f64, sub, -, as_f64, lit_f64, expr_f64, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<f64>, f64, mul, *, as_f64, lit_f64, expr_f64, BinOp::mul);
impl_arith_for_var_num!(Div, Div<f64>, f64, div, /, as_f64, lit_f64, expr_f64, BinOp::div);

impl Neg for Var<i32> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.expr {
            Expr::Literal(Literal::I32(v)) => Var::lit_i32(-v),
            Expr::UnaryOp(UnaryOp {
                op_ty: UnaryOpType::Neg,
                value,
                ..
            }) => Var::expr_i32(*value),
            other => Var::expr_i32(Expr::UnaryOp(UnaryOp::neg(other))),
        }
    }
}

impl Neg for Var<i64> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.expr {
            Expr::Literal(Literal::I64(v)) => Var::lit_i64(-v),
            Expr::UnaryOp(UnaryOp {
                op_ty: UnaryOpType::Neg,
                value,
                ..
            }) => Var::expr_i64(*value),
            other => Var::expr_i64(Expr::UnaryOp(UnaryOp::neg(other))),
        }
    }
}

/// Implements methods on appender var.
impl Var<AppenderType> {
    /// Create a new var of appender with given item type.
    pub fn appender(item_ty: Type) -> Self {
        Var::new(Expr::NewAppender(NewAppender { item_ty }))
    }

    /// Evaluate the appender and returns a var of vector.
    pub fn eval(self) -> Var<VectorType> {
        let result = Eval(Box::new(self.expr));
        assert!(result.ty().is_vector());
        Var::new(Expr::Eval(result))
    }

    /// Merge the appender with given item and return the updated appender.
    ///
    /// The internal implementation of merging might be executed in place
    /// instead of constructing a brand new appender.
    pub fn merge<T>(self, item: T) -> Self
    where
        T: Into<Expr>,
    {
        let m = match self.expr {
            Expr::NewAppender(appender) => appender.merge(item),
            Expr::Merge(merge) => merge.merge(item),
            _ => unreachable!(),
        };
        Var::new(Expr::Merge(m))
    }
}

impl TypeInference for Var<AppenderType> {
    fn ty(&self) -> Type {
        self.expr.ty()
    }
}

impl Var<MergerType> {
    /// Create a new var of merger with given item type and operator.
    pub fn merger(item_ty: Type, op_ty: BinOpType) -> Self {
        Var::new(Expr::NewMerger(NewMerger { item_ty, op_ty }))
    }

    /// Evaluate the merger with static type, e.g. i32, i64, String, etc.
    /// Returns the var of static type.
    pub fn eval_static<T>(self) -> Var<T>
    where
        T: StaticType,
    {
        let result = Eval(Box::new(self.expr));
        assert!(
            result.ty() == T::ty(),
            "Imcompatible types[{:?} and {:?}] on eval operation",
            result.ty(),
            T::ty()
        );
        Var::new(Expr::Eval(result))
    }

    /// Evaluate the merger with dynamic type, e.g. Tuple, Vector, etc.
    ///
    /// The input type should be consistent with the eval type.
    pub fn eval_dynamic<T>(self, item_ty: T) -> Var<T>
    where
        T: DynamicType,
    {
        let item_ty = item_ty.ty();
        let result = Eval(Box::new(self.expr));
        assert!(
            result.ty() == item_ty,
            "Imcompatible types[{:?} and {:?}] on eval operation",
            result.ty(),
            item_ty
        );
        Var::new(Expr::Eval(result))
    }

    /// Merge the merger with given item and return the updated merger.
    pub fn merge<T>(self, item: T) -> Self
    where
        T: Into<Expr>,
    {
        let m = match self.expr {
            Expr::NewMerger(merger) => merger.merge(item),
            Expr::Merge(merge) => merge.merge(item),
            _ => unreachable!(),
        };
        Var::new(Expr::Merge(m))
    }
}

impl TypeInference for Var<MergerType> {
    fn ty(&self) -> Type {
        self.expr.ty()
    }
}

impl Var<DictMergerType> {
    /// Create a new var of dictmerger with given key, value type and operator.
    pub fn dictmerger(key_ty: Type, value_ty: Type, op_ty: BinOpType) -> Self {
        Var::new(Expr::NewDictMerger(NewDictMerger {
            key_ty,
            value_ty,
            op_ty,
        }))
    }

    /// Evaluate the dictmerger and returns a var of dict.
    pub fn eval(self) -> Var<DictType> {
        let result = Eval(Box::new(self.expr));
        assert!(result.ty().is_dict());
        Var::new(Expr::Eval(result))
    }

    /// Merge the dictmerger with given item and return the updated dictmerger.
    pub fn merge<T>(self, item: T) -> Self
    where
        T: Into<Expr>,
    {
        let m = match self.expr {
            Expr::NewDictMerger(dictmerger) => dictmerger.merge(item),
            Expr::Merge(merge) => merge.merge(item),
            _ => unreachable!(),
        };
        Var::new(Expr::Merge(m))
    }
}

/// Implements methods on vector var.
impl Var<VectorType> {
    /// Create a new var of vector with given items
    pub fn vector<T>(items: Vec<T>) -> Self
    where
        T: Into<Expr>,
    {
        assert!(
            !items.is_empty(),
            "Empty list of items not allowed in creating new vector, use appender instead"
        );
        let items: Vec<Expr> = items.into_iter().map(Into::into).collect();
        let item_ty: Type = items[0].ty();
        Var::new(Expr::NewVector(NewVector { item_ty, items }))
    }

    pub fn pfor<B, T, F>(self, builder: Var<B>, f: F) -> For
    where
        T: Into<Expr>,
        F: FnOnce(Var<B>, Var<u64>, T) -> Var<B>,
        F: 'static,
    {
        let iter = Iter {
            data: Box::new(self.expr),
            start: None,
            end: None,
        };

        // function params
        // let p1 = Var::new()

        // // function body

        // For{
        //     iters: vec![iter],
        //     builder: Box::new(builder.expr),
        //     func:
        // }
        todo!()
    }
}

/// Marker trait for var with builder type
pub trait BuilderVar {
    fn expr(self) -> Expr;
}

impl BuilderVar for Var<AppenderType> {
    fn expr(self) -> Expr {
        self.expr
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_var_lit_add() {
        let v1 = Var::lit_i32(1);
        let v2 = Var::lit_i32(2);
        let v3 = v1 + v2;
        assert_eq!(Var::lit_i32(3), v3);
        let v4 = Var::lit_i32(4);
        assert_eq!(Var::lit_i32(8), v4 + 4);
    }

    #[test]
    fn test_var_appender() {
        let a1 = Var::appender(Type::I32);
        let a2 = a1.merge(1);
        let a3 = a2.merge(2);
        let a4 = a3.merge(3);
        println!("{}", a4.eval().expr);
    }

    #[test]
    fn test_var_merger() {
        let m1 = Var::merger(Type::I32, BinOpType::Add);
        let m2 = m1.merge(1);
        let m3 = m2.merge(2);
        let m4 = m3.merge(3);
        println!("{}", m4.eval_static::<i32>().expr);
    }

    #[test]
    fn test_var_vector() {
        let v1 = Var::vector(vec![1, 2, 3]);
        let m1 = Var::merger(Type::I32, BinOpType::Add);
        let f1 = v1.pfor(m1, |b, i, e: Var<i32>| b.merge(e));
    }
}
