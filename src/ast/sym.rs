use std::rc::Rc;

/// Symbol represents a named variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol {
    pub(super) name: Rc<str>,
    pub(super) id: i32,
}

impl Symbol {
    pub fn unspecified() -> Self {
        Symbol {
            name: Rc::from("_"),
            id: 0,
        }
    }

    pub fn new(name: Rc<str>, id: i32) -> Self {
        Symbol { name, id }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
