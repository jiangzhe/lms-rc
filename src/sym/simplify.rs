use super::Symbol;
use crate::ast::*;
use crate::Result;
use std::collections::HashMap;

pub fn simplify(expr: &mut Expr, syms: &HashMap<Symbol, Literal>) -> Result<bool> {
    Simplify::with_syms(syms).transform_expr(expr)
}

struct Simplify<'map> {
    syms: &'map HashMap<Symbol, Literal>,
}

impl<'map> ExprTransformer for Simplify<'map> {
    fn transform_expr(&mut self, expr: &mut Expr) -> Result<bool> {
        match expr {
            Expr::BinOp(BinOp { op_ty, left, right }) => {
                let mut r = false;
                r |= self.transform_expr(left.as_mut())?;
                r |= self.transform_expr(right.as_mut())?;

                if let Some(new_expr) = self.simplify_bo(
                    op_ty,
                    &ExprOrLit::Expr(left.as_ref()),
                    &ExprOrLit::Expr(right.as_ref()),
                    r,
                )? {
                    *expr = new_expr;
                    return Ok(true);
                }
                Ok(r)
            }
            Expr::UnaryOp(UnaryOp { op_ty, value }) => {
                if self.transform_expr(value.as_mut())? {
                    if let Some(new_expr) =
                        self.simplify_uo(op_ty, &ExprOrLit::Expr(value.as_ref()))?
                    {
                        *expr = new_expr;
                    }
                    return Ok(true);
                }
                Ok(false)
            }
            Expr::Symbol(sym) => {
                if let Some(lit) = self.syms.get(sym) {
                    *expr = Expr::Literal(lit.clone());
                    return Ok(true);
                }
                Ok(false)
            }
            Expr::Literal(_) => Ok(false),
            _ => todo!(),
        }
    }

    fn transform_lambda(&mut self, lambda: &mut Lambda) -> Result<bool> {
        self.transform_expr(lambda.body.as_mut())
    }
}

impl<'map> Simplify<'map> {
    pub fn with_syms(syms: &'map HashMap<Symbol, Literal>) -> Self {
        Simplify { syms }
    }

    #[inline]
    fn simplify_bo(
        &self,
        op_ty: &BinOpType,
        left: &ExprOrLit,
        right: &ExprOrLit,
        recreate: bool,
    ) -> Result<Option<Expr>> {
        if let (Some(lit0), Some(lit1)) = (left.as_lit(), right.as_lit()) {
            let r = lit0.apply_bin_op(lit1, op_ty)?;
            return Ok(Some(Expr::Literal(r)));
        }
        if let Some(sym0) = left.as_symbol() {
            if let Some(lit0) = self.syms.get(sym0) {
                return self.simplify_bo(op_ty, &ExprOrLit::Lit(lit0), right, true);
            }
        }
        if let Some(sym1) = right.as_symbol() {
            if let Some(lit1) = self.syms.get(sym1) {
                return self.simplify_bo(op_ty, left, &ExprOrLit::Lit(lit1), true);
            }
        }
        if recreate {
            return Ok(Some(Expr::BinOp(BinOp {
                op_ty: op_ty.clone(),
                left: Box::new(left.to_expr()),
                right: Box::new(right.to_expr()),
            })));
        }
        Ok(None)
    }

    #[inline]
    fn simplify_uo(&self, op_ty: &UnaryOpType, expr: &ExprOrLit) -> Result<Option<Expr>> {
        if let Some(lit) = expr.as_lit() {
            let r = lit.apply_unary_op(op_ty)?;
            return Ok(Some(Expr::Literal(r)));
        }
        if let Some(sym) = expr.as_symbol() {
            if let Some(lit) = self.syms.get(sym) {
                return self.simplify_uo(op_ty, &ExprOrLit::Lit(lit));
            }
        }
        Ok(None)
    }
}

/// helper struct to avoid copy
enum ExprOrLit<'a> {
    Expr(&'a Expr),
    Lit(&'a Literal),
}

impl<'a> ExprOrLit<'a> {
    fn as_lit(&self) -> Option<&Literal> {
        match self {
            ExprOrLit::Expr(expr) => expr.as_lit(),
            ExprOrLit::Lit(lit) => Some(lit),
        }
    }

    fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            ExprOrLit::Expr(expr) => expr.as_symbol(),
            ExprOrLit::Lit(_) => None,
        }
    }

    fn to_expr(&self) -> Expr {
        match self {
            ExprOrLit::Expr(expr) => (*expr).clone(),
            ExprOrLit::Lit(lit) => Expr::Literal((*lit).clone()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simplify_bin_op() {
        let syms = {
            let mut tmp = HashMap::new();
            tmp.insert(Symbol::named("a", I32), 1.into());
            tmp
        };

        let v1 = Var::new_symbol("a", I32);
        let mut v2 = v1 + 1;
        simplify(&mut v2.expr, &syms).unwrap();
        assert_eq!(2, v2.expr.as_lit().unwrap().as_i32().unwrap());

        let mut v3 = 1 + Var::new_symbol("a", I32) + 1;
        simplify(&mut v3.expr, &syms).unwrap();
        assert_eq!(3, v3.expr.as_lit().unwrap().as_i32().unwrap());

        let mut v4 = Var::lit_i32(1);
        simplify(&mut v4.expr, &syms).unwrap();
        assert_eq!(1, v4.expr.as_lit().unwrap().as_i32().unwrap());
    }

    #[test]
    fn test_simplify_unary_op() {
        let syms = {
            let mut tmp = HashMap::new();
            tmp.insert(Symbol::named("a", I32), 1.into());
            tmp
        };
        let v1 = Var::new_symbol("a", I32);
        let mut v2 = -v1;
        simplify(&mut v2.expr, &syms).unwrap();
        assert_eq!(-1, v2.expr.as_lit().unwrap().as_i32().unwrap());
    }
}
