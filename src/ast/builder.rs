use super::{BinOpKind, Expr, Type, TypeInference};

/// Appender appends input item to a list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppenderKind(Box<Type>);

/// A new Appender.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewAppender(Box<Expr>);

impl TypeInference for NewAppender {
    fn ty(&self) -> Type {
        todo!()
    }
}

/// Merger merges input into single value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MergerKind(Box<Type>, BinOpKind);

/// A new Merger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewMerger(Box<Expr>);

impl TypeInference for NewMerger {
    fn ty(&self) -> Type {
        todo!()
    }
}

/// DictMerger merge value by key into a dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictMergerKind(Box<Type>, Box<Type>, BinOpKind);

/// A new DictMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewDictMerger(Box<Expr>);

impl TypeInference for NewDictMerger {
    fn ty(&self) -> Type {
        todo!()
    }
}

/// GroupMerger append value into list group by key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupMergerKind(Box<Type>, Box<Type>);

/// A new GroupMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewGroupMerger(Box<Expr>);

impl TypeInference for NewGroupMerger {
    fn ty(&self) -> Type {
        todo!()
    }
}

/// VecMerger update vector in place by given index and item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecMergerKind(Box<Type>, BinOpKind);

/// A new VecMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewVecMerger(Box<Expr>);

impl TypeInference for NewVecMerger {
    fn ty(&self) -> Type {
        todo!()
    }
}
