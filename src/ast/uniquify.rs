use crate::ast::Symbol;
use crate::{Error, Result};
use std::collections::HashMap;

/// A trait that uniquifies symbol names in-place
pub trait Uniquify {
    fn uniquify(&mut self) -> Result<()>;
}

struct SymbolStack {
    stack: HashMap<Symbol, Vec<u32>>,
    next_unique_id: HashMap<String, u32>,
}

impl SymbolStack {
    fn new() -> Self {
        SymbolStack {
            stack: HashMap::new(),
            next_unique_id: HashMap::new(),
        }
    }

    /// Push a new symbol onto stack and assign a unique id.
    /// Original symbol should be kept for removal later.
    fn push(&mut self, sym: &Symbol) {
        let stack_entry = self.stack.entry(sym.clone()).or_insert_with(|| vec![]);
        let next_entry = self.next_unique_id.entry(sym.name.to_owned()).or_insert_with(|| 0);
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
        let stack = self.stack.get(&sym).ok_or_else(|| compile_err!("Undefined symbol {}", sym))?;
        let id = stack.last().cloned().ok_or_else(|| compile_err!("Stack of symbol {} is empty", sym))?;
        Ok(Symbol::new(&sym.name, sym.ty.clone(), id))
    }
}
