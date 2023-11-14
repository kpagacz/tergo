use crate::ast::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::none_of,
    combinator::{map, peek},
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};

use crate::expression::expression;

// library(library_name)
// Library names in R have some constraints but I don't care enough
// to check them here.
fn library(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            tag("library"),
            delimited(
                nom::character::complete::char('('),
                many1(none_of(")")),
                nom::character::complete::char(')'),
            ),
        )),
        |(_, lib_name)| Statement::Library(String::from_iter(lib_name.into_iter())),
    )(input)
}

fn break_stmt(input: &str) -> IResult<&str, Statement> {
    map(tag("break"), |_| Statement::Break)(input)
}

fn next_stmt(input: &str) -> IResult<&str, Statement> {
    map(tag("next"), |_| Statement::Next)(input)
}

pub(crate) fn statement(input: &str) -> IResult<&str, Statement> {
    println!("statement:{input}");
    peek(none_of("};"))(input)?;
    alt((
        break_stmt,
        library,
        map(many1(expression), |expressions| {
            Statement::Expressions(expressions.into_iter().map(|b| *b).collect())
        }),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library() {
        assert_eq!(
            library("library(something)"),
            Ok(("", Statement::Library(String::from("something"))))
        )
    }

    #[test]
    fn test_break() {
        assert_eq!(break_stmt("break"), Ok(("", Statement::Break)))
    }

    #[test]
    fn test_next() {
        assert_eq!(next_stmt("next"), Ok(("", Statement::Next)))
    }
}
