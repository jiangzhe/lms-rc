use crate::Result;

/// ExprTransform defines transformation on type T.
pub trait Transformer<T> {
    
    /// Transform given expression
    fn transform(&mut self, expr: &mut T) -> Result<()>;
}
