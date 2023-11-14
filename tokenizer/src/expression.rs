use crate::{ast::*, statement::statement};
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{multispace0, multispace1, none_of, one_of, satisfy, space0},
    combinator::{map, not, opt, peek, recognize},
    error::ParseError,
    multi::{fold_many0, many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
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

// // TODO: Add support for vectorized operators
// // && and ||
// fn bop(input: &str) -> IResult<&str, Bop> {
//     alt((
//         map(tag("+"), |_| Bop::Plus),
//         map(tag("-"), |_| Bop::Minus),
//         map(tag("*"), |_| Bop::Multiply),
//         map(tag("/"), |_| Bop::Divide),
//         map(tag("%%"), |_| Bop::Modulo),
//         map(tag("^"), |_| Bop::Power),
//         map(tag(">"), |_| Bop::Greater),
//         map(tag(">="), |_| Bop::Ge),
//         map(tag("<"), |_| Bop::Lower),
//         map(tag("<="), |_| Bop::Le),
//         map(tag("=="), |_| Bop::Equal),
//         map(tag("!="), |_| Bop::NotEqual),
//         map(tag("&"), |_| Bop::And),
//         map(tag("|"), |_| Bop::Or),
//         map(tag("~"), |_| Bop::ModelFormulae),
//         // map(tag("<-"), |_| Bop::Assignment),
//         // map(tag("->"), |_| Bop::RightAssignment),
//         // map(tag("="), |_| Bop::OldAssignment),
//         map(tag("$"), |_| Bop::Dollar),
//         map(tag(":"), |_| Bop::Colon),
//         infix,
//     ))(input)
// }

fn uop(input: &str) -> IResult<&str, Uop> {
    alt((
        map(tag("+"), |_| Uop::Plus),
        map(tag("-"), |_| Uop::Minus),
        map(tag("!"), |_| Uop::Not),
    ))(input)
}

fn true_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("TRUE"), |_| Literal::True)(input)
}

fn false_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("FALSE"), |_| Literal::False)(input)
}

fn null_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("NULL"), |_| Literal::Null)(input)
}

fn na_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("NA"), |_| Literal::Na)(input)
}

pub fn nan_literal(input: &str) -> IResult<&str, Literal> {
    map(tag("NaN"), |_| Literal::NaN)(input)
}

fn inf_literal(input: &str) -> IResult<&str, Literal> {
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
fn string_literal(input: &str) -> IResult<&str, Literal> {
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
fn hexadecimal(input: &str) -> IResult<&str, Literal> {
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

fn number_literal(input: &str) -> IResult<&str, Literal> {
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
fn integer_literal(input: &str) -> IResult<&str, Literal> {
    map(
        tuple((number_literal, nom::character::complete::char('L'))),
        |(num, _)| match num {
            Literal::Number(num) => Literal::Integer(format!("{num}L")),
            _ => unreachable!(),
        },
    )(input)
}

fn complex_literal(input: &str) -> IResult<&str, Literal> {
    map(
        tuple((number, nom::character::complete::char('i'))),
        |(num, _)| match num {
            Literal::Number(num) => Literal::Complex(format!("{num}i")),
            _ => unreachable!(),
        },
    )(input)
}

fn literal(input: &str) -> IResult<&str, Box<Expression>> {
    eprintln!("Literal:{input}");
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
        |literal| Box::new(Expression::Literal(literal)),
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
fn identifier(input: &str) -> IResult<&str, Box<Expression>> {
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
    peek(not(alt((
        tag("else"),
        tag("if"),
        tag("repeat"),
        tag("while"),
        tag("function"),
        tag("for"),
        tag("in"),
        tag("next"),
        tag("break"),
        tag("TRUE"),
        tag("FALSE"),
        tag("NULL"),
        tag("Inf"),
        tag("NaN"),
        tag("NA"),
        tag("NA_integer_"),
        tag("NA_real_"),
        tag("NA_complex_"),
        tag("NA_character_"),
        tag("..."),
        // TODO: add ..1, ..2, etc to reserved
    ))))(input)?;
    fn letter_digit_period_underscore(input: &str) -> IResult<&str, &str> {
        input.split_at_position_complete(|item| {
            !item.is_alphanumeric() && item != '.' && item != '_'
        })
    }

    eprintln!("identifier:{input}");
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
        |identifier| Box::new(Expression::Identifier(identifier.to_owned())),
    )(input)
}

fn atomic_expression(input: &str) -> IResult<&str, Box<Expression>> {
    alt((literal, identifier))(input)
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
fn call(input: &str) -> IResult<&str, Box<Expression>> {
    eprintln!("call input: {input}");
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
        |(function, args)| Box::new(Expression::Call(function, args)),
    )(input)
}

// expression: test [<- test]
pub(crate) fn expression(input: &str) -> IResult<&str, Box<Expression>> {
    // TODO different assignments
    map(
        tuple((test, opt(tuple((tag("<-"), test))))),
        |(left, right)| match right {
            Some((_, rhs)) => Box::new(Expression::Assignment(left, rhs)),
            None => left,
        },
    )(input)
}

// test: or_test
fn test(input: &str) -> IResult<&str, Box<Expression>> {
    // Add if expr
    or_test(input)
}

fn bop<'a, Error: ParseError<&'a str>, C, B>(
    child_parser: C,
    bop_parser: B,
) -> impl FnMut(&'a str) -> IResult<&str, Box<Expression>, Error>
where
    C: Parser<&'a str, Box<Expression>, Error> + Copy,
    B: Parser<&'a str, Bop, Error>,
{
    map(
        tuple((
            child_parser,
            fold_many0(
                tuple((delimited(space0, bop_parser, multispace0), child_parser)),
                Vec::new,
                |mut acc: Vec<_>, (op, f)| {
                    acc.push((op, *f));
                    acc
                },
            ),
        )),
        |(first, rest)| match rest.len() {
            0 => first,
            1 => {
                let (op, rhs) = &rest[0];
                Box::new(Expression::Bop(op.clone(), first, Box::new(rhs.clone())))
            }
            _ => Box::new(Expression::MultiBop(first, rest)),
        },
    )
}

// or_test: and_test [| and_test]*
fn or_test(input: &str) -> IResult<&str, Box<Expression>> {
    bop(and_test, map(tag("|"), |_| Bop::Or))(input)
}

// and_test: not_test [& not_test]*
fn and_test(input: &str) -> IResult<&str, Box<Expression>> {
    bop(not_test, map(tag("&"), |_| Bop::And))(input)
}

// not_test: ! not_test | comparison
fn not_test(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        map(preceded(tuple((tag("!"), multispace0)), not_test), |e| {
            Box::new(Expression::Uop(Uop::Not, e))
        }),
        comparison,
    ))(input)
}

// comparison: expr (comp_op expr)*
// comp_op: > >= < <= == !=
fn comparison(input: &str) -> IResult<&str, Box<Expression>> {
    bop(
        arithmetic_op,
        alt((
            map(tag(">"), |_| Bop::Greater),
            map(tag(">="), |_| Bop::Ge),
            map(tag("<"), |_| Bop::Lower),
            map(tag("<="), |_| Bop::Le),
            map(tag("=="), |_| Bop::Equal),
            map(tag("!="), |_| Bop::NotEqual),
        )),
    )(input)
}

// arithmetic_op: factor [factor_op factor]*
// factor_op: * /
fn arithmetic_op(input: &str) -> IResult<&str, Box<Expression>> {
    bop(
        infix_op,
        alt((
            map(tag("*"), |_| Bop::Multiply),
            map(tag("/"), |_| Bop::Divide),
        )),
    )(input)
}

// infix_op: array_literal [infix_op array_literal]*
// infix_op: %xyz% |>
fn infix_op(input: &str) -> IResult<&str, Box<Expression>> {
    bop(factor, alt((infix, map(tag("|>"), |_| Bop::Pipe))))(input)
}

// factor: (+ | -) factor | power
fn factor(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        map(
            preceded(tuple((multispace0, tag("+"), multispace0)), factor),
            |f| Box::new(Expression::Uop(Uop::Plus, f)),
        ),
        map(
            preceded(tuple((multispace0, tag("-"), multispace0)), factor),
            |f| Box::new(Expression::Uop(Uop::Minus, f)),
        ),
        power,
    ))(input)
}

// power: atomic_expression [^ factor]
fn power(input: &str) -> IResult<&str, Box<Expression>> {
    map(
        tuple((
            atomic_expression,
            opt(preceded(
                tuple((multispace0, tag("^"), multispace0)),
                factor,
            )),
        )),
        |(atomic, f)| match f {
            Some(f) => Box::new(Expression::Bop(Bop::Power, atomic, f)),
            None => atomic,
        },
    )(input)
}

pub(crate) fn block(input: &str) -> IResult<&str, Expression> {
    eprintln!("block:{input}");
    map(
        delimited(
            nom::character::complete::char('{'),
            many0(statement),
            nom::character::complete::char('}'),
        ),
        |stmts| Expression::Block(stmts),
    )(input)
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
        expression::identifier,
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
                IResult::Ok(("", Box::new(Expression::Identifier(example.to_owned())))),
            )
        }
        assert_eq!(
            identifier("value,"),
            Ok((",", Box::new(Expression::Identifier(String::from("value")))))
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
            // "lib::f()"
        ];

        for input in valid_examples {
            let call = call(input);
            eprintln!("{call:?}");
            assert!(call.is_ok());
        }
    }
}
