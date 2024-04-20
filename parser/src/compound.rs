use nom::{
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{
    ast::{
        Arg, Args, ElseIfConditional, Expression, FunctionDefinition, IfConditional, IfExpression,
        TrailingElse, WhileExpression,
    },
    expressions::{expr, term_expr},
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
            par_delimited_comma_sep_exprs,
            many0(newline),
            term_expr,
        )),
        |(keyword, _, args, _, body)| {
            Expression::FunctionDef(FunctionDefinition::new(keyword, args, Box::new(body)))
        },
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
        tuple((if_token, lparen, term_expr, rparen, term_expr)),
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
    map(tuple((else_token, term_expr)), |(else_keyword, body)| {
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
                    Box::new(Expression::Literal(tokens[1])),
                    vec![],
                    Box::new(Expression::Literal(tokens[2]))
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
