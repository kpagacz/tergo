use crate::ast::*;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{multispace0, none_of, one_of, satisfy, space0},
    combinator::{map, opt, peek, recognize},
    error::ParseError,
    multi::{fold_many0, fold_many1, many0, many1, separated_list0},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, InputTakeAtPosition, Parser,
};

/// R allows user-defined infix operators.
/// These have the form of a string of characters delimited by the ‘%’ character.
/// The string can contain any printable character except ‘%’.
/// The escape sequences for strings do not apply here.
/// Note that the following operators are predefined
/// %% %*% %/% %in% %o% %x%
fn infix(input: &str) -> IResult<&str, Bop> {
    map(
        recognize(tuple((
            nom::character::complete::char('%'),
            many0(none_of("%")),
            nom::character::complete::char('%'),
        ))),
        |op| Bop::Infix(String::from(op)),
    )(input)
}

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
        infix,
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

/// String constants are delimited by a pair of single (‘'’) or double (‘"’) quotes
/// and can contain all other printable characters. Quotes and other special
/// characters within strings are specified using escape sequences: \' single quote
/// \" double quote \n newline (aka ‘line feed’, LF) \r carriage return (CR)
/// \t tab character \b backspace \a bell \f form feed \v vertical tab \\ backslash itself
/// \nnn character with given octal code – sequences of one, two or three digits
/// in the range 0 ... 7 are accepted.  \xnn character with given hex code – sequences
/// of one or two hex digits (with entries 0 ... 9 A ... F a ... f).
/// \unnnn \u{nnnn} (where multibyte locales are supported, otherwise an error).
/// Unicode character with given hex code – sequences of up to four hex digits.
/// The character needs to be valid in the current locale.
/// \Unnnnnnnn \U{nnnnnnnn} (where multibyte locales are supported, otherwise an error).
/// Unicode character with given hex code – sequences of up to eight hex digits.
/// A single quote may also be embedded directly in a double-quote delimited string and vice versa.
/// A ‘nul’ (\0) is not allowed in a character string,
/// so using \0 in a string constant terminates the constant (usually with a warning):
/// further characters up to the closing quote are scanned but ignored.
pub fn string_literal(input: &str) -> IResult<&str, Literal> {
    // TODO: support line breaks in the middle of string literals
    // I don't even know how R behaves if there are line breaks in the middle
    // of a string literal
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

/// Numeric constants can also be hexadecimal, starting with ‘0x’ or ‘0x’
/// followed by zero or more digits, ‘a-f’ or ‘A-F’. Hexadecimal floating point
/// constants are supported using C99 syntax, e.g. ‘0x1.1p1’.
pub fn hexadecimal(input: &str) -> IResult<&str, Literal> {
    // TODO: Add hexadecimal fraction
    fn hex_prefix(input: &str) -> IResult<&str, &str> {
        alt((tag("0x"), tag("0X")))(input)
    }
    map(
        recognize(tuple((hex_prefix, many1(one_of("0123456789abcdefABCDEF"))))),
        |num| Literal::Number(num.to_owned()),
    )(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789")))(input)
}

/// Numeric constants follow a similar syntax to that of the C language.
/// They consist of an integer part consisting of zero or more digits,
/// followed optionally by ‘.’ and a fractional part of zero or more digits
/// optionally followed by an exponent part consisting of an ‘E’ or an ‘e’,
/// an optional sign and a string of one or more digits.
/// Either the fractional or the decimal part can be empty, but not both at once.
/// Valid numeric constants: 1 10 0.1 .2 1e-7 1.2e+7
/// Adapted from https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#floating-point-numbers
fn number(input: &str) -> IResult<&str, Literal> {
    // The below doesn't work with &str, would need to cast it to u8 and then back to &str
    // map(recognize_float, |num| Literal::Number(num))(input)
    map(
        alt((
            // Case one: .42
            recognize(tuple((
                nom::character::complete::char('.'),
                decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
            ))),
            // Case two: 42e42 and 42.42e42
            recognize(tuple((
                decimal,
                opt(preceded(nom::character::complete::char('.'), decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal,
            ))),
            // Case three: 42. and 42.42
            recognize(tuple((
                decimal,
                nom::character::complete::char('.'),
                opt(decimal),
            ))),
            recognize(decimal),
        )),
        |num| Literal::Number(num.to_owned()),
    )(input)
}

pub fn number_literal(input: &str) -> IResult<&str, Literal> {
    alt((hexadecimal, number))(input)
}

/// There is now a separate class of integer constants.
/// They are created by using the qualifier L at the end of the number.
/// For example, 123L gives an integer value rather than a numeric value.
/// The suffix L can be used to qualify
/// any non-complex number with the intent of creating an integer.
/// So it can be used with numbers given by hexadecimal or scientific notation.
/// However, if the value is not a valid integer, a warning is emitted and the numeric value created.
/// The following shows examples of valid integer constants,
/// values which will generate a warning and give numeric constants and syntax errors.
///
/// Valid integer constants:  1L, 0x10L, 1000000L, 1e6L
/// Valid numeric constants:  1.1L, 1e-3L, 0x1.1p-2
/// Syntax error:  12iL 0x1.1
pub fn integer_literal(input: &str) -> IResult<&str, Literal> {
    map(
        tuple((number_literal, nom::character::complete::char('L'))),
        |(num, _)| match num {
            Literal::Number(num) => Literal::Integer(format!("{num}L")),
            _ => unreachable!(),
        },
    )(input)
}

pub fn complex_literal(input: &str) -> IResult<&str, Literal> {
    map(
        tuple((number, nom::character::complete::char('i'))),
        |(num, _)| match num {
            Literal::Number(num) => Literal::Complex(format!("{num}i")),
            _ => unreachable!(),
        },
    )(input)
}

pub fn literal(input: &str) -> IResult<&str, Expression> {
    eprintln!("Literal: {input}");
    map(
        alt((
            true_literal,
            false_literal,
            null_literal,
            na_literal,
            nan_literal,
            inf_literal,
            number_literal,
            complex_literal,
            integer_literal,
            string_literal,
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
    // TODO: add support for identifiers declared and referenced in this way:
    // e.g.
    // `% test%` <- function(first, second) first + second
    // `% test%`(1, 2) == 3
    // Interestingly, this doesn't work
    // # 1 `% test%` 2
    // but this does:
    // `some thing` <- 1
    // print(`some thing` + 2)
    // What it tells me is that the binary operator cannot be referenced in this way
    // (it cannot be referenced as an identifier), but it can be referenced as the name
    // of the function.
    fn letter_digit_period_underscore(input: &str) -> IResult<&str, &str> {
        input.split_at_position_complete(|item| {
            !item.is_alphanumeric() && item != '.' && item != '_'
        })
    }

    eprintln!("identifier: {input}");
    map(
        alt((
            recognize(pair(
                satisfy(|c| c.is_alphabetic()),
                letter_digit_period_underscore,
            )),
            recognize(tuple((
                nom::character::complete::char('.'),
                satisfy(|c| c.is_alphabetic()),
                letter_digit_period_underscore,
            ))),
        )),
        |identifier| Expression::Identifier(identifier.to_owned()),
    )(input)
}

/// A function call takes the form of a function reference followed
/// by a comma-separated list of arguments within a set of parentheses.
/// function_reference ( arg1, arg2, ...... , argn )
/// The function reference can be either
///     an identifier (the name of the function)
///     a text string (ditto, but handy if the function has a name which is not a valid identifier)
///     an expression (which should evaluate to a function object)
/// Each argument can be tagged (tag=expr), or just be a simple expression.
/// It can also be empty or it can be one of the special tokens ..., ..2, etc.
/// A tag can be an identifier or a text string.
/// Examples:
/// f(x)
/// g(tag = value, , 5)
/// "odd name"("strange tag" = 5, y)
/// (function(x) x^2)(5)
pub fn call(input: &str) -> IResult<&str, Expression> {
    println!("call input: {input}");
    fn function_args(input: &str) -> IResult<&str, Vec<Argument>> {
        separated_list0(
            delimited(
                multispace0,
                nom::character::complete::char(','),
                multispace0,
            ),
            alt((
                map(
                    tuple((
                        opt(tuple((
                            alt((recognize(identifier), recognize(string_literal))),
                            multispace0,
                            nom::character::complete::char('='),
                            multispace0,
                        ))),
                        expression,
                    )),
                    |(optional, value)| match optional {
                        Some((tag, _, _, _)) => Argument::Named(String::from(tag), value),
                        None => Argument::Positional(value),
                    },
                ),
                map(multispace0, |_| Argument::Empty),
            )),
        )(input)
    }

    map(
        tuple((
            expression,
            delimited(
                nom::character::complete::char('('),
                function_args,
                nom::character::complete::char(')'),
            ),
        )),
        |(function, args)| Expression::Call(Box::new(function), args),
    )(input)
}

// uop_expr := uop space_with_newlines expression
pub fn uop_expr(input: &str) -> IResult<&str, Expression> {
    println!("uop: {input}");
    map(tuple((uop, multispace0, expression)), |(uop, _, expr)| {
        Expression::Uop(uop, Box::new(expr))
    })(input)
}

// lhs (bop rhs)+
pub fn bop_expr(input: &str) -> IResult<&str, Expression> {
    println!("bop: {input}");
    map(
        tuple((
            expression,
            space0,
            bop,
            multispace0,
            expression,
            fold_many0(
                tuple((space0, bop, multispace0, expression)),
                || vec![],
                |mut acc, (_, bop, _, expr)| {
                    acc.push((bop, Box::new(expr)));
                    acc
                },
            ),
        )),
        |(expr1, _, bop, _, expr2, rest)| {
            if rest.is_empty() {
                Expression::Bop(bop, Box::new(expr1), Box::new(expr2))
            } else {
                Expression::MultiBop(
                    Box::new(Expression::Bop(bop, Box::new(expr1), Box::new(expr2))),
                    rest,
                )
            }
        },
    )(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    // Unfortunately, the order here is important.
    // If the bop_expr goes before the uop_expr
    // the recurrent relation between box_expr and expression
    // will never end for such expressions: +-5
    // It's a poor man's 1 character look ahead.
    println!("expr:{input}");
    peek(none_of(","))(input)?;
    alt((literal, identifier, uop_expr, bop_expr, call))(input)
}

// TODO: disallow reserved keywords
// The following identifiers have a special meaning and cannot be used for object names
// if else repeat while function for in next break
// TRUE FALSE NULL Inf NaN
// NA NA_integer_ NA_real_ NA_complex_ NA_character_
// ... ..1 ..2 etc.

#[cfg(test)]
mod tests {
    use nom::IResult;

    use crate::{
        ast::{Expression, Literal},
        expression::{bop_expr, identifier},
        helpers::assert_parse_eq,
    };

    use super::*;

    #[test]
    fn test_infix() {
        let valid_examples = ["% something %", "%%"];
        for example in valid_examples {
            assert_parse_eq(
                infix(example),
                IResult::Ok(("", Bop::Infix(example.to_owned()))),
            )
        }
        let remaining = "%%2";
        assert_parse_eq(
            infix(remaining),
            IResult::Ok(("2", Bop::Infix(remaining[..2].to_owned()))),
        );
        let invalid_examples = ["a%%", "..."];
        for example in invalid_examples {
            assert!(infix(example).is_err())
        }
    }

    #[test]
    fn test_hexadecimal() {
        let valid_examples = ["0X1", "0x0", "0xABCDEF", "0xabcdef", "0X1234567890abcdef"];
        for example in valid_examples {
            assert_parse_eq(
                hexadecimal(example),
                IResult::Ok(("", Literal::Number(example.to_owned()))),
            )
        }
        let invalid_examples = [".3", "_something", "123", "0X\n0"];
        for example in invalid_examples {
            assert!(hexadecimal(example).is_err())
        }
    }

    #[test]
    fn test_number() {
        let valid_examples = ["0.1", ".2", "1e-7", "1.2e+7"];
        for example in valid_examples {
            assert_parse_eq(
                number(example),
                IResult::Ok(("", Literal::Number(example.to_owned()))),
            )
        }
        let invalid_examples = [".something", "_something", "a123", "X\n0"];
        for example in invalid_examples {
            assert!(number(example).is_err(), "Panicked at: {example}")
        }
    }

    #[test]
    fn test_number_literal() {
        let valid_examples = [
            "0X1",
            "0x0",
            "0xABCDEF",
            "0xabcdef",
            "0X1234567890abcdef",
            "0.1",
            "1",
            "10",
            ".2",
            "1e-7",
            "1.2e+7",
        ];
        for example in valid_examples {
            assert_parse_eq(
                number_literal(example),
                IResult::Ok(("", Literal::Number(example.to_owned()))),
            )
        }
        let invalid_examples = ["something", "\"something\"", "\"0.3\""];
        for example in invalid_examples {
            assert!(number_literal(example).is_err())
        }
    }

    #[test]
    fn test_integer_literal() {
        let valid_examples = ["0.1L", ".2L", "1e-7L", "1.2e+7L", "0X0L", "0x0L"];
        for example in valid_examples {
            assert_parse_eq(
                integer_literal(example),
                IResult::Ok(("", Literal::Integer(example.to_owned()))),
            )
        }
        let invalid_examples = ["123", ".something", "_something", "a123", "X\n0"];
        for example in invalid_examples {
            assert!(integer_literal(example).is_err(), "Panicked at: {example}")
        }
    }

    #[test]
    fn test_complex_literal() {
        let valid_examples = ["0.1i", ".2i", "1e-7i", "1.2e+7i"];
        for example in valid_examples {
            assert_parse_eq(
                complex_literal(example),
                IResult::Ok(("", Literal::Complex(example.to_owned()))),
            )
        }
        let invalid_examples = ["123", ".something", "_something", "a123", "X\n0"];
        for example in invalid_examples {
            assert!(integer_literal(example).is_err(), "Panicked at: {example}")
        }
    }

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
        assert_eq!(
            identifier("value,"),
            Ok((",", Expression::Identifier(String::from("value"))))
        );

        let invalid_examples = [".3", "_something", "123"];
        for example in invalid_examples {
            assert!(identifier(example).is_err())
        }
    }

    #[test]
    fn test_call() {
        let valid_examples = vec![
            "f(x)",
            "g(tag = value, , 5)",
            "\"odd name\"(\"strange tag\" = 5, y)",
            // "(function(x) x^2)(5)", TODO: make this test pass after function definitions are implemented
        ];

        for input in valid_examples {
            let call = call(input);
            println!("{call:?}");
            assert!(call.is_ok());
        }
    }

    #[test]
    fn test_uop_expr() {
        assert_parse_eq(
            uop_expr("+5"),
            IResult::Ok((
                "",
                Expression::Uop(
                    Uop::Plus,
                    Box::new(Expression::Literal(Literal::Number(String::from("5")))),
                ),
            )),
        );

        assert_parse_eq(
            uop_expr("-5"),
            IResult::Ok((
                "",
                Expression::Uop(
                    Uop::Minus,
                    Box::new(Expression::Literal(Literal::Number(String::from("5")))),
                ),
            )),
        );

        assert_parse_eq(
            uop_expr("!5"),
            IResult::Ok((
                "",
                Expression::Uop(
                    Uop::Not,
                    Box::new(Expression::Literal(Literal::Number(String::from("5")))),
                ),
            )),
        );

        assert_parse_eq(
            uop_expr("+\n\n5"),
            IResult::Ok((
                "",
                Expression::Uop(
                    Uop::Plus,
                    Box::new(Expression::Literal(Literal::Number(String::from("5")))),
                ),
            )),
        );

        assert_parse_eq(
            uop_expr("+-5"),
            IResult::Ok((
                "",
                Expression::Uop(
                    Uop::Plus,
                    Box::new(Expression::Uop(
                        Uop::Minus,
                        Box::new(Expression::Literal(Literal::Number(String::from("5")))),
                    )),
                ),
            )),
        );

        assert!(uop_expr(".something").is_err());
        assert!(uop_expr("3").is_err());
    }

    #[test]
    fn test_bop_expr() {
        let valid_examples = ["TRUE & TRUE", "TRUE & FALSE"];
        for input in valid_examples {
            let bop = bop_expr(input);
            println!("{bop:?}");
            assert!(bop.is_ok());
        }
        let valid_examples = ["TRUE&TRUE", "TRUE&FALSE"];
        for input in valid_examples {
            let bop = bop_expr(input);
            println!("{bop:?}");
            assert!(bop_expr(input).is_ok());
        }
        let valid_examples = ["TRUE&\nTRUE", "TRUE &\n FALSE"];
        for input in valid_examples {
            let bop = bop_expr(input);
            println!("{bop:?}");
            assert!(bop_expr(input).is_ok());
        }

        let invalid_examples = ["TRUE", "TRUE\n&FALSE"];
        for input in invalid_examples {
            assert!(bop_expr(input).is_err());
        }
    }

    #[test]
    fn test_multibox_expr() {
        assert_eq!(
            bop_expr("3 + 7 + 8"),
            Ok((
                "",
                Expression::MultiBop(
                    Box::new(Expression::Bop(
                        Bop::Plus,
                        Box::new(Expression::Literal(Literal::Number(String::from("3")))),
                        Box::new(Expression::Literal(Literal::Number(String::from("7"))))
                    )),
                    vec![(
                        Bop::Plus,
                        Box::new(Expression::Literal(Literal::Number(String::from("8"))))
                    )]
                )
            ))
        );

        assert_eq!(
            bop_expr("3 +\n 7 +\n 8"),
            Ok((
                "",
                Expression::MultiBop(
                    Box::new(Expression::Bop(
                        Bop::Plus,
                        Box::new(Expression::Literal(Literal::Number(String::from("3")))),
                        Box::new(Expression::Literal(Literal::Number(String::from("7"))))
                    )),
                    vec![(
                        Bop::Plus,
                        Box::new(Expression::Literal(Literal::Number(String::from("8"))))
                    )]
                )
            ))
        );

        assert_eq!(
            bop_expr("3 +\n 7 +\n 8 * 3"),
            Ok((
                "",
                Expression::MultiBop(
                    Box::new(Expression::Bop(
                        Bop::Plus,
                        Box::new(Expression::Literal(Literal::Number(String::from("3")))),
                        Box::new(Expression::Literal(Literal::Number(String::from("7"))))
                    )),
                    vec![
                        (
                            Bop::Plus,
                            Box::new(Expression::Literal(Literal::Number(String::from("8"))))
                        ),
                        (
                            Bop::Multiply,
                            Box::new(Expression::Literal(Literal::Number(String::from("3"))))
                        )
                    ]
                )
            ))
        );
    }
}
