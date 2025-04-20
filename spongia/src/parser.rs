use log::{debug, trace};
use tokenizer::Token;

use crate::{Input, ast::Expression};

pub fn parse<'a, 'b: 'a>(mut tokens: Input<'a, 'b>) -> Result<Vec<Expression<'a>>, String> {
    let mut expressions = vec![];

    while !tokens.is_empty() && !matches!(tokens.first().unwrap().token, Token::EOF) {
        trace!("Main parse function, remaining tokens: {}", &tokens);
        let (new_remaining_tokens, expr) = crate::program::statement_or_expr(tokens)
            .map_err(|err| format!("Could not parse: {:?}", err))?;
        expressions.push(expr);
        tokens = new_remaining_tokens;
        debug!("Remaining tokens length: {}", &tokens.len());
        debug!("Current expressions length: {}", expressions.len());
        trace!("New remaining tokens: {}", &tokens);
    }
    expressions.push(Expression::EOF(tokens[0]));

    Ok(expressions)
}
