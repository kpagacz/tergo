use std::num::{NonZeroI8, NonZeroUsize};

use crate::ast::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, satisfy},
    combinator::{map, recognize},
    error::{self, Error, ErrorKind},
    multi::many0,
    sequence::{pair, tuple},
    Err, IResult, InputTakeAtPosition, Needed,
};

pub fn bop(input: &str) -> IResult<&str, Bop> {
    alt((
        map(tag("+"), |_| Bop::Plus),
        map(tag("-"), |_| Bop::Plus),
        map(tag("*"), |_| Bop::Plus),
        map(tag("/"), |_| Bop::Plus),
        map(tag("<-"), |_| Bop::Plus),
        map(tag("="), |_| Bop::Plus),
        map(tag("=="), |_| Bop::Plus),
        map(tag(">="), |_| Bop::Plus),
        map(tag("<="), |_| Bop::Plus),
        map(tag(">"), |_| Bop::Plus),
        map(tag("<"), |_| Bop::Plus),
    ))(input)
}

/// Parses an identifier of a variable
///
/// A variable can have a short name (like x and y) or a more descriptive name (age, carname, total_volume).
/// Rules for R variables are:
/// * A variable name must start with a letter and can be a combination of letters, digits, period(.)
/// and underscore(_). If it starts with period(.), it cannot be followed by a digit.
/// * A variable name cannot start with a number or underscore (_)
/// * Variable names are case-sensitive (age, Age and AGE are three different variables)
/// * Reserved words cannot be used as variables (TRUE, FALSE, NULL, if...)
///
pub fn identifier(input: &str) -> IResult<&str, Expression> {
    fn letter_digit_period_underscore(input: &str) -> IResult<&str, &str> {
        input.split_at_position_complete(|item| {
            !item.is_alphanumeric() && item != '.' && item != '_'
        })
    }
    map(
        recognize(alt((
            map(
                pair(
                    satisfy(|c| c.is_alphabetic()),
                    letter_digit_period_underscore,
                ),
                |(first, second)| format!("{first}{second}"),
            ),
            map(
                tuple((
                    tag("."),
                    satisfy(|c| c.is_alphabetic()),
                    letter_digit_period_underscore,
                )),
                |(first, second, third)| format!("{first}{second}{third}"),
            ),
        ))),
        |identifier| Expression::Identifier(identifier.to_owned()),
    )(input)
}

pub fn true_literal(input: &str) -> IResult<&str, Expression> {
    map(tag("TRUE"), |_| Expression::True)(input)
}

pub fn false_literal(input: &str) -> IResult<&str, Expression> {
    map(tag("FALSE"), |_| Expression::False)(input)
}

pub fn null_literal(input: &str) -> IResult<&str, Expression> {
    map(tag("NULL"), |_| Expression::Null)(input)
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use crate::{ast::Expression, expression::identifier, helpers::assert_parse_eq};

    #[test]
    fn test_identifier() {
        let valid_examples = ["Test", "t1", "l", ".something", ".s", "underscore_"];
        for example in valid_examples {
            assert_parse_eq(
                identifier(example),
                IResult::Ok(("", Expression::Identifier(example.to_owned()))),
            )
        }
    }
}
