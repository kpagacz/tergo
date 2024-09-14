use log::trace;
use nom::{
    combinator::{map, opt},
    error::Error,
    multi::many0,
    sequence::tuple,
    IResult, Parser,
};

use crate::{
    ast::{
        Arg, Args, Delimiter, ElseIfConditional, Expression, ForLoop, FunctionDefinition,
        IfConditional, IfExpression, Lambda, RepeatExpression, TrailingElse, WhileExpression,
    },
    expressions::expr,
    program::statement_or_expr,
    token_parsers::*,
    Input,
};

// Function definition
pub(crate) fn function_def<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        tuple((
            function,
            many0(newline),
            delimited_comma_sep_exprs(map(lparen, Delimiter::Paren), map(rparen, Delimiter::Paren)),
            many0(newline),
            expr,
        )),
        |(keyword, _, args, _, body)| {
            Expression::FunctionDef(FunctionDefinition::new(keyword, args, Box::new(body)))
        },
    )(tokens)
}

pub(crate) fn delimited_comma_sep_exprs<'a, P1, P2>(
    left_delimiter: P1,
    right_delimiter: P2,
) -> impl Parser<Input<'a, 'a>, Args<'a>, Error<Input<'a, 'a>>>
where
    P1: Parser<Input<'a, 'a>, Delimiter<'a>, Error<Input<'a, 'a>>>,
    P2: Parser<Input<'a, 'a>, Delimiter<'a>, Error<Input<'a, 'a>>>,
{
    map(
        tuple((
            left_delimiter,
            many0(newline),
            opt(expr),
            many0(tuple((
                tuple((comma, many0(newline))),
                tuple((expr, many0(newline))),
            ))),
            many0(newline),
            right_delimiter,
        )),
        |(ldelim, _, first_arg, comma_delimited_args, _, rdelim)| match first_arg {
            Some(xpr) => {
                let comma_delimited_args = comma_delimited_args
                    .into_iter()
                    .flat_map(|((sep, _), (xpr, _))| [Expression::Literal(sep), xpr]);
                let mut comma_delimited_args = std::iter::once(xpr).chain(comma_delimited_args);
                let mut args = vec![];
                while let Some(xpr) = comma_delimited_args.next() {
                    args.push(Arg(xpr, comma_delimited_args.next()));
                }
                trace!("delimited_comma_sep_exprs: parsed args {args:?}");
                Args::new(ldelim, args, rdelim)
            }
            None => Args::new(ldelim, vec![], rdelim),
        },
    )
}

// If expression
pub(crate) fn if_expression<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        tuple((if_conditional, many0(else_if), opt(trailing_else))),
        |(if_conditional, else_ifs, trailing_else)| {
            Expression::IfExpression(IfExpression {
                if_conditional,
                else_ifs,
                trailing_else,
            })
        },
    )(tokens)
}

fn if_conditional<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, IfConditional<'a>> {
    map(
        tuple((if_token, lparen, expr, rparen, expr)),
        |(keyword, left_delimiter, condition, right_delimiter, body)| IfConditional {
            keyword,
            left_delimiter,
            condition: Box::new(condition),
            right_delimiter,
            body: Box::new(body),
        },
    )(tokens)
}

fn else_if<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, ElseIfConditional<'a>> {
    map(
        tuple((else_token, if_conditional)),
        |(else_keyword, if_conditional)| ElseIfConditional {
            else_keyword,
            if_conditional,
        },
    )(tokens)
}

fn trailing_else<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, TrailingElse<'a>> {
    map(tuple((else_token, expr)), |(else_keyword, body)| {
        TrailingElse {
            else_keyword,
            body: Box::new(body),
        }
    })(tokens)
}

// While expression
pub(crate) fn while_expression<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        tuple((while_token, many0(newline), expr, many0(newline), expr)),
        |(while_keyword, _, condition, _, body)| {
            Expression::WhileExpression(WhileExpression {
                while_keyword,
                condition: Box::new(condition),
                body: Box::new(body),
            })
        },
    )(tokens)
}

// Repeat expression
pub(crate) fn repeat_expression<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        tuple((repeat, many0(newline), expr)),
        |(repeat_keyword, _, body)| {
            Expression::RepeatExpression(RepeatExpression {
                repeat_keyword,
                body: Box::new(body),
            })
        },
    )(tokens)
}

// For loops
pub(crate) fn for_loop_expression<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        tuple((
            for_token,
            many0(newline),
            map(lparen, Delimiter::Paren),
            many0(newline),
            expr,
            many0(newline),
            in_token,
            many0(newline),
            expr,
            many0(newline),
            map(rparen, Delimiter::Paren),
            many0(newline),
            expr,
        )),
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
    )(tokens)
}

// Lambda
pub(crate) fn lambda_function<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        tuple((
            lambda,
            many0(newline),
            delimited_comma_sep_exprs(map(lparen, Delimiter::Paren), map(rparen, Delimiter::Paren)),
            many0(newline),
            statement_or_expr,
        )),
        |(keyword, _, args, _, body)| {
            Expression::LambdaFunction(Lambda {
                keyword,
                args,
                body: Box::new(body),
            })
        },
    )(tokens)
}

#[cfg(test)]
mod tests {
    use tokenizer::tokens::commented_tokens;
    use tokenizer::tokens::CommentedToken;

    use crate::ast::TermExpr;

    use super::*;
    use tokenizer::Token::*;

    fn log_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn no_args_no_body_function_def() {
        log_init();
        let tokens_ = commented_tokens![Function, LParen, RParen, LBrace, RBrace, EOF];
        let tokens: Vec<_> = tokens_.iter().collect();
        let parsed = expr(&tokens).unwrap();
        let res = parsed.1;
        assert_eq!(
            res,
            Expression::FunctionDef(FunctionDefinition::new(
                tokens[0],
                Args::new(
                    Delimiter::Paren(tokens[1]),
                    vec![],
                    Delimiter::Paren(tokens[2])
                ),
                Box::new(Expression::Term(Box::new(TermExpr::new(
                    Some(tokens[3]),
                    vec![],
                    Some(tokens[4])
                ))))
            ))
        );

        // Because the eof is left
        assert_eq!(parsed.0.len(), 1);
    }
}
