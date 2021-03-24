use super::{Expr, Lambda};
use crate::Result;

/// ExprTransformer defines transformation on type Expr.
pub trait ExprTransformer {
    /// Transform given expression
    fn transform_expr(&mut self, expr: &mut Expr) -> Result<()>;

    /// Transform lambda
    ///
    /// Lambda differs from other expression is that it contains its own scoped symbols.
    /// The implementation should be in sync with transform_expr:
    /// transform_expr() should call transform_lambda if Lambda matched.
    fn transform_lambda(&mut self, lambda: &mut Lambda) -> Result<()>;
}

/// ExprVisitor defines visitor on Expr.
pub trait ExprVisitor {
    
    fn visit_expr(&mut self, expr: &Expr) -> Result<()>;

    fn visit_lambda(&mut self, lambda: &Lambda) -> Result<()>;
}
