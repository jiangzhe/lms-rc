use super::Symbol;
use crate::ast::{Expr, ExprTransformer, Lambda};
use crate::Result;
use std::collections::HashMap;

pub fn uniquify(expr: &mut Expr) -> Result<()> {
    let mut su = Uniquifier::new();
    su.transform_expr(expr)
}

struct Uniquifier {
    stack: HashMap<Symbol, Vec<u32>>,
    next_unique_id: HashMap<String, u32>,
}

impl ExprTransformer for Uniquifier {
    fn transform_expr(&mut self, expr: &mut Expr) -> Result<()> {
        match expr {
            Expr::Symbol(sym) => {
                // match and rename the symbol
                *sym = self.get(sym)?;
            }
            other => {
                other.apply_children(self)?;
            }
        }
        Ok(())
    }

    fn transform_lambda(&mut self, lambda: &mut Lambda) -> Result<()> {
        let Lambda {
            params,
            ref mut body,
        } = lambda;

        let orig_params = params.clone();
        for param in params {
            self.push(param);
            *param = self.get(param)?;
        }
        self.transform_expr(body.as_mut())?;

        for param in orig_params.iter().rev() {
            self.pop(param)?;
        }
        Ok(())
    }
}

impl Uniquifier {
    fn new() -> Self {
        Uniquifier {
            stack: HashMap::new(),
            next_unique_id: HashMap::new(),
        }
    }

    /// Push a new symbol onto stack and assign a unique id.
    /// Original symbol should be kept for removal later.
    fn push(&mut self, sym: &Symbol) {
        let stack_entry = self.stack.entry(sym.clone()).or_insert_with(|| vec![]);
        let next_entry = self
            .next_unique_id
            .entry(sym.name.to_owned())
            .or_insert_with(|| 0);
        if sym.id > *next_entry {
            *next_entry = sym.id
        } else {
            *next_entry += 1
        }
        stack_entry.push(*next_entry);
    }

    /// Remove a symbol from the stack.
    /// The input symbol should refer to the original symbol(before uniquifying).
    fn pop(&mut self, sym: &Symbol) -> Result<()> {
        let entry = self
            .stack
            .get_mut(sym)
            .ok_or_else(|| compile_err!("Pop undefined symbol {}", sym))?;
        entry
            .pop()
            .ok_or_else(|| compile_err!("Stack of symbol {} is empty", sym))?;
        Ok(())
    }

    /// Get the symbol in current scope for given symbol.
    /// The returned symbol has unique id within current scope.
    fn get(&mut self, sym: &Symbol) -> Result<Symbol> {
        let stack = self
            .stack
            .get(&sym)
            .ok_or_else(|| compile_err!("Undefined symbol {}", sym))?;
        let id = stack
            .last()
            .cloned()
            .ok_or_else(|| compile_err!("Stack of symbol {} is empty", sym))?;
        Ok(Symbol::new(&sym.name, sym.ty.clone(), id))
    }
}

#[cfg(test)]
mod tests {

    use crate::ast::*;

    #[test]
    fn test_uniquify() {
        let v2 = Var::merger(I32, BinOpType::Max);
        let v3 = Var::vector(vec![1, 2, 3]);
        let v4 = v2
            .pfor(v3.clone(), |b, _, e: Var<i32>| b.merge(e))
            .eval(I32);
        let v5 = Var::merger(I32, BinOpType::Min);
        let v6 = v5.pfor(v3, |b, _, e: Var<i32>| b.merge(e)).eval(I32);
        let mut v7: Expr = (v4 + v6).into();
        println!("{}", v7);
        super::uniquify(&mut v7).unwrap();
        println!("{}", v7);
    }
}
