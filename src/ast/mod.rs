//! AST(Abstract Syntax Tree) module defines all AST types.
//!
//! There is no front-end script parsing, the AST is designed to be
//! written directly using Rust.
//! There are serveral benifits:
//! 1. No parsing, better performance, fewer mistakes.
//!    Static type check can prevent user from invoking methods on
//!    wrong instance.
//! 2. High-level abstraction can be built easier, especially for
//!    specific domains like relational algebric.
//! 3. Staging can integrate some static data into the AST and
//!    build a simpler program to run.
#[macro_use]
mod macros;
mod bin_op;
mod broadcast;
mod builder;
mod cast;
mod dict;
mod eval;
mod expr;
mod get_field;
mod ifte;
mod iter;
mod lambda;
mod length;
mod lit;
mod lookup;
mod merge;
mod pfor;
mod transform;
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
pub use eval::Eval;
pub use expr::Expr;
pub use get_field::GetField;
pub use ifte::IfThenElse;
pub use iter::Iter;
pub use lambda::{Lambda, LambdaType};
pub use length::Length;
pub use lit::Literal;
pub use lookup::Lookup;
pub use merge::Merge;
pub use pfor::For;
pub use transform::Transformer;
pub use tuple::TupleType;
pub use ty::{Bool, BuilderType, Str, Type, TypeInference, F32, F64, I32, I64, U32, U64, U8};
pub use unary_op::{UnaryOp, UnaryOpType};
pub use var::Var;
pub use vector::{NewVector, VectorType};
