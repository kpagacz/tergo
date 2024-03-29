use nom::{branch::alt, combinator::map, multi::many0, sequence::tuple, IResult};

use crate::ast::Expression;
use crate::expressions::expr;
use crate::token_parsers::{eof, newline, semicolon};
use crate::whitespace::whitespace_or_comment;
use crate::Input;

fn statement_or_expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    alt((
        map(tuple((expr, alt((semicolon, newline)))), |(expr, _)| expr),
        map(whitespace_or_comment, Expression::Whitespace),
        map(eof, Expression::EOF),
    ))(tokens)
}

pub(crate) fn program<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Vec<Expression<'a>>> {
    many0(statement_or_expr)(tokens)
}

#[cfg(test)]
mod tests {
    use tokenizer::tokens::commented_tokens;
    use tokenizer::tokens::CommentedToken;
    use tokenizer::Token;

    use super::*;

    #[test]
    fn program_parses_newline() {
        let tokens_ = commented_tokens![Token::Newline, Token::EOF];
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = program(&tokens).unwrap().1;

        assert_eq!(
            res,
            vec![
                Expression::Whitespace(&tokens[..1]),
                Expression::EOF(tokens[1])
            ]
        );
    }

    #[test]
    fn parses_literal_ending_with_a_newline() {
        let tokens_ = commented_tokens![Token::Literal("7"), Token::Newline, Token::EOF];
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = program(&tokens).unwrap().1;
        assert_eq!(
            res,
            vec![Expression::Literal(tokens[0]), Expression::EOF(tokens[2])]
        );
    }

    #[test]
    fn parses_literal_ending_with_2_newlines() {
        let tokens_ = commented_tokens![
            Token::Literal("a"),
            Token::Newline,
            Token::Newline,
            Token::EOF
        ];
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = program(&tokens).unwrap().1;
        assert_eq!(
            res,
            vec![
                Expression::Literal(tokens[0]),
                Expression::Whitespace(&tokens[2..3]),
                Expression::EOF(tokens[3]),
            ]
        );
    }
}
