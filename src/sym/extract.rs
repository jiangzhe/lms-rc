use super::Symbol;
use crate::ast::{Expr, ExprVisitor, Lambda};
use crate::Result;
use std::collections::HashSet;

pub fn extract(expr: &Expr) -> HashSet<Symbol> {
    let mut ex = Extract {
        syms: HashSet::new(),
    };
    // this transformation won't fail
    ex.visit_expr(expr).unwrap();
    ex.syms
}

struct Extract {
    syms: HashSet<Symbol>,
}

impl ExprVisitor for Extract {
    fn visit_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Lambda(lambda) => {
                self.visit_lambda(lambda)?;
            }
            Expr::Symbol(sym) => {
                self.syms.insert(sym.clone());
            }
            _ => expr.traverse_children(self)?,
        };
        Ok(())
    }

    fn visit_lambda(&mut self, lambda: &Lambda) -> Result<()> {
        for p in &lambda.params {
            self.syms.insert(p.clone());
        }
        lambda.body.traverse_children(self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_extract_symbols() {
        let v1 = Var::new_symbol("a", I32);
        let v2 = v1 + 100;
        assert_eq!(1, extract(&v2.expr).len());
        let v3 = v2 + Var::new_symbol("b", I32);
        assert_eq!(2, extract(&v3.expr).len());
    }
}
