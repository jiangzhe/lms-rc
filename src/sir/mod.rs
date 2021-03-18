mod iter;
mod stmt;

use std::collections::HashMap;

pub(self) struct BasicBlockId(usize);

pub(self) struct FunctionId(usize);

pub(self) struct ProgramSite(FunctionId, BasicBlockId);

// pub(self) type SiteSymbolMap = HashMap<StmtExpr, Symbol>;
