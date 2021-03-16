use super::{BinOpType, Expr, Merge, Type, TypeInference};

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
    pub(super) item_ty: Box<Type>,
}

impl_from_for_type!(AppenderType, Type::Appender);

impl std::fmt::Display for AppenderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a<{}>", self.item_ty)
    }
}

/// A new Appender.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewAppender {
    pub(super) item_ty: Type,
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

impl std::fmt::Display for NewAppender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewAppender<{}>", self.item_ty)
    }
}

/// Merger merges input into single value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MergerType {
    pub(super) item_ty: Box<Type>,
    pub(super) op_ty: BinOpType,
}

impl_from_for_type!(MergerType, Type::Merger);

impl std::fmt::Display for MergerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m<{}, {}>", self.item_ty, self.op_ty)
    }
}

/// A new Merger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewMerger {
    pub(super) item_ty: Type,
    pub(super) op_ty: BinOpType,
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

impl std::fmt::Display for NewMerger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewMerger<{}, {}>", self.item_ty, self.op_ty)
    }
}

/// DictMerger merge value by key into a dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictMergerType {
    pub(super) key_ty: Box<Type>,
    pub(super) value_ty: Box<Type>,
    pub(super) op_ty: BinOpType,
}

impl_from_for_type!(DictMergerType, Type::DictMerger);

impl std::fmt::Display for DictMergerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dm<{}, {}, {}>", self.key_ty, self.value_ty, self.op_ty)
    }
}

/// A new DictMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewDictMerger {
    pub(super) key_ty: Type,
    pub(super) value_ty: Type,
    pub(super) op_ty: BinOpType,
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

impl std::fmt::Display for NewDictMerger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NewDictMerger<{}, {}, {}>",
            self.key_ty, self.value_ty, self.op_ty
        )
    }
}

/// GroupMerger append value into list group by key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupMergerType {
    pub(super) key_ty: Box<Type>,
    pub(super) value_ty: Box<Type>,
}

impl_from_for_type!(GroupMergerType, Type::GroupMerger);

impl std::fmt::Display for GroupMergerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gm<{}, {}>", self.key_ty, self.value_ty)
    }
}

/// A new GroupMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewGroupMerger {
    pub(super) key_ty: Type,
    pub(super) value_ty: Type,
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

impl std::fmt::Display for NewGroupMerger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewGroupMerger<{}, {}>", self.key_ty, self.value_ty)
    }
}

/// VecMerger update vector in place by given index and item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecMergerType {
    pub(super) item_ty: Box<Type>,
    pub(super) op_ty: BinOpType,
}

impl_from_for_type!(VecMergerType, Type::VecMerger);

impl std::fmt::Display for VecMergerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vm<{}, {}>", self.item_ty, self.op_ty)
    }
}

/// A new VecMerger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewVecMerger {
    pub(super) item_ty: Type,
    pub(super) op_ty: BinOpType,
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

impl std::fmt::Display for NewVecMerger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewVecMerger<{}, {}>", self.item_ty, self.op_ty)
    }
}
