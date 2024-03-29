use tokenizer::tokens::CommentedToken;

use crate::ast::Expression;

pub fn parse<'a, 'b: 'a>(
    tokens: &'b [&'a CommentedToken<'a>],
) -> Result<Vec<Expression<'a>>, String> {
    let mut expressions = vec![];
    let mut remaining_tokens = tokens;

    while !remaining_tokens.is_empty() {
        let (new_remaining_tokens, mut expr) = crate::program::program(remaining_tokens)
            .map_err(|err| format!("Could not parse: {:?}", err))?;
        expressions.append(&mut expr);
        remaining_tokens = new_remaining_tokens;
    }

    Ok(expressions)
}
