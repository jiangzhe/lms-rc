use super::{BinOpType, Expr, Merge, TupleType, Type, TypeInference};

/// Base trait of builder with mutable state
pub trait Builder {
    /// Merge given item and return a merge expression.
    fn merge<T>(self, item: T) -> Merge
    where
        T: Into<Expr>;

    /// Returns the result type of evaluating this builder.
    fn eval_type(&self) -> Type;
}

/// Appender appends input item to a list.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppenderType {
    pub(crate) item_ty: Box<Type>,
}

impl_from_for_type!(AppenderType, Type::Appender);

/// A new Appender.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewAppender {
    pub(crate) item_ty: Type,
}

impl TypeInference for NewAppender {
    fn ty(&self) -> Type {
        Type::Appender(AppenderType {
            item_ty: Box::new(self.item_ty.clone()),
        })
    }
}

impl Builder for NewAppender {
    fn merge<T>(self, item: T) -> Merge
    where
        T: Into<Expr>,
    {
        let value: Expr = item.into();
        assert!(
            self.item_ty == value.ty(),
            "Incompatible types[{:?} and {:?}] on merge operation",
            self.item_ty,
            value.ty()
        );
        Merge {
            builder: Box::new(Expr::NewAppender(self)),
            value: Box::new(value),
        }
    }

    fn eval_type(&self) -> Type {
        self.ty().eval()
    }
}

/// Merger merges input into single value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MergerType {
    pub(crate) item_ty: Box<Type>,
    pub(crate) op_ty: BinOpType,
}

impl_from_for_type!(MergerType, Type::Merger);

/// A new Merger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewMerger {
    pub(crate) item_ty: Type,
    pub(crate) op_ty: BinOpType,
}

impl TypeInference for NewMerger {
    fn ty(&self) -> Type {
        Type::Merger(MergerType {
            item_ty: Box::new(self.item_ty.clone()),
            op_ty: self.op_ty,
        })
    }
}

impl Builder for NewMerger {
    fn merge<T>(self, item: T) -> Merge
    where
        T: Into<Expr>,
    {
        let value: Expr = item.into();
        assert!(
            self.item_ty == value.ty(),
            "Incompatible types[{:?} and {:?}] on merge operation",
            self.item_ty,
            value.ty()
        );
        Merge {
            builder: Box::new(Expr::NewMerger(self)),
            value: Box::new(value),
        }
    }

    fn eval_type(&self) -> Type {
        self.ty().eval()
    }
}

/// DictMerger merge value by key into a dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictMergerType {
    pub(crate) key_ty: Box<Type>,
    pub(crate) value_ty: Box<Type>,
    pub(crate) op_ty: BinOpType,
}

impl_from_for_type!(DictMergerType, Type::DictMerger);

/// A new DictMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewDictMerger {
    pub(crate) key_ty: Type,
    pub(crate) value_ty: Type,
    pub(crate) op_ty: BinOpType,
}

impl TypeInference for NewDictMerger {
    fn ty(&self) -> Type {
        Type::DictMerger(DictMergerType {
            key_ty: Box::new(self.key_ty.clone()),
            value_ty: Box::new(self.value_ty.clone()),
            op_ty: self.op_ty,
        })
    }
}

impl Builder for NewDictMerger {
    fn merge<T>(self, item: T) -> Merge
    where
        T: Into<Expr>,
    {
        let value: Expr = item.into();
        let merge_ty = self.ty().merge();
        assert!(
            merge_ty == value.ty(),
            "Incompatible types[{:?} and {:?}] on merge operation",
            merge_ty,
            value.ty()
        );
        Merge {
            builder: Box::new(Expr::NewDictMerger(self)),
            value: Box::new(value),
        }
    }

    fn eval_type(&self) -> Type {
        self.ty().eval()
    }
}

/// GroupMerger append value into list group by key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupMergerType {
    pub(crate) key_ty: Box<Type>,
    pub(crate) value_ty: Box<Type>,
}

impl_from_for_type!(GroupMergerType, Type::GroupMerger);

/// A new GroupMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewGroupMerger {
    pub(crate) key_ty: Type,
    pub(crate) value_ty: Type,
}

impl TypeInference for NewGroupMerger {
    fn ty(&self) -> Type {
        Type::GroupMerger(GroupMergerType {
            key_ty: Box::new(self.key_ty.clone()),
            value_ty: Box::new(self.value_ty.clone()),
        })
    }
}

impl Builder for NewGroupMerger {
    fn merge<T>(self, item: T) -> Merge
    where
        T: Into<Expr>,
    {
        let value: Expr = item.into();
        let merge_ty = self.ty().merge();
        assert!(
            merge_ty == value.ty(),
            "Incompatible types[{:?} and {:?}] on merge operation",
            merge_ty,
            value.ty()
        );
        Merge {
            builder: Box::new(Expr::NewGroupMerger(self)),
            value: Box::new(value),
        }
    }

    fn eval_type(&self) -> Type {
        self.ty().eval()
    }
}

/// VecMerger update vector in place by given index and item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecMergerType {
    pub(crate) item_ty: Box<Type>,
    pub(crate) op_ty: BinOpType,
}

impl_from_for_type!(VecMergerType, Type::VecMerger);

/// A new VecMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewVecMerger {
    pub(crate) item_ty: Type,
    pub(crate) op_ty: BinOpType,
}

impl TypeInference for NewVecMerger {
    fn ty(&self) -> Type {
        Type::VecMerger(VecMergerType {
            item_ty: Box::new(self.item_ty.clone()),
            op_ty: self.op_ty,
        })
    }
}

impl Builder for NewVecMerger {
    fn merge<T>(self, item: T) -> Merge
    where
        T: Into<Expr>,
    {
        let value: Expr = item.into();
        let merge_ty = self.ty().merge();
        assert!(
            merge_ty == value.ty(),
            "Incompatible types[{:?} and {:?}] on merge operation",
            merge_ty,
            value.ty()
        );
        Merge {
            builder: Box::new(Expr::NewVecMerger(self)),
            value: Box::new(value),
        }
    }

    fn eval_type(&self) -> Type {
        self.ty().eval()
    }
}
