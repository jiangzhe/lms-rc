use super::*;
use crate::sym::Symbol;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};

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
    pub fn new(expr: Expr) -> Self {
        Var {
            expr,
            _marker: PhantomData,
        }
    }

    pub fn zip<U>(self, other: Var<U>) -> Var<TupleType> {
        match (self.expr.is_tuple(), other.expr.is_tuple()) {
            (true, true) => {
                let mut tp0 = self.expr.to_tuple();
                let tp1 = other.expr.to_tuple();
                tp0.0.extend(tp1.0);
                Var::new(Expr::Tuple(tp0))
            }
            (true, false) => {
                let mut tp0 = self.expr.to_tuple();
                tp0.0.push(other.expr);
                Var::new(Expr::Tuple(tp0))
            }
            (false, true) => {
                let tp1 = other.expr.to_tuple();
                let mut exprs = Vec::with_capacity(tp1.0.len() + 1);
                exprs.push(self.expr);
                exprs.extend(tp1.0);
                Var::new(Expr::Tuple(Tuple(exprs)))
            }
            (false, false) => {
                let mut exprs = Vec::with_capacity(2);
                exprs.push(self.expr);
                exprs.push(other.expr);
                Var::new(Expr::Tuple(Tuple(exprs)))
            }
        }
    }

    /// Clone a symbol.
    ///
    /// The type should be consistent with generic
    /// type T.
    /// This factory method is used on constructing
    /// Function from input DSL.
    pub fn clone_symbol(sym: Symbol) -> Self {
        Var {
            expr: Expr::Symbol(sym),
            _marker: PhantomData,
        }
    }

    /// Create a new symbol.
    pub fn new_symbol<S>(name: S, ty: T) -> Self
    where
        S: Into<String>,
        T: Into<Type>,
    {
        Var {
            expr: Expr::Symbol(Symbol::named(name, ty)),
            _marker: PhantomData,
        }
    }
}

impl<T> From<Var<T>> for Expr {
    fn from(src: Var<T>) -> Self {
        src.expr
    }
}

impl<T> TypeInference for Var<T> {
    fn ty(&self) -> Type {
        self.expr.ty()
    }
}

impl Var<Bool> {
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

impl Not for Var<Bool> {
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

impl Var<Str> {
    /// Create a new var with a literal.
    pub fn lit_string(value: String) -> Self {
        Var::new(Expr::Literal(value.into()))
    }

    /// Create a new var with an expression.
    #[inline]
    pub fn expr_string(expr: Expr) -> Self {
        assert!(
            expr.ty().is_str(),
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

impl_num_var!(U8, u8, lit_u8, expr_u8, is_u8);
impl_num_var!(U32, u32, lit_u32, expr_u32, is_u32);
impl_num_var!(I32, i32, lit_i32, expr_i32, is_i32);
impl_num_var!(U64, u64, lit_u64, expr_u64, is_u64);
impl_num_var!(I64, i64, lit_i64, expr_i64, is_i64);
impl_num_var!(F32, f32, lit_f32, expr_f32, is_f32);
impl_num_var!(F64, f64, lit_f64, expr_f64, is_f64);

impl_arith_for_var_num!(Add, Add<Var<U8>>, Add<u8>, Var<U8>, u8, add, +, as_u8, lit_u8, expr_u8, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<Var<U8>>, Sub<u8>, Var<U8>, u8, sub, -, as_u8, lit_u8, expr_u8, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<Var<U8>>, Mul<u8>, Var<U8>, u8, mul, *, as_u8, lit_u8, expr_u8, BinOp::mul);
impl_arith_for_var_num!(Div, Div<Var<U8>>, Div<u8>, Var<U8>, u8, div, /, as_u8, lit_u8, expr_u8, BinOp::div);
impl_arith_for_var_num!(Rem, Rem<Var<U8>>, Rem<u8>, Var<U8>, u8, rem, %, as_u8, lit_u8, expr_u8, BinOp::rem);

impl_arith_for_var_num!(Add, Add<Var<U32>>, Add<u32>, Var<U32>, u32, add, +, as_u32, lit_u32, expr_u32, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<Var<U32>>, Sub<u32>, Var<U32>, u32, sub, -, as_u32, lit_u32, expr_u32, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<Var<U32>>, Mul<u32>, Var<U32>, u32, mul, *, as_u32, lit_u32, expr_u32, BinOp::mul);
impl_arith_for_var_num!(Div, Div<Var<U32>>, Div<u32>, Var<U32>, u32, div, /, as_u32, lit_u32, expr_u32, BinOp::div);
impl_arith_for_var_num!(Rem, Rem<Var<U32>>, Rem<u32>, Var<U32>, u32, rem, /, as_u32, lit_u32, expr_u32, BinOp::rem);

impl_arith_for_var_num!(Add, Add<Var<I32>>, Add<i32>, Var<I32>, i32, add, +, as_i32, lit_i32, expr_i32, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<Var<I32>>, Sub<i32>, Var<I32>, i32, sub, -, as_i32, lit_i32, expr_i32, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<Var<I32>>, Mul<i32>, Var<I32>, i32, mul, *, as_i32, lit_i32, expr_i32, BinOp::mul);
impl_arith_for_var_num!(Div, Div<Var<I32>>, Div<i32>, Var<I32>, i32, div, /, as_i32, lit_i32, expr_i32, BinOp::div);
impl_arith_for_var_num!(Rem, Rem<Var<I32>>, Rem<i32>, Var<I32>, i32, rem, /, as_i32, lit_i32, expr_i32, BinOp::rem);

impl_arith_for_var_num!(Add, Add<Var<U64>>, Add<u64>, Var<U64>, u64, add, +, as_u64, lit_u64, expr_u64, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<Var<U64>>, Sub<u64>, Var<U64>, u64, sub, -, as_u64, lit_u64, expr_u64, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<Var<U64>>, Mul<u64>, Var<U64>, u64, mul, *, as_u64, lit_u64, expr_u64, BinOp::mul);
impl_arith_for_var_num!(Div, Div<Var<U64>>, Div<u64>, Var<U64>, u64, div, /, as_u64, lit_u64, expr_u64, BinOp::div);
impl_arith_for_var_num!(Rem, Rem<Var<U64>>, Rem<u64>, Var<U64>, u64, rem, /, as_u64, lit_u64, expr_u64, BinOp::rem);

impl_arith_for_var_num!(Add, Add<Var<I64>>, Add<i64>, Var<I64>, i64, add, +, as_i64, lit_i64, expr_i64, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<Var<I64>>, Sub<i64>, Var<I64>, i64, sub, -, as_i64, lit_i64, expr_i64, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<Var<I64>>, Mul<i64>, Var<I64>, i64, mul, *, as_i64, lit_i64, expr_i64, BinOp::mul);
impl_arith_for_var_num!(Div, Div<Var<I64>>, Div<i64>, Var<I64>, i64, div, /, as_i64, lit_i64, expr_i64, BinOp::div);
impl_arith_for_var_num!(Rem, Rem<Var<I64>>, Rem<i64>, Var<I64>, i64, rem, /, as_i64, lit_i64, expr_i64, BinOp::rem);

impl_arith_for_var_num!(Add, Add<Var<F32>>, Add<f32>, Var<F32>, f32, add, +, as_f32, lit_f32, expr_f32, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<Var<F32>>, Sub<f32>, Var<F32>, f32, sub, -, as_f32, lit_f32, expr_f32, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<Var<F32>>, Mul<f32>, Var<F32>, f32, mul, *, as_f32, lit_f32, expr_f32, BinOp::mul);
impl_arith_for_var_num!(Div, Div<Var<F32>>, Div<f32>, Var<F32>, f32, div, /, as_f32, lit_f32, expr_f32, BinOp::div);
impl_arith_for_var_num!(Rem, Rem<Var<F32>>, Rem<f32>, Var<F32>, f32, rem, /, as_f32, lit_f32, expr_f32, BinOp::rem);

impl_arith_for_var_num!(Add, Add<Var<F64>>, Add<f64>, Var<F64>, f64, add, +, as_f64, lit_f64, expr_f64, BinOp::add);
impl_arith_for_var_num!(Sub, Sub<Var<F64>>, Sub<f64>, Var<F64>, f64, sub, -, as_f64, lit_f64, expr_f64, BinOp::sub);
impl_arith_for_var_num!(Mul, Mul<Var<F64>>, Mul<f64>, Var<F64>, f64, mul, *, as_f64, lit_f64, expr_f64, BinOp::mul);
impl_arith_for_var_num!(Div, Div<Var<F64>>, Div<f64>, Var<F64>, f64, div, /, as_f64, lit_f64, expr_f64, BinOp::div);
impl_arith_for_var_num!(Rem, Rem<Var<F64>>, Rem<f64>, Var<F64>, f64, rem, /, as_f64, lit_f64, expr_f64, BinOp::rem);

impl Neg for Var<I32> {
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

impl Neg for Var<I64> {
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

impl Neg for Var<F32> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.expr {
            Expr::Literal(Literal::F32(v)) => {
                let f = f32::from_bits(v);
                Var::lit_f32(-f)
            }
            Expr::UnaryOp(UnaryOp {
                op_ty: UnaryOpType::Neg,
                value,
                ..
            }) => Var::expr_f32(*value),
            other => Var::expr_f32(Expr::UnaryOp(UnaryOp::neg(other))),
        }
    }
}

impl Neg for Var<F64> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.expr {
            Expr::Literal(Literal::F64(v)) => {
                let f = f64::from_bits(v);
                Var::lit_f64(-f)
            }
            Expr::UnaryOp(UnaryOp {
                op_ty: UnaryOpType::Neg,
                value,
                ..
            }) => Var::expr_f64(*value),
            other => Var::expr_f64(Expr::UnaryOp(UnaryOp::neg(other))),
        }
    }
}

impl<B: BuilderType> Var<B> {
    /// pfor represents the parallel for expression of builder operation.
    /// The provided lambda will be called in parallel with non-overlapping
    /// items in iteration.
    pub fn pfor<T, F>(self, it: Var<VectorType>, f: F) -> Self
    where
        F: FnOnce(Self, Var<u64>, Var<T>) -> Self,
        F: 'static,
    {
        let iter = Iter {
            data: Box::new(it.expr),
            start: None,
            end: None,
        };

        let sym_b = Symbol::named("b", self.ty());
        let sym_i = Symbol::named("i", U64);
        let sym_e = Symbol::named("e", self.ty().merge());

        let b = Var::<B>::clone_symbol(sym_b.clone());
        let i = Var::<u64>::clone_symbol(sym_i.clone());
        let e = Var::<T>::clone_symbol(sym_e.clone());

        let body = f(b, i, e);

        let func = Lambda {
            params: vec![sym_b, sym_i, sym_e],
            body: Box::new(body.expr),
        };

        let pfor = For {
            iters: vec![iter],
            builder: Box::new(self.expr),
            func: Box::new(Expr::Lambda(func)),
        };
        Var::new(Expr::For(pfor))
    }
}

/// Implements methods on appender var.
impl Var<AppenderType> {
    /// Create a new var of appender with given item type.
    pub fn appender<T: Into<Type>>(item_ty: T) -> Self {
        Var::new(Expr::NewAppender(NewAppender {
            item_ty: item_ty.into(),
        }))
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
            Expr::NewAppender(a) => a.merge(item),
            Expr::Merge(m) => m.merge(item),
            Expr::Symbol(sym) => sym.merge(item),
            _ => unreachable!(),
        };
        Var::new(Expr::Merge(m))
    }
}

impl Var<MergerType> {
    /// Create a new var of merger with given item type and operator.
    pub fn new_merger<T: Into<Type>>(item_ty: T, op_ty: BinOpType) -> Self {
        Var::new(Expr::NewMerger(NewMerger {
            item_ty: item_ty.into(),
            op_ty,
        }))
    }

    /// Evaluate the merger with dynamic type, e.g. Tuple, Vector, etc.
    ///
    /// The input type should be consistent with the eval type.
    pub fn eval<T: Into<Type>>(self, item_ty: T) -> Var<T> {
        let item_ty = item_ty.into();
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
            Expr::NewMerger(m) => m.merge(item),
            Expr::Merge(m) => m.merge(item),
            Expr::Symbol(sym) => sym.merge(item),
            _ => unreachable!(),
        };
        Var::new(Expr::Merge(m))
    }
}

impl Var<DictMergerType> {
    /// Create a new var of dictmerger with given key, value type and operator.
    pub fn dictmerger<K: Into<Type>, V: Into<Type>>(
        key_ty: K,
        value_ty: V,
        op_ty: BinOpType,
    ) -> Self {
        Var::new(Expr::NewDictMerger(NewDictMerger {
            key_ty: key_ty.into(),
            value_ty: value_ty.into(),
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
            Expr::NewDictMerger(dm) => dm.merge(item),
            Expr::Merge(m) => m.merge(item),
            Expr::Symbol(sym) => sym.merge(item),
            _ => unreachable!(),
        };
        Var::new(Expr::Merge(m))
    }
}

impl Var<GroupMergerType> {
    /// Create a new var of groupmerger with given key, value type.
    pub fn groupmerger<K: Into<Type>, V: Into<Type>>(key_ty: K, value_ty: V) -> Self {
        Var::new(Expr::NewGroupMerger(NewGroupMerger {
            key_ty: key_ty.into(),
            value_ty: value_ty.into(),
        }))
    }

    /// Evaluate the groupmerger and returns a var of dict.
    pub fn eval(self) -> Var<DictType> {
        let result = Eval(Box::new(self.expr));
        assert!(result.ty().is_dict());
        Var::new(Expr::Eval(result))
    }

    /// Merge the groupmerger with given item and return the updated groupmerger.
    pub fn merge<T>(self, item: T) -> Self
    where
        T: Into<Expr>,
    {
        let m = match self.expr {
            Expr::NewGroupMerger(gm) => gm.merge(item),
            Expr::Merge(m) => m.merge(item),
            Expr::Symbol(sym) => sym.merge(item),
            _ => unreachable!(),
        };
        Var::new(Expr::Merge(m))
    }
}

impl Var<VecMergerType> {
    /// Create a new var of vecmerger with given item type and operator.
    pub fn vecmerger<T: Into<Type>>(item_ty: T, op_ty: BinOpType) -> Self {
        Var::new(Expr::NewVecMerger(NewVecMerger {
            item_ty: item_ty.into(),
            op_ty,
        }))
    }

    /// Evaluate the vecmerger and returns a var of vector.
    pub fn eval(self) -> Var<VectorType> {
        let result = Eval(Box::new(self.expr));
        assert!(result.ty().is_vector());
        Var::new(Expr::Eval(result))
    }

    pub fn merge<T>(self, item: T) -> Self
    where
        T: Into<Expr>,
    {
        let m = match self.expr {
            Expr::NewVecMerger(vm) => vm.merge(item),
            Expr::Merge(m) => m.merge(item),
            Expr::Symbol(sym) => sym.merge(item),
            _ => unreachable!(),
        };
        Var::new(Expr::Merge(m))
    }
}

/// Implements methods on vector var.
impl Var<VectorType> {
    /// Create a new var of vector with given items
    pub fn new_vector<T>(items: Vec<T>) -> Self
    where
        T: Into<Expr>,
    {
        assert!(
            !items.is_empty(),
            "Empty list of items not allowed in creating new vector, use appender instead"
        );
        let items: Vec<Expr> = items.into_iter().map(Into::into).collect();
        let item_ty: Type = items[0].ty();
        Var::new(Expr::Vector(Vector { item_ty, items }))
    }
}

impl Var<TupleType> {
    pub fn new_tuple(items: Vec<Expr>) -> Self {
        assert!(
            !items.is_empty(),
            "Empty list of items not allowed in creating new tuple"
        );
        Var::new(Expr::Tuple(Tuple(items)))
    }

    pub fn get<T>(&self, index: u32, ty: T) -> Var<T>
    where
        T: Into<Type>,
    {
        let ty: Type = ty.into();
        let get_field = GetField {
            tuple: Box::new(self.expr.clone()),
            index,
        };
        assert_eq!(
            ty,
            get_field.ty(),
            "Imcompatible type[{}] in get_field opertaion",
            ty
        );
        Var::new(Expr::GetField(get_field))
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
        let a1 = Var::appender(I32);
        let a2 = a1.merge(1);
        let a3 = a2.merge(2);
        let a4 = a3.merge(3);
        println!("{}", a4.eval().expr);
    }

    #[test]
    fn test_var_merger() {
        let m1 = Var::new_merger(I32, BinOpType::Add);
        let m2 = m1.merge(1);
        let m3 = m2.merge(2);
        let m4 = m3.merge(3);
        println!("{}", m4.eval(I32).expr);
    }

    #[test]
    fn test_var_vector() {
        let v1 = Var::new_vector(vec![1, 2, 3]);
        let m1 = Var::new_merger(I32, BinOpType::Add);
        let m2 = m1.pfor(v1, |b, _i, e: Var<i32>| b.merge(e));
        println!("{}", m2.expr);
    }

    #[test]
    fn test_var_tuple() {
        let v1 = Var::new_tuple(vec![1.into(), true.into()]);
        let v2 = v1.get(0, I32);
        println!("{}", v2.expr);

        let v3 = Var::lit_i32(1);
        let v4 = Var::lit_bool(true);
        let v5 = v3.zip(v4);
        println!("{}", v5.expr);
    }
}
