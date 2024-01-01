use nom::{
    branch::alt,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{ast::Expression, expression::expr_or_assign_or_help, helpers::CodeSpan};

pub fn program(input: CodeSpan) -> IResult<CodeSpan, Vec<Box<Expression>>> {
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
    fn test_program() {
        let tests = [
            // surrounding empty lines
            (
                r#"
        TRUE
        "#,
                vec![Box::new(Expression::Literal(Literal::True))],
            ),
            // empty program
            ("", vec![]),
            // multiple lines
            (
                r#"
        TRUE

        TRUE
        "#,
                vec![
                    Box::new(Expression::Literal(Literal::True)),
                    Box::new(Expression::Literal(Literal::True)),
                ],
            ),
            // multiline expression
            (
                r#"
        if 
        (FALSE) {} else 
        if (FALSE) {}
        "#,
                vec![Box::new(Expression::If(
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
                ))],
            ),
        ];
        for (input, expected) in tests {
            let input = CodeSpan::new(input);
            assert_eq!(program(input).unwrap().1, expected);
        }
    }
}
