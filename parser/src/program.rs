use nom::{branch::alt, combinator::map, multi::many0, sequence::tuple, IResult};
use tokenizer::tokens::CommentedToken;

use crate::ast::Expression;
use crate::expressions::expr;
use crate::token_parsers::{eof, newline, semicolon};
use crate::whitespace::whitespace_or_comment;

fn statement_or_expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    alt((
        map(tuple((expr, alt((semicolon, newline)))), |(expr, _)| expr),
        map(whitespace_or_comment, Expression::Whitespace),
        map(eof, Expression::EOF),
    ))(tokens)
}

pub(crate) fn program<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Vec<Expression<'a>>> {
    many0(statement_or_expr)(tokens)
}

#[cfg(test)]
mod tests {
    use tokenizer::Token;

    use crate::helpers::commented_tokens;

    use super::*;

    #[test]
    fn program_parses_newline() {
        let tokens = commented_tokens![Token::Newline, Token::EOF];
        let res = program(&tokens).unwrap().1;

        assert_eq!(
            res,
            vec![
                Expression::Whitespace(&tokens[..1]),
                Expression::EOF(&tokens[1])
            ]
        );
    }

    #[test]
    fn parses_literal_ending_with_a_newline() {
        let tokens = commented_tokens![Token::Literal("7"), Token::Newline, Token::EOF];
        let res = program(&tokens).unwrap().1;
        assert_eq!(
            res,
            vec![Expression::Literal(&tokens[0]), Expression::EOF(&tokens[2])]
        );
    }

    #[test]
    fn parses_literal_ending_with_2_newlines() {
        let tokens = commented_tokens![
            Token::Literal("a"),
            Token::Newline,
            Token::Newline,
            Token::EOF
        ];
        let res = program(&tokens).unwrap().1;
        assert_eq!(
            res,
            vec![
                Expression::Literal(&tokens[0]),
                Expression::Whitespace(&tokens[2..3]),
                Expression::EOF(&tokens[3]),
            ]
        );
    }
}
