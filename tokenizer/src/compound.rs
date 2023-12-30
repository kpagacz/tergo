use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::{
    bytes::complete::tag,
    sequence::{preceded, tuple},
    IResult,
};

use crate::ast::{CompoundStatement, Literal};
use crate::expression::condition;
use crate::{ast::Expression, expression::expr};

pub fn repeat(input: &str) -> IResult<&str, Box<Expression>> {
    map(
        preceded(tuple((tag("repeat"), multispace0)), expr),
        |repeat_expr| Box::new(Expression::Compound(CompoundStatement::Repeat(repeat_expr))),
    )(input)
}

pub fn while_stmt(input: &str) -> IResult<&str, Box<Expression>> {
    map(
        tuple((preceded(tag("while"), condition), expr)),
        |(cond, while_expr)| {
            Box::new(Expression::Compound(CompoundStatement::While(
                cond, while_expr,
            )))
        },
    )(input)
}

pub fn for_stmt(input: &str) -> IResult<&str, Box<Expression>> {
    map(tag("for"), |_| Box::new(Expression::Literal(Literal::Na)))(input)
}

mod tests {
    use super::{repeat, while_stmt, CompoundStatement, Expression, Literal};

    #[test]
    fn test_repeat() {
        let input = "repeat TRUE";
        let expected = Box::new(Expression::Compound(CompoundStatement::Repeat(Box::new(
            Expression::Literal(Literal::True),
        ))));
        assert_eq!(repeat(input), Ok(("", expected)));

        let input = r#"repeat
        {}"#;
        let expected = Box::new(Expression::Compound(CompoundStatement::Repeat(Box::new(
            Expression::Expressions(vec![]),
        ))));
        assert_eq!(repeat(input), Ok(("", expected)));
    }

    #[test]
    fn test_while() {
        let input = "while(TRUE)FALSE";
        let expected = Box::new(Expression::Compound(CompoundStatement::While(
            Box::new(Expression::Literal(Literal::True)),
            Box::new(Expression::Literal(Literal::False)),
        )));
        assert_eq!(while_stmt(input), Ok(("", expected)));

        let input_with_nl = r#"while
        (TRUE)
        FALSE"#;
        assert_eq!(while_stmt(input), while_stmt(input_with_nl));
    }
}
