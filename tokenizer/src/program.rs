use nom::{
    branch::alt,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{ast::Expression, expression::expr_or_assign_or_help};

pub fn program(input: &str) -> IResult<&str, Vec<Box<Expression>>> {
    many0(alt((
        delimited(
            multispace0,
            expr_or_assign_or_help,
            tuple((
                multispace0,
                opt(nom::character::complete::char(';')),
                multispace0,
            )),
        ),
        map(multispace1, |_| Box::new(Expression::Expressions(vec![]))),
    )))(input)
}

#[cfg(test)]
mod tests {
    use crate::ast::Literal;

    use super::*;

    #[test]
    fn test_empty_lines_around_code() {
        let example = r#"
        TRUE
        "#;
        let expected = vec![Box::new(Expression::Literal(Literal::True))];
        assert_eq!(program(example), Ok(("", expected)));
    }

    #[test]
    fn just_empty_program() {
        let example = "";
        assert_eq!(program(example), Ok(("", vec![])));
    }

    #[test]
    fn test_two_expressions() {
        let example = r#"
        TRUE

        TRUE
        "#;
        let expected = vec![
            Box::new(Expression::Literal(Literal::True)),
            Box::new(Expression::Literal(Literal::True)),
        ];
        assert_eq!(program(example), Ok(("", expected)));
    }

    #[test]
    fn test_multiline_expression() {
        let input = r#"
        if 
        (FALSE) {} else 
        if (FALSE) {}
        "#;
        let expected = vec![Box::new(Expression::If(
            vec![
                (
                    Box::new(Expression::Literal(Literal::False)),
                    Box::new(Expression::Expressions(vec![])),
                ),
                (
                    Box::new(Expression::Literal(Literal::False)),
                    Box::new(Expression::Expressions(vec![])),
                ),
            ],
            None,
        ))];
        assert_eq!(program(input), Ok(("", expected)));
    }
}
