use crate::ast::*;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{multispace0, none_of, one_of, satisfy, space0},
    combinator::{map, not, recognize},
    error::ParseError,
    multi::many0,
    sequence::{delimited, pair, tuple},
    AsChar, Compare, FindToken, IResult, InputIter, InputTake, InputTakeAtPosition, Parser,
};

// TODO: Add support for vectorized operators
// && and ||
pub fn bop(input: &str) -> IResult<&str, Bop> {
    alt((
        map(tag("+"), |_| Bop::Plus),
        map(tag("-"), |_| Bop::Minus),
        map(tag("*"), |_| Bop::Multiply),
        map(tag("/"), |_| Bop::Divide),
        map(tag("%%"), |_| Bop::Modulo),
        map(tag("^"), |_| Bop::Power),
        map(tag(">"), |_| Bop::Greater),
        map(tag(">="), |_| Bop::Ge),
        map(tag("<"), |_| Bop::Lower),
        map(tag("<="), |_| Bop::Le),
        map(tag("=="), |_| Bop::Equal),
        map(tag("!="), |_| Bop::NotEqual),
        map(tag("&"), |_| Bop::And),
        map(tag("|"), |_| Bop::Or),
        map(tag("~"), |_| Bop::ModelFormulae),
        // map(tag("<-"), |_| Bop::Assignment),
        // map(tag("->"), |_| Bop::RightAssignment),
        // map(tag("="), |_| Bop::OldAssignment),
        map(tag("$"), |_| Bop::Dollar),
        map(tag(":"), |_| Bop::Colon),
    ))(input)
}

pub fn uop(input: &str) -> IResult<&str, Uop> {
    alt((
        map(tag("+"), |_| Uop::Plus),
        map(tag("-"), |_| Uop::Minus),
        map(tag("!"), |_| Uop::Not),
    ))(input)
}

pub fn true_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("TRUE"), |_| Literal::True)(input)
}

pub fn false_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("FALSE"), |_| Literal::False)(input)
}

pub fn null_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("NULL"), |_| Literal::Null)(input)
}

pub fn na_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("NA"), |_| Literal::Na)(input)
}

pub fn nan_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("NaN"), |_| Literal::NaN)(input)
}

pub fn inf_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("Inf"), |_| Literal::Inf)(input)
}

// TODO: support line breaks in the middle of string literals
// I don't even know how R behaves if there are line breaks in the middle
// of a string literal
pub fn string_literal(input: &str) -> IResult<&str, Literal> {
    fn parse_delimited_string<'a, E: ParseError<&'a str>>(
        delimited_by: &'a str,
        string_chars: &'a str,
    ) -> impl Parser<&'a str, &'a str, E> {
        delimited(
            tag(delimited_by),
            escaped(none_of(string_chars), '\\', one_of("\\nrtbafvxuU'\"")),
            tag(delimited_by),
        )
    }

    map(
        alt((
            parse_delimited_string("\"", "\\\""),
            parse_delimited_string("\'", "\\\'"),
        )),
        |s: &str| Literal::String(s.to_owned()),
    )(input)
}

pub fn literal(input: &str) -> IResult<&str, Expression> {
    map(
        alt((
            true_literal,
            false_literal,
            null_literal,
            na_literal,
            nan_literal,
            inf_literal,
        )),
        |literal| Expression::Literal(literal),
    )(input)
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
    // TODO: disallow reserved keywords
    // The following identifiers have a special meaning and cannot be used for object names
    // if else repeat while function for in next break
    // TRUE FALSE NULL Inf NaN
    // NA NA_integer_ NA_real_ NA_complex_ NA_character_
    // ... ..1 ..2 etc.
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

pub fn call(input: &str) -> IResult<&str, Expression> {
    todo!()
}

pub fn uop_expr(input: &str) -> IResult<&str, Expression> {
    map(tuple((uop, multispace0, expression)), |(uop, _, expr)| {
        Expression::Uop(uop, Box::new(expr))
    })(input)
}

pub fn bop_expr(input: &str) -> IResult<&str, Expression> {
    map(
        tuple((expression, space0, bop, multispace0, expression)),
        |res| Expression::Bop(res.2, Box::new(res.0), Box::new(res.4)),
    )(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    alt((literal, identifier, call, bop_expr))(input)
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use crate::{
        ast::{Expression, Literal},
        expression::{bop_expr, identifier},
        helpers::assert_parse_eq,
    };

    use super::string_literal;

    #[test]
    fn test_string_literal() {
        // " delimited
        assert_parse_eq(
            string_literal("\"Something\""),
            IResult::Ok(("", Literal::String(String::from("Something")))),
        );
        assert_parse_eq(
            string_literal("\"\\\\\\\"\""), // Printed as: "\\\""
            IResult::Ok(("", Literal::String(String::from("\\\\\\\"")))),
        );
        assert_parse_eq(
            string_literal("\"apostro\'phe\""),
            IResult::Ok(("", Literal::String(String::from("apostro\'phe")))),
        );
        assert_parse_eq(
            string_literal("\"Something\" test"),
            IResult::Ok((" test", Literal::String(String::from("Something")))),
        );

        // ' delimited
        assert_parse_eq(
            string_literal("\"Something\""),
            IResult::Ok(("", Literal::String(String::from("Something")))),
        );
        assert_parse_eq(
            string_literal("\"\\\\\\\"\""), // Printed as: "\\\""
            IResult::Ok(("", Literal::String(String::from("\\\\\\\"")))),
        );
        assert_parse_eq(
            string_literal("\'apostro\"phe\'"),
            IResult::Ok(("", Literal::String(String::from("apostro\"phe")))),
        );
        assert_parse_eq(
            string_literal("\"Something\" test"),
            IResult::Ok((" test", Literal::String(String::from("Something")))),
        );
    }

    #[test]
    fn test_identifier() {
        let valid_examples = ["Test", "t1", "l", ".something", ".s", "underscore_"];
        for example in valid_examples {
            assert_parse_eq(
                identifier(example),
                IResult::Ok(("", Expression::Identifier(example.to_owned()))),
            )
        }
        let invalid_examples = [".3", "_something", "123"];
        for example in invalid_examples {
            assert!(identifier(example).is_err())
        }
    }

    #[test]
    fn test_uop_expr() {
        // TODO: write tests for these
        // let valid_examples = ["+5", "-5", "!2", "!FALSE", "!\n\nTRUE", "+-5"];
        // let expected = [5, 5, 2, Literal::False, Literal::True, Expression::Uop(Uop::Minus, Box::new())]
        // for example in valid_examples {
        //     assert_parse_eq(
        //         identifier(example),
        //         IResult::Ok(("", Expression::Identifier(example.to_owned()))),
        //     )
        // }
        // let invalid_examples = [".3", "_something", "123"];
        // for example in invalid_examples {
        //     assert!(identifier(example).is_err())
        // }
    }

    #[test]
    fn test_bop_expr() {
        let valid_examples = ["TRUE & TRUE", "TRUE & FALSE"];
        for input in valid_examples {
            assert!(bop_expr(input).is_ok());
        }
        let valid_examples = ["TRUE&TRUE", "TRUE&FALSE"];
        for input in valid_examples {
            assert!(bop_expr(input).is_ok());
        }
        let valid_examples = ["TRUE&\nTRUE", "TRUE &\n FALSE"];
        for input in valid_examples {
            assert!(bop_expr(input).is_ok());
        }

        let invalid_examples = ["TRUE", "TRUE\n&FALSE"];
        for input in invalid_examples {
            assert!(bop_expr(input).is_err());
        }
    }
}
