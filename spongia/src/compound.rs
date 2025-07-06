use log::trace;
use nom::{
    IResult, Parser,
    branch::alt,
    combinator::{map, opt},
    multi::many0,
};

use crate::{
    Input,
    ast::{
        Arg, Args, Delimiter, ElseIfConditional, Expression, ForLoop, FunctionDefinition,
        IfConditional, IfExpression, Lambda, RepeatExpression, TrailingElse, WhileExpression,
    },
    expressions::{expr, expr_with_newlines, unary_term_with_newlines},
    program::statement_or_expr,
    token_parsers::*,
};

// Function definition
pub(crate) fn function_def<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        (
            function,
            many0(newline),
            delimited_comma_sep_exprs(map(lparen, Delimiter::Paren), map(rparen, Delimiter::Paren)),
            many0(newline),
            expr,
        ),
        |(keyword, _, args, _, body)| {
            Expression::FunctionDef(FunctionDefinition::new(keyword, args, Box::new(body)))
        },
    )
    .parse(tokens)
}

pub(crate) fn delimited_comma_sep_exprs<'a, F, G>(
    left_delimiter: F,
    right_delimiter: G,
) -> impl Parser<Input<'a, 'a>, Error = nom::error::Error<Input<'a, 'a>>, Output = Args<'a>>
where
    F: Parser<Input<'a, 'a>, Error = nom::error::Error<Input<'a, 'a>>, Output = Delimiter<'a>>,
    G: Parser<Input<'a, 'a>, Error = nom::error::Error<Input<'a, 'a>>, Output = Delimiter<'a>>,
{
    map(
        (
            left_delimiter,
            many0(newline),
            args,
            many0(newline),
            right_delimiter,
        ),
        |(ldelim, _, mut args, _, rdelim)| {
            if !args.is_empty() && does_have_comma(args.last().unwrap()) {
                args.push(Arg::Proper(None, None));
            }
            trace!("delimited_comma_sep_exprs: parsed args {args:?}");
            Args::new(ldelim, args, rdelim)
        },
    )
}

fn does_have_comma(arg: &Arg) -> bool {
    match arg {
        Arg::Proper(_, comma) => comma.is_some(),
        Arg::EmptyEqual(_, _, comma) => comma.is_some(),
    }
}

pub(crate) fn args<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Vec<Arg<'a>>> {
    map(
        many0(alt((
            map(
                (
                    many0(newline),
                    expr_with_newlines,
                    many0(newline),
                    opt(comma),
                    many0(newline),
                ),
                |(_, expr, _, comma, _)| Arg::Proper(Some(expr), comma.map(Expression::Literal)),
            ),
            map(
                (
                    many0(newline),
                    unary_term_with_newlines,
                    many0(newline),
                    old_assign,
                    many0(newline),
                    opt(comma),
                    many0(newline),
                ),
                |(_, arg_name, _, equal_sign, _, comma, _)| {
                    Arg::EmptyEqual(arg_name, equal_sign, comma.map(Expression::Literal))
                },
            ),
            map((many0(newline), comma, many0(newline)), |(_, comma, _)| {
                Arg::Proper(None, Some(Expression::Literal(comma)))
            }),
        ))),
        |args| args,
    )
    .parse(tokens)
}

// If expression
pub(crate) fn if_expression<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("if_expression: parsing tokens: {}", &tokens);
    map(
        (if_conditional, many0(else_if), opt(trailing_else)),
        |(if_conditional, else_ifs, trailing_else)| {
            Expression::IfExpression(IfExpression {
                if_conditional,
                else_ifs,
                trailing_else,
            })
        },
    )
    .parse(tokens)
}

fn if_conditional<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, IfConditional<'a>> {
    trace!("if_conditional: parsing tokens: {}", &tokens);
    map(
        (
            if_token,
            lparen,
            many0(newline),
            expr_with_newlines,
            many0(newline),
            rparen,
            many0(newline),
            expr,
        ),
        |(keyword, left_delimiter, _, condition, _, right_delimiter, _, body)| IfConditional {
            keyword,
            left_delimiter,
            condition: Box::new(condition),
            right_delimiter,
            body: Box::new(body),
        },
    )
    .parse(tokens)
}

fn else_if<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, ElseIfConditional<'a>> {
    map(
        (else_token, if_conditional),
        |(else_keyword, if_conditional)| ElseIfConditional {
            else_keyword,
            if_conditional,
        },
    )
    .parse(tokens)
}

fn trailing_else<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, TrailingElse<'a>> {
    trace!("trailing_else: parsing tokens: {}", &tokens);
    map(
        (many0(newline), else_token, many0(newline), expr),
        |(_, else_keyword, _, body)| TrailingElse {
            else_keyword,
            body: Box::new(body),
        },
    )
    .parse(tokens)
}

// While expression
pub(crate) fn while_expression<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        (
            while_token,
            many0(newline),
            expr_with_newlines,
            many0(newline),
            expr,
        ),
        |(while_keyword, _, condition, _, body)| {
            Expression::WhileExpression(WhileExpression {
                while_keyword,
                condition: Box::new(condition),
                body: Box::new(body),
            })
        },
    )
    .parse(tokens)
}

// Repeat expression
pub(crate) fn repeat_expression<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        (repeat, many0(newline), expr),
        |(repeat_keyword, _, body)| {
            Expression::RepeatExpression(RepeatExpression {
                repeat_keyword,
                body: Box::new(body),
            })
        },
    )
    .parse(tokens)
}

// For loops
pub(crate) fn for_loop_expression<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        (
            for_token,
            many0(newline),
            map(lparen, Delimiter::Paren),
            many0(newline),
            expr_with_newlines,
            many0(newline),
            in_token,
            many0(newline),
            expr_with_newlines,
            many0(newline),
            map(rparen, Delimiter::Paren),
            many0(newline),
            expr,
        ),
        |(
            keyword,
            _,
            left_delim,
            _,
            identifier,
            _,
            in_keyword,
            _,
            collection,
            _,
            right_delim,
            _,
            body,
        )| {
            Expression::ForLoopExpression(ForLoop {
                keyword,
                left_delim,
                identifier: Box::new(identifier),
                in_keyword,
                collection: Box::new(collection),
                right_delim,
                body: Box::new(body),
            })
        },
    )
    .parse(tokens)
}

// Lambda
pub(crate) fn lambda_function<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        (
            lambda,
            many0(newline),
            delimited_comma_sep_exprs(map(lparen, Delimiter::Paren), map(rparen, Delimiter::Paren)),
            many0(newline),
            statement_or_expr,
        ),
        |(keyword, _, args, _, body)| {
            Expression::LambdaFunction(Lambda {
                keyword,
                args,
                body: Box::new(body),
            })
        },
    )
    .parse(tokens)
}
