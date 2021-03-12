use super::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleKind(Vec<Type>);
