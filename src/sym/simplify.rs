use super::Symbol;
use crate::ast::*;
use crate::Result;
use std::collections::{HashMap, HashSet};

struct Simplify {
    syms: HashMap<Symbol, Literal>,
    simplified: HashSet<Symbol>,
}

impl ExprTransformer for Simplify {
    fn transform_expr(&mut self, expr: &mut Expr) -> Result<()> {
        // match expr {
        //     Expr::BinOp(bo) => {
        //         bo.left
        //     }
        // }
        todo!()
    }

    fn transform_lambda(&mut self, lambda: &mut Lambda) -> Result<()> {
        todo!()
    }
}

impl Simplify {
    pub fn new(syms: HashMap<Symbol, Literal>) -> Self {
        Simplify {
            syms,
            simplified: HashSet::new(),
        }
    }

    #[inline]
    fn simplify_bo(&self, bo: &BinOp) -> Option<Expr> {
        let BinOp { op_ty, left, right } = bo;
        // match (left.as_ref(), right.as_ref()) {
        //     (Expr::Symbol(sym), Expr::Literal(rlit)) => {
        //         if let Some(llit) = self.syms.get(sym) {

        //         }
        //     }
        // }

        todo!()
    }
}
