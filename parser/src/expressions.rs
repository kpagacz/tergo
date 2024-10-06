use log::trace;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use tokenizer::tokens::CommentedToken;
use tokenizer::tokens_buffer::TokensBuffer;
use tokenizer::Token::*;

use crate::ast::Args;
use crate::ast::Delimiter;
use crate::ast::Expression;
use crate::ast::FunctionCall;
use crate::ast::TermExpr;
use crate::compound::delimited_comma_sep_exprs;
use crate::compound::for_loop_expression;
use crate::compound::function_def;
use crate::compound::if_expression;
use crate::compound::lambda_function;
use crate::compound::repeat_expression;
use crate::compound::while_expression;
use crate::program::statement_or_expr;
use crate::token_parsers::*;
use crate::Input;

fn symbol_expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(symbol, Expression::Symbol)(tokens)
}

fn literal_expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(literal, Expression::Literal)(tokens)
}

pub(crate) fn term_expr<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("term_expr: {}", TokensBuffer(tokens));
    alt((
        for_loop_expression,
        while_expression,
        repeat_expression,
        function_def,
        lambda_function,
        if_expression,
        map(break_token, Expression::Break),
        map(continue_token, Expression::Continue),
        map(symbol_expr, |symbol| symbol),
        map(literal_expr, |literal| literal),
        map(
            tuple((
                lparen,
                delimited(many0(newline), opt(expr), many0(newline)),
                rparen,
            )),
            |(lparen, term, rparen)| {
                Expression::Term(Box::new(TermExpr::new(
                    Some(lparen),
                    term.map(|t| vec![t]).unwrap_or(vec![]),
                    Some(rparen),
                )))
            },
        ),
        map(
            tuple((
                lbrace,
                delimited(many0(newline), many0(statement_or_expr), many0(newline)),
                rbrace,
            )),
            |(lbrace, term, rbrace)| {
                Expression::Term(Box::new(TermExpr::new(Some(lbrace), term, Some(rbrace))))
            },
        ),
    ))(tokens)
}

#[derive(Debug)]
enum Tail<'a> {
    Call(Args<'a>),
    DoubleSubset(Args<'a>),
    SingleSubset(Args<'a>),
}

fn unary_op<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, &'b CommentedToken<'a>> {
    alt((minus, plus, unary_not, tilde, help))(tokens)
}

fn unary_term<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    alt((
        map(tuple((tilde, expr)), |(tilde, term)| {
            Expression::Formula(tilde, Box::new(term))
        }),
        map(tuple((unary_op, unary_term)), |(op, term)| {
            Expression::Unary(op, Box::new(term))
        }),
        atomic_term,
    ))(tokens)
}

pub(crate) fn atomic_term<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("atomic_term: {}", TokensBuffer(tokens));
    let (mut tokens, lhs) = term_expr(tokens)?;
    let mut acc = lhs;
    trace!("atomic_term: parsed LHS: {acc}");
    trace!("atomic_term: parsing rhs: {}", TokensBuffer(tokens));
    while let Ok((new_tokens, tail)) = alt((
        map(
            delimited_comma_sep_exprs(map(lparen, Delimiter::Paren), map(rparen, Delimiter::Paren)),
            Tail::Call,
        ),
        map(
            delimited_comma_sep_exprs(
                map(tuple((lbracket, lbracket)), Delimiter::DoubleBracket),
                map(tuple((rbracket, rbracket)), Delimiter::DoubleBracket),
            ),
            Tail::DoubleSubset,
        ),
        map(
            delimited_comma_sep_exprs(
                map(lbracket, Delimiter::SingleBracket),
                map(rbracket, Delimiter::SingleBracket),
            ),
            Tail::SingleSubset,
        ),
    ))(tokens)
    {
        trace!("atomic_term: parsed the rhs to this tail: {tail:?}");
        match tail {
            Tail::Call(args) => {
                acc = Expression::FunctionCall(FunctionCall {
                    function_ref: Box::new(acc),
                    args,
                })
            }
            Tail::DoubleSubset(args) | Tail::SingleSubset(args) => {
                acc = Expression::SubsetExpression(crate::ast::SubsetExpression {
                    object_ref: Box::new(acc),
                    args,
                })
            }
        }
        tokens = new_tokens;
    }

    trace!("atomic_term: final acc: {acc}");
    Ok((tokens, acc))
}

// Precedence table from https://github.com/SurajGupta/r-source/blob/master/src/main/gram.y
// /* This is the precedence table, low to high */
// %left		'?'
// %left		LOW WHILE FOR REPEAT
// %right		IF
// %left		ELSE
// %right		LEFT_ASSIGN
// %right		EQ_ASSIGN
// %left		RIGHT_ASSIGN
// %left		'~' TILDE
// %left		OR OR3
// %left		AND AND3
// %left		UNOT NOT
// %nonassoc   	GT GE LT LE EQ NE
// %left		'+' '-'
// %left		'*' '/'
// %left		SPECIAL
// %left		':'
// %left		UMINUS UPLUS
// %right		'^'
// %left		'$' '@'
// %left		NS_GET NS_GET_INT
// %nonassoc	'(' '[' LBB

#[derive(Debug, Clone, PartialEq)]
enum Associativity {
    Left,
    Right,
    Non,
}

fn associativity(token: &CommentedToken) -> Associativity {
    match &token.token {
        Help | RAssign | Tilde | Or | VectorizedOr | And | VectorizedAnd | NotEqual | Plus
        | Minus | Multiply | Divide | Colon | Dollar | Slot | NsGet | NsGetInt => {
            Associativity::Left
        }
        LAssign | OldAssign | Power => Associativity::Right,

        _ => Associativity::Non,
    }
}

fn precedence(token: &CommentedToken) -> u8 {
    match &token.token {
        Help => 1,
        LAssign => 5,
        OldAssign => 6,
        RAssign => 7,
        Tilde => 8,
        Or | VectorizedOr => 9,
        And | VectorizedAnd => 10,
        GreaterThan | GreaterEqual | LowerThan | LowerEqual | Equal | NotEqual => 12,
        Plus | Minus => 13,
        Multiply | Divide => 14,
        Special(_) => 15,
        Colon => 16,
        Power => 18,
        Dollar | Slot => 19,
        NsGet | NsGetInt => 20,
        _ => panic!("{token:?} is not a binary operator"),
    }
}

fn is_binary_operator(token: &CommentedToken) -> bool {
    matches!(
        &token.token,
        Help | RAssign
            | Tilde
            | Or
            | VectorizedOr
            | And
            | VectorizedAnd
            | NotEqual
            | GreaterThan
            | GreaterEqual
            | LowerThan
            | LowerEqual
            | Equal
            | Plus
            | Minus
            | Multiply
            | Divide
            | Colon
            | Dollar
            | Slot
            | NsGet
            | NsGetInt
            | LAssign
            | OldAssign
            | Power
            | Special(_)
    )
}

// This implements the precedence climbing method described here:
// https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm#climbing
struct ExprParser(u8);

impl ExprParser {
    fn parse<'a, 'b: 'a>(
        &self,
        mut lhs: Expression<'a>,
        mut tokens: Input<'a, 'b>,
    ) -> IResult<Input<'a, 'b>, Expression<'a>> {
        trace!("ExprParser::parse: {}", TokensBuffer(tokens));
        let mut lookahead = &tokens[0];
        while is_binary_operator(lookahead) && precedence(lookahead) >= self.0 {
            let op = lookahead;
            let mut it = 1;
            while matches!(tokens[it].token, Newline) {
                it += 1;
            }
            tokens = &tokens[it..];
            let (new_tokens, mut rhs) = unary_term(tokens)?;
            tokens = new_tokens;
            lookahead = &tokens[0];
            while is_binary_operator(lookahead)
                && (precedence(lookahead) > precedence(op)
                    || (associativity(lookahead) == Associativity::Right
                        && precedence(op) == precedence(lookahead)))
            {
                let q = precedence(op)
                    + (if precedence(lookahead) > precedence(op) {
                        1
                    } else {
                        0
                    });
                let parser = ExprParser(q);
                let (new_tokens, new_rhs) = parser.parse(rhs, tokens)?;
                rhs = new_rhs;
                tokens = new_tokens;
                lookahead = &tokens[0];
            }
            lhs = Expression::Bop(op, Box::new(lhs), Box::new(rhs));
        }
        trace!("ExprParser::parse: LHS {lhs}");
        trace!("ExprParser::parse: tokens left: {}", TokensBuffer(tokens));
        Ok((tokens, lhs))
    }
}

pub(crate) fn expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("expr: {}", TokensBuffer(tokens));
    let (tokens, term) = unary_term(tokens)?;
    if !tokens.is_empty() {
        let parser = ExprParser(0);
        parser.parse(term, tokens)
    } else {
        Ok((tokens, term))
    }
}

#[cfg(test)]
mod tests {
    use tokenizer::tokens::commented_tokens;

    use super::*;
    use tokenizer::Token::{self};

    fn binary_op_tokens() -> Vec<Token<'static>> {
        vec![
            Help,
            RAssign,
            Tilde,
            Or,
            VectorizedOr,
            And,
            VectorizedAnd,
            NotEqual,
            GreaterThan,
            GreaterEqual,
            LowerThan,
            LowerEqual,
            Equal,
            NotEqual,
            Plus,
            Minus,
            Multiply,
            Divide,
            Colon,
            Dollar,
            Slot,
            NsGet,
            NsGetInt,
            LAssign,
            OldAssign,
            Power,
            Special("%>%"),
        ]
    }

    #[test]
    fn symbol_exprs() {
        let tokens_ = commented_tokens!(Symbol("a"));
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = symbol_expr(&tokens).unwrap().1;
        assert_eq!(res, Expression::Symbol(tokens[0]));
    }

    #[test]
    fn literal_exprs() {
        let tokens_ = commented_tokens!(Literal("1"));
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = literal_expr(&tokens).unwrap().1;
        assert_eq!(res, Expression::Literal(tokens[0]));
    }

    #[test]
    fn expressions() {
        for token in binary_op_tokens() {
            let tokens_ = commented_tokens!(Literal("1"), token, Literal("1"), EOF);
            let tokens: Vec<_> = tokens_.iter().collect();
            let res = expr(&tokens).unwrap().1;
            assert_eq!(
                res,
                Expression::Bop(
                    tokens[1],
                    Box::new(Expression::Literal(tokens[0])),
                    Box::new(Expression::Literal(tokens[2]))
                )
            );
        }
    }

    #[test]
    fn right_associative_bop() {
        let tokens_ =
            commented_tokens!(Literal("1"), Power, Literal("2"), Power, Literal("3"), EOF);
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = expr(&tokens).unwrap().1;
        assert_eq!(
            res,
            Expression::Bop(
                tokens[1],
                Box::new(Expression::Literal(tokens[0])),
                Box::new(Expression::Bop(
                    tokens[3],
                    Box::new(Expression::Literal(tokens[2])),
                    Box::new(Expression::Literal(tokens[4]))
                ))
            )
        );
    }

    #[test]
    fn left_associative_bop() {
        let tokens_ = commented_tokens!(Literal("1"), Plus, Literal("2"), Plus, Literal("3"), EOF);
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = expr(&tokens).unwrap().1;
        assert_eq!(
            res,
            Expression::Bop(
                tokens[3],
                Box::new(Expression::Bop(
                    tokens[1],
                    Box::new(Expression::Literal(tokens[0])),
                    Box::new(Expression::Literal(tokens[2]))
                )),
                Box::new(Expression::Literal(tokens[4]))
            )
        );
    }

    #[test]
    fn bop_precedence() {
        let tokens_ = commented_tokens!(
            Literal("1"),
            Multiply,
            Literal("2"),
            Plus,
            Literal("3"),
            EOF
        );
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = expr(&tokens).unwrap().1;
        assert_eq!(
            res,
            Expression::Bop(
                tokens[3],
                Box::new(Expression::Bop(
                    tokens[1],
                    Box::new(Expression::Literal(tokens[0])),
                    Box::new(Expression::Literal(tokens[2]))
                )),
                Box::new(Expression::Literal(tokens[4]))
            )
        )
    }

    #[test]
    fn double_brace() {
        let tokens_ = commented_tokens!(LBrace, LBrace, Literal("1"), RBrace, RBrace);
        let tokens: Vec<_> = tokens_.iter().collect();
        let res = expr(&tokens).unwrap().1;
        assert_eq!(
            res,
            Expression::Term(Box::new(TermExpr {
                pre_delimiters: Some(tokens[0]),
                term: vec![Expression::Term(Box::new(TermExpr {
                    pre_delimiters: Some(tokens[1]),
                    term: vec![Expression::Literal(tokens[2])],
                    post_delimiters: Some(tokens[3])
                }))],
                post_delimiters: Some(tokens[4])
            }))
        )
    }
}
