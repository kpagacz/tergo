use log::trace;
use nom::{
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};
use tokenizer::tokens_buffer::TokensBuffer;

use crate::{
    ast::{Arg, Args, Expression, FunctionDefinition},
    expressions::expr,
    program::program,
    token_parsers::*,
    Input,
};

pub(crate) fn function_def<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(
        tuple((
            function,
            many0(newline),
            par_delimited_comma_sep_exprs,
            many0(newline),
            function_body,
        )),
        |(_, _, args, _, body)| Expression::FunctionDef(FunctionDefinition::new(args, body)),
    )(tokens)
}

fn par_delimited_comma_sep_exprs<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Args<'a>> {
    map(
        tuple((
            lparen,
            many0(newline),
            opt(expr),
            many0(tuple((
                tuple((comma, many0(newline))),
                tuple((expr, many0(newline))),
            ))),
            many0(newline),
            rparen,
        )),
        |(lpar, _, first_arg, comma_delimited_args, _, rpar)| match first_arg {
            Some(xpr) => {
                let comma_delimited_args = comma_delimited_args
                    .into_iter()
                    .flat_map(|((sep, _), (xpr, _))| [Expression::Literal(sep), xpr]);
                let mut comma_delimited_args = std::iter::once(xpr).chain(comma_delimited_args);
                let mut args = vec![];
                while let Some(xpr) = comma_delimited_args.next() {
                    args.push(Arg(xpr, comma_delimited_args.next()));
                }
                Args::new(
                    Box::new(Expression::Literal(lpar)),
                    args,
                    Box::new(Expression::Literal(rpar)),
                )
            }
            None => Args::new(
                Box::new(Expression::Literal(lpar)),
                vec![],
                Box::new(Expression::Literal(rpar)),
            ),
        },
    )(tokens)
}

fn function_body<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Vec<Expression<'a>>> {
    trace!("function_body: {}", TokensBuffer(tokens));
    program(tokens)
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
                Args::new(
                    Box::new(Expression::Literal(tokens[1])),
                    vec![],
                    Box::new(Expression::Literal(tokens[2]))
                ),
                vec![Expression::Term(Box::new(TermExpr::new(
                    Some(tokens[3]),
                    None,
                    Some(tokens[4])
                )))]
            ))
        );

        assert_eq!(parsed.0.len(), 0);
    }
}
