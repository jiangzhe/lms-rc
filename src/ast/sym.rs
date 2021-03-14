use std::rc::Rc;

/// Symbol represents a named variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol {
    name: Rc<str>,
    id: i32,
}
