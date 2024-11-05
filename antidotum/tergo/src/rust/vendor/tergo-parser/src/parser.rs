use log::trace;
use tokenizer::{tokens::CommentedToken, tokens_buffer::TokensBuffer};

use crate::ast::Expression;

pub fn parse<'a, 'b: 'a>(
    tokens: &'b [&'a CommentedToken<'a>],
) -> Result<Vec<Expression<'a>>, String> {
    let mut expressions = vec![];
    let mut remaining_tokens = tokens;

    while !remaining_tokens.is_empty() {
        trace!(
            "Main parse function, remaining tokens: {}",
            TokensBuffer(remaining_tokens)
        );
        let (new_remaining_tokens, mut expr) = crate::program::program(remaining_tokens)
            .map_err(|err| format!("Could not parse: {:?}", err))?;
        expressions.append(&mut expr);
        trace!("New remaining tokens: {}", TokensBuffer(remaining_tokens));
        remaining_tokens = new_remaining_tokens;
    }

    Ok(expressions)
}
