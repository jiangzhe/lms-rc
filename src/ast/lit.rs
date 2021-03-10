// restrict numeric types
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum LiteralKind {
    BoolLiteral(bool),
    // I8Literal(i8),
    // I16Literal(i16),
    I32Literal(i32),
    I64Literal(i64),
    // U8Literal(u8),
    // U16Literal(u16),
    U32Literal(u32),
    U64Literal(u64),
    F32Literal(u32),
    F64Literal(u64),
    StringLiteral(String),
}

impl_from_for_lit_kind!(bool, LiteralKind::BoolLiteral);
impl_from_for_lit_kind!(i32, LiteralKind::I32Literal);
impl_from_for_lit_kind!(i64, LiteralKind::I64Literal);
impl_from_for_lit_kind!(u32, LiteralKind::U32Literal);
impl_from_for_lit_kind!(u64, LiteralKind::U64Literal);
impl_from_for_lit_kind!(String, LiteralKind::StringLiteral);

impl From<f32> for LiteralKind {
    fn from(src: f32) -> Self {
        LiteralKind::F32Literal(f32::to_bits(src))
    }
}

impl From<f64> for LiteralKind {
    fn from(src: f64) -> Self {
        LiteralKind::F64Literal(f64::to_bits(src))
    }
}    