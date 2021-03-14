#[macro_use]
mod macros;
mod bin_op;
mod broadcast;
mod builder;
mod cast;
mod dict;
mod eval;
mod expr;
mod function;
mod get_field;
mod ifte;
mod iter;
mod lambda;
mod length;
mod lit;
mod lookup;
mod merge;
mod param;
mod pfor;
mod sym;
mod tuple;
mod ty;
mod unary_op;
mod var;
mod vector;

pub use bin_op::{BinOp, BinOpType};
pub use broadcast::Broadcast;
pub use builder::{
    AppenderType, Builder, DictMergerType, GroupMergerType, MergerType, NewAppender, NewDictMerger,
    NewGroupMerger, NewMerger, NewVecMerger, VecMergerType,
};
pub use cast::Cast;
pub use dict::{DictType, NewDict};
use enum_dispatch::enum_dispatch;
pub use eval::Eval;
pub use expr::Expr;
pub use function::FunctionType;
pub use get_field::GetField;
pub use ifte::IfThenElse;
pub use iter::Iter;
pub use lambda::Lambda;
pub use length::Length;
pub use lit::Literal;
pub use lookup::Lookup;
pub use merge::Merge;
pub use param::Parameter;
pub use pfor::For;
pub use sym::Symbol;
pub use tuple::TupleType;
pub use ty::Type;
pub use unary_op::{UnaryOp, UnaryOpType};
pub use var::Var;
pub use vector::{NewVector, VectorType};

/// Enable type inference on any expression.
///
/// The type inference is executed on the program staging,
/// before the code generation and compilation.
#[enum_dispatch]
pub trait TypeInference {
    fn ty(&self) -> Type;
}
