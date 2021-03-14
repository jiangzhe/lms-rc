use super::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType(pub(crate) Vec<Type>);
