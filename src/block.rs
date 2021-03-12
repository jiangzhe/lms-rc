use crate::exp::Exp;

#[derive(Debug, Clone)]
pub struct Block<T> {
    pub(super) res: Exp<T>,
}
