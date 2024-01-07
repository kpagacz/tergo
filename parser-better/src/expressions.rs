use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

use crate::ast::CommentedToken;
use crate::ast::Expression;
use crate::ast::TermExpr;
use crate::token_parsers::*;

fn symbol_expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    map(symbol, Expression::Symbol)(tokens)
}

fn literal_expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    map(literal, Expression::Literal)(tokens)
}

fn term_expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    alt((
        map(symbol_expr, |symbol| {
            Expression::Term(Box::new(symbol.into()))
        }),
        map(literal_expr, |literal| {
            Expression::Term(Box::new(literal.into()))
        }),
        map(
            tuple((
                lparen,
                delimited(many0(newline), term_expr, many0(newline)),
                rparen,
            )),
            |(lparen, term, rparen)| {
                Expression::Term(Box::new(TermExpr::new(Some(lparen), term, Some(rparen))))
            },
        ),
        map(
            tuple((
                lbrace,
                delimited(many0(newline), term_expr, many0(newline)),
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
// %left		OR OR2
// %left		AND AND2
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

// This implements the precedence climbing method described here:
// https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm#climbing
struct ExprParser {
    precedence: u8,
}

impl ExprParser {
    fn new(precedence: u8) -> Self {
        Self { precedence }
    }

    fn parse<'a, 'b: 'a>(
        &self,
        tokens: &'b [CommentedToken<'a>],
    ) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
        term_expr(tokens)
    }
}

fn expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    let parser = ExprParser::new(0);
    parser.parse(tokens)
}

#[cfg(test)]
mod tests {
    use crate::commented_tokens;

    use super::*;
    use tokenizer::{LocatedToken, Token::*};

    #[test]
    fn symbol_exprs() {
        let tokens = commented_tokens!(Symbol("a"));
        let res = symbol_expr(&tokens).unwrap().1;
        assert_eq!(res, Expression::Symbol(&tokens[0]));
    }

    #[test]
    fn literal_exprs() {
        let tokens = commented_tokens!(Literal("1"));
        let res = literal_expr(&tokens).unwrap().1;
        assert_eq!(res, Expression::Literal(&tokens[0]));
    }
}
