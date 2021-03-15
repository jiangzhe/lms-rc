use super::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType(pub(crate) Vec<Type>);

impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('(')?;
        for t in &self.0 {
            t.fmt(f)?;
        }
        f.write_char(')')
    }
}
