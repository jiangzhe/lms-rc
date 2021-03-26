use super::Symbol;
use crate::ast::*;
use crate::Result;
use std::collections::HashMap;

pub fn simplify(expr: &mut Expr, syms: HashMap<Symbol, Literal>) -> Result<bool> {
    Simplify::with_syms(syms).transform_expr(expr)
}

struct Simplify {
    syms: HashMap<Symbol, Literal>,
}

impl ExprTransformer for Simplify {
    fn transform_expr(&mut self, expr: &mut Expr) -> Result<bool> {
        match expr {
            Expr::BinOp(BinOp { op_ty, left, right }) => {
                let mut r = false;
                r |= self.transform_expr(left.as_mut())?;
                r |= self.transform_expr(right.as_mut())?;

                if let Some(new_expr) = self.simplify_bo(op_ty, left, right, r)? {
                    *expr = new_expr;
                    return Ok(true);
                }
                Ok(false)
            }
            Expr::Symbol(_) | Expr::Literal(_) => Ok(false),
            _ => todo!(),
        }
    }

    fn transform_lambda(&mut self, lambda: &mut Lambda) -> Result<bool> {
        self.transform_expr(lambda.body.as_mut())
    }
}

impl Simplify {
    pub fn new() -> Self {
        Simplify {
            syms: HashMap::new(),
        }
    }

    pub fn with_syms(syms: HashMap<Symbol, Literal>) -> Self {
        Simplify { syms }
    }

    pub fn with_sym(mut self, sym: Symbol, lit: Literal) -> Self {
        self.syms.insert(sym, lit);
        self
    }

    #[inline]
    fn simplify_bo(
        &self,
        op_ty: &BinOpType,
        left: &Expr,
        right: &Expr,
        recreate: bool,
    ) -> Result<Option<Expr>> {
        if let (Some(lit0), Some(lit1)) = (left.as_lit(), right.as_lit()) {
            let r = lit0.apply_bin_op(lit1, op_ty)?;
            return Ok(Some(Expr::Literal(r)));
        }
        if let Some(sym0) = left.as_symbol() {
            if let Some(lit0) = self.syms.get(sym0) {
                return self.simplify_bo(op_ty, &Expr::Literal(lit0.clone()), right, true);
            }
        }
        if let Some(sym1) = right.as_symbol() {
            if let Some(lit1) = self.syms.get(sym1) {
                return self.simplify_bo(op_ty, left, &Expr::Literal(lit1.clone()), true);
            }
        }
        if recreate {
            return Ok(Some(Expr::BinOp(BinOp {
                op_ty: op_ty.clone(),
                left: Box::new(left.clone()),
                right: Box::new(right.clone()),
            })));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::*;

    #[test]
    fn test_simplify_bin_op() {
        let v1 = Var::new_symbol("a", I32);
        let mut v2 = v1 + 1;
        Simplify::new()
            .with_sym(Symbol::named("a", I32), 1.into())
            .transform_expr(&mut v2.expr)
            .unwrap();
        assert_eq!(2, v2.expr.as_lit().unwrap().as_i32().unwrap());

        let mut v3 = 1 + Var::new_symbol("a", I32) + 1;
        Simplify::new()
            .with_sym(Symbol::named("a", I32), 1.into())
            .transform_expr(&mut v3.expr)
            .unwrap();
        assert_eq!(3, v3.expr.as_lit().unwrap().as_i32().unwrap());

        let mut v4 = Var::lit_i32(1);
        Simplify::new()
            .with_sym(Symbol::named("a", I32), 1.into())
            .transform_expr(&mut v4.expr)
            .unwrap();
        assert_eq!(1, v4.expr.as_lit().unwrap().as_i32().unwrap());
    }
}
