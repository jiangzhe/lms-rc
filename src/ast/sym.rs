use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol {
    name: Rc<str>,
    id: i32,
}
