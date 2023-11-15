use crate::ast::*;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{multispace0, newline, none_of, one_of, satisfy, space0},
    combinator::{map, not, opt, peek, recognize},
    error::ParseError,
    multi::{fold_many0, many0, many1},
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

fn placeholder(input: &str) -> IResult<&str, Literal> {
    map(tag("_"), |_| Literal::Placeholder)(input)
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
            placeholder,
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

fn program(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        map(
            tuple((expr_or_assign_or_help, multispace0, newline)),
            |(e, _, _)| Box::new(Expression::Expressions(vec![*e])),
        ),
        map(
            tuple((expr_or_assign_or_help, multispace0, tag(";"))),
            |(e, _, _)| Box::new(Expression::Expressions(vec![*e])),
        ),
        map(multispace0, |_| Box::new(Expression::Expressions(vec![]))),
    ))(input)
}

// expr_or_assign_or_help:
// expr |
// expr_or_assign_or_help EQ_ASSIGN expr_or_assign_or_help |
// expr_or_assign_or_help '?'  expr_or_assign_or_help
fn expr_or_assign_or_help(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        map(
            tuple((expr, space0, tag("="), multispace0, expr_or_assign_or_help)),
            |(e1, _, _, _, e2)| {
                Box::new(Expression::Expressions(vec![Expression::Bop(
                    Bop::OldAssignment,
                    e1,
                    e2,
                )]))
            },
        ),
        map(
            tuple((expr, space0, tag("?"), multispace0, expr_or_assign_or_help)),
            |(e1, _, _, _, e2)| Box::new(Expression::Expressions(vec![*e1, *e2])),
        ),
        expr,
    ))(input)
}

// expr_or_help:
// expr |
// expr_or_help '?' expr_or_help
fn expr_or_help(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        map(
            tuple((expr, space0, tag("?"), multispace0, expr_or_help)),
            |(e1, _, _, _, e2)| Box::new(Expression::Bop(Bop::Questionmark, e1, e2)),
        ),
        expr,
    ))(input)
}

// expr:
// left_assignment |
// literal |
// identifier
fn expr(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        left_assignment,
        map(
            delimited(
                tuple((tag("{"), multispace0)),
                exprlist,
                tuple((multispace0, tag("}"))),
            ),
            |e| e,
        ),
    ))(input)
}

fn exprlist(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        map(
            tuple((
                expr_or_assign_or_help,
                space0,
                tag(";"),
                multispace0,
                expr_or_assign_or_help,
            )),
            |(exprl, _, _, _, exprl2)| Box::new(Expression::Expressions(vec![*exprl, *exprl2])),
        ),
        map(
            tuple((
                expr_or_assign_or_help,
                space0,
                newline,
                multispace0,
                expr_or_assign_or_help,
            )),
            |(e1, _, _, _, e2)| Box::new(Expression::Expressions(vec![*e1, *e2])),
        ),
        map(
            tuple((expr_or_assign_or_help, space0, tag(";"), multispace0)),
            |(e1, _, _, _)| e1,
        ),
        map(
            tuple((expr_or_assign_or_help, space0, newline, multispace0)),
            |(e1, _, _, _)| e1,
        ),
        expr_or_assign_or_help,
        map(multispace0, |_| Box::new(Expression::Expressions(vec![]))),
    ))(input)
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

// left_assignment: right_assignment ['<-' right_assignment]*
fn left_assignment(input: &str) -> IResult<&str, Box<Expression>> {
    bop(right_assignment, map(tag("<-"), |_| Bop::Assignment))(input)
}

// right_assignment: tilde_bop ['->' tilde_bop]*
fn right_assignment(input: &str) -> IResult<&str, Box<Expression>> {
    bop(tilde_bop, map(tag("->"), |_| Bop::RightAssignment))(input)
}

// tilde_bop: or_test ['~' or_test]*
fn tilde_bop(input: &str) -> IResult<&str, Box<Expression>> {
    bop(or_test, map(tag("~"), |_| Bop::ModelFormulae))(input)
}

// or_test: and_test [| and_test]*
fn or_test(input: &str) -> IResult<&str, Box<Expression>> {
    bop(and_test, map(tag("|"), |_| Bop::Or))(input)
}

// and_test: not_test [& not_test]*
fn and_test(input: &str) -> IResult<&str, Box<Expression>> {
    bop(not_test, map(tag("&"), |_| Bop::And))(input)
}

// not_test: '!' not_test | comparison
fn not_test(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        map(preceded(tuple((tag("!"), multispace0)), not_test), |e| {
            Box::new(Expression::Uop(Uop::Not, e))
        }),
        comparison,
    ))(input)
}

// comparison: arithmetic_op [comp_op arithmetic_op]*
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

// arithmetic_op: term [('+' | '-') term]*
fn arithmetic_op(input: &str) -> IResult<&str, Box<Expression>> {
    bop(
        term,
        alt((map(tag("+"), |_| Bop::Plus), map(tag("-"), |_| Bop::Minus))),
    )(input)
}

// term: pipe_op [('*' | '/') pipe_op]*
fn term(input: &str) -> IResult<&str, Box<Expression>> {
    bop(
        pipe_op,
        alt((
            map(tag("*"), |_| Bop::Multiply),
            map(tag("/"), |_| Bop::Divide),
        )),
    )(input)
}

// infix_op: factor [infix_op factor]*
// infix_op: %xyz% |>
fn pipe_op(input: &str) -> IResult<&str, Box<Expression>> {
    bop(factor, alt((infix, map(tag("|>"), |_| Bop::Pipe))))(input)
}

// factor: (+ | -) factor | power
fn factor(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        map(
            preceded(tuple((space0, tag("+"), multispace0)), factor),
            |f| Box::new(Expression::Uop(Uop::Plus, f)),
        ),
        map(
            preceded(tuple((space0, tag("-"), multispace0)), factor),
            |f| Box::new(Expression::Uop(Uop::Minus, f)),
        ),
        power,
    ))(input)
}

// power: expr [^ factor]
fn power(input: &str) -> IResult<&str, Box<Expression>> {
    map(
        tuple((
            atomic_expression,
            opt(preceded(tuple((space0, tag("^"), multispace0)), factor)),
        )),
        |(atomic, f)| match f {
            Some(f) => Box::new(Expression::Bop(Bop::Power, atomic, f)),
            None => atomic,
        },
    )(input)
}

fn atomic_expression(input: &str) -> IResult<&str, Box<Expression>> {
    alt((
        literal,
        identifier,
        delimited(tag("("), expr_or_assign_or_help, tag(")")),
        delimited(tag("{"), expr_or_assign_or_help, tag("}")),
    ))(input)
}

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
    fn test_power() {
        // Parses literals
        assert_eq!(
            power("7"),
            Ok((
                "",
                Box::new(Expression::Literal(Literal::Number(String::from("7"))))
            ))
        );

        // Spaces don't matter
        assert_eq!(power("7^1"), power("7 ^ 1"));

        // Newlines don't matter after the operator
        assert_eq!(power("7^1"), power("7^\n 1"));
    }

    #[test]
    fn test_factor() {
        // Parses literals
        assert_eq!(
            factor("7"),
            Ok((
                "",
                Box::new(Expression::Literal(Literal::Number(String::from("7"))))
            ))
        );

        // Parses plus and minus unary ops
        assert_eq!(
            factor("+7"),
            Ok((
                "",
                Box::new(Expression::Uop(
                    Uop::Plus,
                    Box::new(Expression::Literal(Literal::Number(String::from("7"))))
                ))
            ))
        );

        assert_eq!(
            factor("-7"),
            Ok((
                "",
                Box::new(Expression::Uop(
                    Uop::Minus,
                    Box::new(Expression::Literal(Literal::Number(String::from("7"))))
                ))
            ))
        );

        // Precedence with ^
        assert_eq!(
            factor("-7^1"),
            Ok((
                "",
                Box::new(Expression::Uop(
                    Uop::Minus,
                    Box::new(Expression::Bop(
                        Bop::Power,
                        Box::new(Expression::Literal(Literal::Number(String::from("7")))),
                        Box::new(Expression::Literal(Literal::Number(String::from("1"))))
                    ))
                ))
            ))
        );
    }

    #[test]
    fn test_pipe() {
        // Parses literals
        assert_eq!(
            pipe_op("7"),
            Ok((
                "",
                Box::new(Expression::Literal(Literal::Number(String::from("7"))))
            ))
        );

        assert_eq!(
            pipe_op("7 |> 1"),
            Ok((
                "",
                Box::new(Expression::Bop(
                    Bop::Pipe,
                    Box::new(Expression::Literal(Literal::Number(String::from("7")))),
                    Box::new(Expression::Literal(Literal::Number(String::from("1"))))
                ))
            ))
        );

        // Spaces don't matter
        assert_eq!(pipe_op("7|>1"), pipe_op("7 |> \n1"));
    }
    #[test]
    fn test_expression() {
        // Test with parentheses
        assert_eq!(
            expr("(7)"),
            Ok((
                "",
                Box::new(Expression::Literal(Literal::Number(String::from("7"))))
            ))
        );

        // With or without parentheses ast is the same
        assert_eq!(expr("7"), expr("(7)"));

        println!("{:?}", expr("\"a\" + 1"));
        println!("{:?}", expr("(7+1)*1"));
        println!("{:?}", expr("(7+1)*(0+1)"));
        println!("{:?}", expr("7+1*0+1"));
    }
}
