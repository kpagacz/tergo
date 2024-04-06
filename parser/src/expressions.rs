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

use crate::ast::Expression;
use crate::ast::TermExpr;
use crate::compound::function_def;
use crate::token_parsers::*;
use crate::Input;

fn symbol_expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(symbol, Expression::Symbol)(tokens)
}

fn literal_expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    map(literal, Expression::Literal)(tokens)
}

fn term_expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("term_expr: {}", TokensBuffer(tokens));
    alt((
        function_def,
        map(symbol_expr, |symbol| symbol),
        map(literal_expr, |literal| literal),
        map(
            tuple((
                lparen,
                delimited(many0(newline), opt(expr), many0(newline)),
                rparen,
            )),
            |(lparen, term, rparen)| {
                Expression::Term(Box::new(TermExpr::new(Some(lparen), term, Some(rparen))))
            },
        ),
        map(
            tuple((
                lbrace,
                delimited(many0(newline), opt(expr), many0(newline)),
                rbrace,
            )),
            |(lbrace, term, rbrace)| {
                Expression::Term(Box::new(TermExpr::new(Some(lbrace), term, Some(rbrace))))
            },
        ),
    ))(tokens)
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
        let mut lookahead = &tokens[0];
        while is_binary_operator(lookahead) && precedence(lookahead) >= self.0 {
            let op = lookahead;
            tokens = &tokens[1..];
            let (new_tokens, mut rhs) = term_expr(tokens)?;
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
        Ok((tokens, lhs))
    }
}

pub(crate) fn expr<'a, 'b: 'a>(tokens: Input<'a, 'b>) -> IResult<Input<'a, 'b>, Expression<'a>> {
    trace!("expr: {}", TokensBuffer(tokens));
    let (tokens, term) = term_expr(tokens)?;
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
}
