use log::trace;
use nom::Parser;
use nom::combinator::opt;
use nom::{IResult, branch::alt, combinator::map};

use crate::Input;
use crate::ast::Expression;
use crate::expressions::expr;
use crate::token_parsers::{newline, semicolon};
use crate::whitespace::whitespace_or_comment;

pub(crate) fn statement_or_expr<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("statement_or_expr: {}", tokens);
    alt((
        map((expr, opt(alt((semicolon, newline)))), |(expr, _)| expr),
        map(whitespace_or_comment, Expression::Whitespace),
    ))
    .parse(tokens)
}
