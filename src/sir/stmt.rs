use crate::ast::*;
use crate::sym::Symbol;
use crate::sir::iter::StmtIter;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Stmt {
    sym: Option<Symbol>,
    expr: StmtExpr,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum StmtExpr {
    Assign(Symbol),
    /// Broadcasts a scalar into a vector.
    Broadcast(Symbol),
    /// Applies a binary operator to child expressions.
    BinOp {
        op_ty: BinOpType,
        left: Symbol,
        right: Symbol,
    },
    /// Applies a unary operator to child expressions.
    UnaryOp {
        op_ty: UnaryOpType,
        value: Symbol,
    },
    /// Cast a scalar expression to another type.
    Cast {
        ty: Type,
        value: Symbol,
    },
    /// Access a tuple field at given index.
    GetField {
        value: Symbol,
        index: u32,
    },
    /// Get the length of a vector as an u64.
    Length(Symbol),
    /// Lookup a value in Dict.
    Lookup {
        value: Symbol,
        index: Symbol,
    },
    /// Evaluate different branch based on condition.
    IfThenElse {
        i: Symbol,
        t: Symbol,
        e: Symbol,
    },
    /// Update a builder in parallel by itearating over data.
    For {
        data: Vec<StmtIter>,
        start: Option<Symbol>,
        end: Option<Symbol>,
    },
    /// Update a builder value, returning a new builder.
    Merge {
        builder: Symbol,
        value: Symbol,
    },
    /// An expression representing a function.
    // Lambda(Lambda),
    /// Construct a new vector.
    NewVector(Vec<Symbol>),
    /// Construct a new dictionary.
    // NewDict(NewDict),
    /// Construct a new appender.
    NewAppender {
        item_ty: Type,
    },
    /// Construct a new merger.
    NewMerger {
        item_ty: Type,
        op_ty: BinOpType,
    },
    /// Construct a new dictmerger.
    NewDictMerger {
        key_ty: Type,
        value_ty: Type,
        op_ty: BinOpType,
    },
    /// Construct a new groupmerger.
    NewGroupMerger {
        key_ty: Type,
        value_ty: Type,
    },
    /// Construct a new vecmerger.
    NewVecMerger {
        item_ty: Type,
        op_ty: BinOpType,
    },
    /// Consume a builder and return its result
    Eval(Symbol),
}
