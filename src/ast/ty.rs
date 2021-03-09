use crate::ast::scalar::ScalarKind;
use crate::ast::builder::BuilderKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Scalar(ScalarKind),
    Vector(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Builder(BuilderKind),
    Struct(Vec<Type>),
}
