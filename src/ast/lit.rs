#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum LiteralKind {
    BoolLiteral(bool),
    I8Literal(i8),
    I16Literal(i16),
    I32Literal(i32),
    I64Literal(i64),
    U8Literal(u8),
    U16Literal(u16),
    U32Literal(u32),
    U64Literal(u64),
    F32Literal(u32),
    F64Literal(u64),
    StringLiteral(String),
}