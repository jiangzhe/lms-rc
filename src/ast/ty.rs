use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// A scalar.
    Scalar(ScalarKind),
    /// A variable-length vector.
    Vector(VectorKind),
    /// A dictionary mapping keys to values.
    Dict(DictKind),
    /// An ordered tuple.
    Tuple(TupleKind),
    /// A function with a list of arguments and return type.
    Function(FunctionKind),
    /// A mutable builder to append item to a list.
    Appender(AppenderKind),
    /// A builder that constructs a scalar.
    Merger(MergerKind),
    /// A builder that creates a dictionary.
    DictMerger(DictMergerKind),
    /// A builder that creates groups.
    GroupMerger(GroupMergerKind),
    /// A builder that constructs a vector by updating elements.
    VecMerger(VecMergerKind),
    /// An unknown type, used only before type inference.
    Unknown,
}

impl Type {
    pub fn scalar(&self) -> Option<ScalarKind> {
        match self {
            Type::Scalar(sk) => Some(sk.clone()),
            _ => None,
        }
    }

    pub fn vector(&self) -> Option<VectorKind> {
        match self {
            Type::Vector(vk) => Some(vk.clone()),
            _ => None,
        }
    }

    pub fn dict(&self) -> Option<DictKind> {
        match self {
            Type::Dict(dk) => Some(dk.clone()),
            _ => None,
        }
    }

    pub fn appender(&self) -> Option<AppenderKind> {
        match self {
            Type::Appender(ak) => Some(ak.clone()),
            _ => None,
        }
    }

    pub fn merger(&self) -> Option<MergerKind> {
        match self {
            Type::Merger(mk) => Some(mk.clone()),
            _ => None,
        }
    }

    pub fn dictmerger(&self) -> Option<DictMergerKind> {
        match self {
            Type::DictMerger(dmk) => Some(dmk.clone()),
            _ => None,
        }
    }

    pub fn groupmerger(&self) -> Option<GroupMergerKind> {
        match self {
            Type::GroupMerger(gmk) => Some(gmk.clone()),
            _ => None,
        }
    }

    pub fn vecmerger(&self) -> Option<VecMergerKind> {
        match self {
            Type::VecMerger(vmk) => Some(vmk.clone()),
            _ => None,
        }
    }

    pub fn tuple(&self) -> Option<TupleKind> {
        match self {
            Type::Tuple(tk) => Some(tk.clone()),
            _ => None,
        }
    }

    pub fn function(&self) -> Option<FunctionKind> {
        match self {
            Type::Function(fk) => Some(fk.clone()),
            _ => None,
        }
    }

    pub fn unknown(&self) -> bool {
        match self {
            Type::Unknown => true,
            _ => false,
        }
    }
}
