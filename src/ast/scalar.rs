#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarKind {
    Bool,
    U8,
    // I8,
    // I16,
    I32,
    I64,
    // U16,
    U32,
    U64,
    F32,
    F64,
    String,
}
