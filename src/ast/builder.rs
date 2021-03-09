use crate::ast::ty::Type;
use crate::ast::bin_op::BinOpKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuilderKind {
    Appender(Box<Type>),
    Merger(Box<Type>, BinOpKind),
    DictMerger(Box<Type>, Box<Type>, BinOpKind),
    GroupMerger(Box<Type>, Box<Type>),
    VecMerger(Box<Type>, BinOpKind),
}