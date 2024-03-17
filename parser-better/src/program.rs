use crate::ast::CommentedToken;
use crate::ast::Expression;
use crate::expressions::expr;

pub fn program<'a, 'b: 'a>(tokens: &'b [CommentedToken<'a>]) -> Result<Expression<'a>, String> {
    match expr(tokens) {
        Ok((_, expr)) => Ok(expr),
        Err(e) => Err(e.to_string()),
    }
}
