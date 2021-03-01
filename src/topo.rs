use crate::stmt::{Stmt, StmtEx};
use crate::exp::Sym;
use std::rc::Rc;
use std::collections::{HashSet, HashMap};

pub fn build_topo(defs: &[Rc<dyn StmtEx>], result: Sym) -> Vec<Sym> {
    let lookup = build_lookup(defs);
    let mut processed = HashSet::new();
    let mut topo = vec![];
    build_topo_rec(&lookup, &mut processed, &mut topo, result);
    topo
}

fn build_topo_rec(
    lookup: &HashMap<Sym, Rc<dyn StmtEx>>, 
    processed: &mut HashSet<Sym>, 
    topo: &mut Vec<Sym>,
    sym: Sym,
) {
    if processed.contains(&sym) {
        // may be already processed as dependency of other symbols,
        // just skip
        return;
    }
    let ros = &lookup[&sym];
    let mut dep_syms = ros.rhs().syms();
    if dep_syms.is_empty() {
        // no dependency
        processed.insert(sym.clone());
        topo.push(sym);
        return;
    }

    // process each dependency
    dep_syms.sort_by(|l, r| l.0.cmp(&r.0));
    for ds in dep_syms {
        build_topo_rec(lookup, processed, topo, ds);
    }
    processed.insert(sym.clone());
    topo.push(sym);
}

/// build dependency lookup map from given statement list
fn build_lookup(defs: &[Rc<dyn StmtEx>]) -> HashMap<Sym, Rc<dyn StmtEx>> {
    let mut index = HashMap::new();
    for stmt in defs {
        let s = stmt.lhs();
        // remember the original order of the statements
        index.insert(s, stmt.clone());
    }
    index
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ctx::Context;
    use crate::exp::Exp;

    #[test]
    fn test_build_topo() {
        let mut ctx = Context::new();
        let input = ctx.fresh();
        let result = pow(input, 2, &mut ctx);
        println!("global defs:");
        for d in &ctx.global_defs {
            println!("{:?}", d);
        }
        let topo = build_topo(&ctx.global_defs, result.as_sym().cloned().unwrap());
        println!("topo:");
        for s in &topo {
            println!("{:?}", s);
        }
    }

    fn pow(input: Exp<usize>, n: u32, ctx: &mut Context) -> Exp<usize> {
        if n == 0 {
            Exp::Const(1)
        } else if n == 1 {
            input
        } else {
            let v = pow(input.clone(), n-1, ctx);
            ctx.eval(input * v)
        }
    }
}