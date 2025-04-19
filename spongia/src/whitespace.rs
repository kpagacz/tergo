use log::trace;
use nom::{
    IResult,
    error::{ErrorKind, make_error},
};
use tokenizer::{Token, tokens::CommentedToken};

use crate::Input;

fn is_comment_or_newline(token: &CommentedToken) -> bool {
    matches!(token.token, Token::Comment(_) | Token::Newline)
}

pub(crate) fn whitespace_or_comment<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Input<'a, 'b>> {
    trace!("whitespace_or_comment: {}", tokens);
    if tokens.is_empty() {
        return Err(nom::Err::Error(make_error(tokens, ErrorKind::Tag)));
    }
    match tokens.iter().position(|el| !is_comment_or_newline(el)) {
        Some(0) => Err(nom::Err::Error(make_error(tokens, ErrorKind::Tag))),
        Some(first_non_nl_non_comment) => Ok((
            Input(&tokens[first_non_nl_non_comment..]),
            Input(&tokens[..first_non_nl_non_comment]),
        )),
        None => Ok((Input(&tokens[tokens.len()..]), tokens)),
    }
}
