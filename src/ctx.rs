use crate::def::{Def, DefEx};
use crate::exp::{Exp, Sym, ExpOrDef};
use crate::stmt::{Stmt, StmtEx};
use crate::block::Block;
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt::Debug;

/// Context of LMS
///
/// Other than the Scala implementation, we embed context
/// into each symbol registered to the context.
/// This design will lead to cyclic references with memory leak.
/// So we implements Drop trait for context to clear all stored
/// symbols.
#[derive(Debug, Clone)]
pub struct Context {
    sym_id: usize,
    pub(super) global_defs: Vec<Rc<dyn StmtEx>>,
    local_defs: Vec<Rc<dyn StmtEx>>,
    global_syms: HashMap<Sym, Rc<dyn StmtEx>>,
}

impl Context {

    /// Create a new context.
    pub fn new() -> Self {
        Context{
            sym_id: 0,
            global_defs: vec![], 
            local_defs: vec![], 
            global_syms: HashMap::new(), 
        }
    }

    /// Evaluate given function.
    ///
    /// All intermediate values in the function will
    /// be registered within context and returns
    /// the expression representing final result
    pub fn eval<T, F>(&mut self, f: F) -> Exp<T> 
    where
        F: WithCtx<Target=Exp<T>>,
        T: Debug + Clone + 'static,
    {
        f.with(self)
    }

    pub fn eval_if<T, TF, EF>(&mut self, cond: ExpOrDef<bool>, then_f: TF, else_f: EF) -> Exp<T> 
    where
        TF: WithCtx<Target=Exp<T>>,
        EF: WithCtx<Target=Exp<T>>,
        T: Debug + Clone + 'static,
    {
        if let ExpOrDef::Exp(Exp::Const(c)) = cond {
            // constant condition, optimize with one function
            if c {
                return self.eval(then_f);
            }
            return self.eval(else_f);
        }
        // reify / reflect / summarize / modify effects

        todo!()
    }

    /// dry run the given function with fresh input
    pub fn dry_run<T, U, F>(&mut self, f: F) -> Exp<U> 
    where
        F: FnOnce(&mut Context, Exp<T>) -> ExpOrDef<U>,
        T: Debug + Clone + PartialEq + 'static,
        U: Debug + Clone + PartialEq + 'static,
    {
        let s = self.fresh();
        let u = f(self, s);
        u.with(self)
    }

    #[inline]
    fn next_sym_id(&mut self) -> usize {
        let next = self.sym_id;
        self.sym_id += 1;
        next
    }

    #[inline]
    fn new_sym(&mut self) -> Sym {
        Sym(self.next_sym_id())
    }

    /// Create a new symbol
    pub fn fresh<T>(&mut self) -> Exp<T> 
    where
        T: 'static + Clone + Debug,
    {
        let sym = self.create_stmt::<T>(Def::Nil).lhs();
        Exp::Sym(sym)
    }

    /// find symbol by given definition
    pub fn find_stmt_by_def<T>(&self, d: &Def<T>) -> Option<Rc<dyn StmtEx>>
    where
        T: 'static + Clone + Debug + PartialEq,
    {
        for def in self.global_defs.iter() {
            if let Some(stmt) = def.as_any().downcast_ref::<Stmt<T>>() {
                if stmt.defines_rhs(d).is_some() {
                    return Some(def.clone());
                }
            }
        }
        None
    }

    /// find definition by given symbol index
    pub fn find_stmt_by_sym(&self, s: &Sym) -> Option<Rc<dyn StmtEx>> {
        self.global_syms.get(s).cloned()
    }

    /// to_atomic finds or creates symbol for input definition.
    ///
    /// this operation is to ensure atomicity of the definition.
    pub fn to_atomic<T>(&mut self, d: Def<T>) -> Exp<T>
    where
        T: 'static + Clone + Debug + PartialEq,
    {
        let stmt = self.find_or_create_stmt(d);
        Exp::Sym(stmt.lhs())
    }

    /// find by given definition, if not exists, create
    /// a new one and returns symbol index
    pub fn find_or_create_stmt<T>(&mut self, d: Def<T>) -> Rc<dyn StmtEx>
    where
        T: 'static + Clone + Debug + PartialEq,
    {
        self.find_stmt_by_def(&d)
            .unwrap_or_else(|| self.create_stmt(d))
    }

    fn create_stmt<T>(&mut self, d: Def<T>) -> Rc<dyn StmtEx>
    where
        T: 'static + Clone + Debug,
    {
        let s = self.new_sym();
        let a = Stmt::assign(s, d);
        let stmt = Rc::new(a);
        self.reflect_sub_graph(vec![stmt.clone()]);
        stmt
    }

    fn reify_sub_graph<T, B>(&mut self, b: B) -> (T, Vec<Rc<dyn StmtEx>>) 
    where
        B: FnOnce() -> T,
    {
        let save_global = self.global_defs.clone();
        let save_local = self.local_defs.clone();
        let save_syms = self.global_syms.clone();
        self.local_defs.clear();
        let r = b();
        let defs = std::mem::replace(&mut self.local_defs, save_local);
        self.global_defs = save_global;
        self.global_syms = save_syms;
        (r, defs)
    }

    fn reflect_sub_graph(&mut self, ds: Vec<Rc<dyn StmtEx>>) {
        // todo: allow multiple lhs in single statement
        let existing: Vec<_> = ds.iter()
            .map(|s| s.lhs())
            .flat_map(|s| self.global_syms.get(&s))
            .collect();
        assert!(existing.is_empty(), "symbol already defined");
        self.local_defs.extend_from_slice(&ds);
        self.global_defs.extend_from_slice(&ds);
        for stmt in ds {
            self.global_syms.insert(stmt.lhs(), stmt);
        }
    }

    fn reify_effects<T, B>(block: B, control_scope: bool) -> Block<T> 
    where
        B: FnOnce() -> Block<T>,
    {
        todo!()   
    }

    fn traverse_block<T>(block: Block<T>) {

    }

    // pub fn focus_sub_graph<T>(result: Vec<Exp<T>)

    fn build_schedule_for_result<T>(&self, result: Exp<T>, sort: bool) -> Vec<Rc<dyn StmtEx>> {
        self.get_schedule(result, sort)
    }

    /// identical to build_schedule_for_result
    fn get_schedule<T>(&self, result: Exp<T>, sort: bool) -> Vec<Rc<dyn StmtEx>> {
        let scope_index = self.build_scope_index();

        todo!()
    }

    fn build_scope_index(&self) -> HashMap<Sym, (Rc<dyn StmtEx>, usize)> {
        let mut cache = HashMap::new();
        for (idx, stmt) in self.global_defs.iter().enumerate() {
            let s = stmt.lhs();
            // remember the original order of the statements
            cache.insert(s, (stmt.clone(), idx));
        }
        cache
    }
}

/// Trait to enable convenient conversion
/// between ExpOrDef and Exp
pub trait WithCtx {
    type Target;

    fn with(self, ctx: &mut Context) -> Self::Target;
}

impl<T, F> WithCtx for F 
where
    F: FnOnce(&mut Context) -> ExpOrDef<T>,
    T: Debug + Clone + PartialEq + 'static,
{
    type Target = Exp<T>;

    fn with(self, ctx: &mut Context) -> Self::Target {
        let eod = self(ctx);
        match eod {
            ExpOrDef::Exp(e) => e,
            ExpOrDef::Def(d) => {
                let s = ctx.new_sym();
                let stmt = Stmt::assign(s, d);
                let stmt: Rc<dyn StmtEx> = Rc::new(stmt);
                ctx.reflect_sub_graph(vec![stmt.clone()]);
                Exp::Sym(stmt.lhs())
            }
        }
    }
}