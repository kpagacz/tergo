use log::trace;
use nom::{
    IResult, Parser,
    branch::alt,
    combinator::{map, opt},
    multi::many0,
    sequence::delimited,
};
use tokenizer::{Token::*, tokens::CommentedToken};

use crate::Input;
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

pub(crate) fn symbol_expr<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(symbol, Expression::Symbol).parse(tokens)
}

fn literal_expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(literal, Expression::Literal).parse(tokens)
}

pub(crate) fn term_expr<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("term_expr: {}", &tokens);
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
            (
                lparen,
                delimited(many0(newline), opt(expr), many0(newline)),
                rparen,
            ),
            |(lparen, term, rparen)| {
                Expression::Term(Box::new(TermExpr::new(
                    Some(lparen),
                    term.map(|t| vec![t]).unwrap_or(vec![]),
                    Some(rparen),
                )))
            },
        ),
        map(
            (
                lbrace,
                delimited(many0(newline), many0(statement_or_expr), many0(newline)),
                rbrace,
            ),
            |(lbrace, term, rbrace)| {
                Expression::Term(Box::new(TermExpr::new(Some(lbrace), term, Some(rbrace))))
            },
        ),
    ))
    .parse(tokens)
}

#[derive(Debug)]
enum Tail<'a> {
    Call(Args<'a>),
    DoubleSubset(Args<'a>),
    SingleSubset(Args<'a>),
}

fn unary_op<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, &'b CommentedToken<'a>> {
    alt((minus, plus, unary_not, tilde, help)).parse(tokens)
}

pub(crate) fn unary_term<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    alt((
        map((tilde, expr), |(tilde, term)| {
            Expression::Formula(tilde, Box::new(term))
        }),
        map((unary_op, unary_term), |(op, term)| {
            Expression::Unary(op, Box::new(term))
        }),
        atomic_term,
    ))
    .parse(tokens)
}

pub(crate) fn atomic_term<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("atomic_term: {}", &tokens);
    let (mut tokens, lhs) = term_expr(tokens)?;
    let mut acc = lhs;
    trace!("atomic_term: parsed LHS: {acc}");
    trace!("atomic_term: parsing rhs: {}", &tokens);
    while let Ok((new_tokens, tail)) = alt((
        map(
            delimited_comma_sep_exprs(map(lparen, Delimiter::Paren), map(rparen, Delimiter::Paren)),
            Tail::Call,
        ),
        map(
            delimited_comma_sep_exprs(
                map((lbracket, lbracket), Delimiter::DoubleBracket),
                map((rbracket, rbracket), Delimiter::DoubleBracket),
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
    ))
    .parse(tokens.clone())
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
        | Minus | Multiply | Divide | Colon | Dollar | Slot | NsGet | NsGetInt | Modulo => {
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
        SuperAssign => 5,
        ColonAssign => 5,
        OldAssign => 6,
        RAssign => 7,
        Pipe => 8,
        Tilde => 8,
        Or | VectorizedOr => 9,
        And | VectorizedAnd => 10,
        GreaterThan | GreaterEqual | LowerThan | LowerEqual | Equal | NotEqual => 12,
        Plus | Minus => 13,
        Multiply | Divide => 14,
        Special(_) | Modulo => 15,
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
            | Modulo
            | Minus
            | Multiply
            | Divide
            | Colon
            | Dollar
            | Slot
            | NsGet
            | NsGetInt
            | LAssign
            | SuperAssign
            | ColonAssign
            | OldAssign
            | Power
            | Pipe
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
        let mut lookahead = &tokens[0];
        while is_binary_operator(lookahead) && precedence(lookahead) >= self.0 {
            let op = lookahead;
            let mut it = 1;
            while matches!(tokens[it].token, Newline) {
                it += 1;
            }
            tokens = Input(&tokens[it..]);
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
                let new_rhs = bop_to_multibop(new_rhs);
                rhs = new_rhs;
                tokens = new_tokens;
                lookahead = &tokens[0];
            }
            lhs = Expression::Bop(op, Box::new(lhs), Box::new(rhs));
        }
        Ok((tokens, lhs))
    }
}

pub(crate) fn expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("expr: {}", &tokens);
    let (tokens, term) = unary_term(tokens)?;
    if !tokens.is_empty() {
        let parser = ExprParser(0);
        let (tokens_left, xpr) = parser.parse(term, tokens)?;
        Ok((tokens_left, bop_to_multibop(xpr)))
    } else {
        Ok((tokens, term))
    }
}

pub(crate) fn expr_with_newlines<'a, 'b: 'a>(
    tokens: Input<'a, 'b>,
) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("expr_with_newlines: {}", &tokens);
    let (mut tokens, term) = unary_term(tokens)?;
    while !tokens.is_empty() && tokens[0].token == Newline {
        tokens = Input(&tokens[1..]);
    }
    if !tokens.is_empty() {
        let parser = ExprParser(0);
        let (tokens_left, xpr) = parser.parse(term, tokens)?;
        Ok((tokens_left, bop_to_multibop(xpr)))
    } else {
        Ok((tokens, term))
    }
}

fn bop_to_multibop(bop: Expression) -> Expression {
    match bop {
        Expression::Bop(op, lhs, rhs) => {
            let mut multibop = vec![(op, rhs)];
            let original_precedence = precedence(op);
            let mut lhs = lhs;
            while let Expression::Bop(op, lhs_, rhs) = *lhs {
                if original_precedence != precedence(op) {
                    lhs = Box::new(Expression::Bop(op, lhs_, rhs));
                    break;
                }
                multibop.push((op, rhs));
                lhs = lhs_;
            }
            multibop.reverse();
            Expression::MultiBop(lhs, multibop)
        }
        _ => bop,
    }
}
