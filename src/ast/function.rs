use super::Type;

/// Function with specified argument types and return type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    args: Vec<Type>,
    ret: Box<Type>,
}
