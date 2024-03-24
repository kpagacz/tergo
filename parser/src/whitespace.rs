use nom::{
    error::{make_error, ErrorKind},
    IResult,
};
use tokenizer::Token;

use crate::ast::CommentedToken;

fn is_comment_or_newline(token: &CommentedToken) -> bool {
    matches!(token.token.token, Token::Comment(_) | Token::Newline)
}

pub(crate) fn whitespace_or_comment<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], &'b [CommentedToken<'a>]> {
    if tokens.is_empty() {
        return Err(nom::Err::Error(make_error(tokens, ErrorKind::Tag)));
    }
    match tokens.iter().position(|el| !is_comment_or_newline(el)) {
        Some(0) => Err(nom::Err::Error(make_error(tokens, ErrorKind::Tag))),
        Some(first_non_nl_non_comment) => Ok((
            &tokens[first_non_nl_non_comment..],
            &tokens[..first_non_nl_non_comment],
        )),
        None => Ok((&tokens[tokens.len()..], tokens)),
    }
}

#[cfg(test)]
mod tests {
    use tokenizer::{LocatedToken, Token};

    use super::*;
    use crate::helpers::{commented_tokens, located_tokens};

    #[test]
    fn parses_newline() {
        let located_tokens = located_tokens![Token::Newline, Token::EOF];
        let tokens = commented_tokens(&located_tokens);
        let res = whitespace_or_comment(&tokens).unwrap().1;
        assert_eq!(res, &tokens[..1]);

        let located_tokens = located_tokens![Token::Newline, Token::Newline, Token::EOF];
        let tokens = commented_tokens(&located_tokens);
        let res = whitespace_or_comment(&tokens).unwrap().1;
        assert_eq!(res, &tokens[..2]);
    }

    #[test]
    fn parses_comments() {
        let located_tokens = located_tokens![Token::Comment("hello"), Token::EOF];
        let tokens = commented_tokens(&located_tokens);
        let res = whitespace_or_comment(&tokens).unwrap().1;
        assert_eq!(res, &tokens[..1]);

        let located_tokens =
            located_tokens![Token::Comment("hello"), Token::Comment("world"), Token::EOF];
        let tokens = commented_tokens(&located_tokens);
        let res = whitespace_or_comment(&tokens).unwrap().1;
        assert_eq!(res, &tokens[..2]);
    }

    #[test]
    fn parses_mixed_comments_and_newlines() {
        let located_tokens = located_tokens![
            Token::Comment("hello"),
            Token::Newline,
            Token::Comment("world"),
            Token::EOF
        ];
        let tokens = commented_tokens(&located_tokens);
        let res = whitespace_or_comment(&tokens).unwrap().1;
        assert_eq!(res, &tokens[..3]);
    }

    #[test]
    fn does_not_parse_eof() {
        let located_tokens = located_tokens![Token::EOF];
        let tokens = commented_tokens(&located_tokens);
        let res = whitespace_or_comment(&tokens);
        assert!(res.is_err());
    }
}
