use crate::ast::builder::BuilderKind;
use crate::ast::scalar::ScalarKind;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Scalar(ScalarKind),
    Vector(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Builder(BuilderKind),
    Struct(Vec<Type>),
    Unknown,
}

#[enum_dispatch]
pub trait TypeInference {
    fn ty(&self) -> Type;
}
