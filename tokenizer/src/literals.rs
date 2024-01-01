use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{none_of, one_of},
    combinator::{map, opt, recognize},
    error::ParseError,
    multi::many1,
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

use crate::{
    ast::{Expression, Literal, Na},
    helpers::CodeSpan,
};

fn true_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    map(tag("TRUE"), |_| Literal::True)(input)
}

fn false_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    map(tag("FALSE"), |_| Literal::False)(input)
}

fn null_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    map(tag("NULL"), |_| Literal::Null)(input)
}

fn na_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    alt((
        map(tag("NA_integer_"), |_| Literal::Na(Na::Integer)),
        map(tag("NA_real_"), |_| Literal::Na(Na::Real)),
        map(tag("NA_complex_"), |_| Literal::Na(Na::Complex)),
        map(tag("NA_character_"), |_| Literal::Na(Na::Character)),
        map(tag("NA"), |_| Literal::Na(Na::Generic)),
    ))(input)
}

fn nan_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    map(tag("NaN"), |_| Literal::NaN)(input)
}

fn inf_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    map(tag("Inf"), |_| Literal::Inf)(input)
}

fn placeholder(input: CodeSpan) -> IResult<CodeSpan, Literal> {
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
pub fn string_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    fn parse_delimited_string<'a, E: ParseError<CodeSpan<'a>>>(
        delimited_by: &'a str,
        string_chars: &'a str,
    ) -> impl Parser<CodeSpan<'a>, CodeSpan<'a>, E> {
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
        |s: CodeSpan| Literal::String(s.to_string()),
    )(input)
}

/// Numeric constants can also be hexadecimal, starting with ‘0x’ or ‘0x’
/// followed by zero or more digits, ‘a-f’ or ‘A-F’. Hexadecimal floating point
/// constants are supported using C99 syntax, e.g. ‘0x1.1p1’.
const HEXADECIMAL_DIGITS: &str = "0123456789abcdefABCDEF";
fn hexadecimal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    fn hex_prefix(input: CodeSpan) -> IResult<CodeSpan, CodeSpan> {
        alt((tag("0x"), tag("0X")))(input)
    }
    map(
        alt((
            // 0x1(.1p10)
            recognize(tuple((
                hex_prefix,
                many1(one_of(HEXADECIMAL_DIGITS)),
                opt(tuple((
                    nom::character::complete::char('.'),
                    many1(one_of(HEXADECIMAL_DIGITS)),
                    one_of("pP"),
                    many1(one_of(HEXADECIMAL_DIGITS)),
                ))),
            ))),
            // 0x.1p10
            recognize(tuple((
                hex_prefix,
                nom::character::complete::char('.'),
                many1(one_of(HEXADECIMAL_DIGITS)),
                one_of("pP"),
                many1(one_of(HEXADECIMAL_DIGITS)),
            ))),
        )),
        |num| Literal::Number(num.to_string()),
    )(input)
}

fn decimal(input: CodeSpan) -> IResult<CodeSpan, CodeSpan> {
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
fn number(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    // The below doesn't work with CodeSpan, would need to cast it to u8 and then back to CodeSpan
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
        |num| Literal::Number(num.to_string()),
    )(input)
}

fn number_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
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
fn integer_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    map(
        tuple((number_literal, nom::character::complete::char('L'))),
        |(num, _)| match num {
            Literal::Number(num) => Literal::Integer(format!("{num}L")),
            _ => unreachable!(),
        },
    )(input)
}

fn complex_literal(input: CodeSpan) -> IResult<CodeSpan, Literal> {
    map(
        tuple((number, nom::character::complete::char('i'))),
        |(num, _)| match num {
            Literal::Number(num) => Literal::Complex(format!("{num}i")),
            _ => unreachable!(),
        },
    )(input)
}

pub fn literal(input: CodeSpan) -> IResult<CodeSpan, Box<Expression>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_na_literal() {
        let examples = [
            "NA",
            "NA_integer_",
            "NA_real_",
            "NA_complex_",
            "NA_character_",
        ];
        let expected = [
            Na::Generic,
            Na::Integer,
            Na::Real,
            Na::Complex,
            Na::Character,
        ];
        for (example, expected) in examples.iter().zip(expected) {
            assert_eq!(
                na_literal(CodeSpan::new(example)).unwrap().1,
                Literal::Na(expected)
            );
        }
    }

    #[test]
    fn test_hexadecimal() {
        let valid_examples = ["0X1", "0x0", "0xABCDEF", "0xabcdef", "0X1234567890abcdef"];
        for example in valid_examples {
            assert_eq!(
                hexadecimal(CodeSpan::new(example)).unwrap().1,
                Literal::Number(example.to_string()),
            )
        }
        let invalid_examples = [".3", "_something", "123", "0X\n0"];
        for example in invalid_examples {
            assert!(hexadecimal(CodeSpan::new(example)).is_err())
        }

        // Floating point
        let input = ["0x0.1p10", "0x.1p10", "0x0.1P10"];
        for example in input {
            let expected = Literal::Number(example.to_string());
            assert_eq!(hexadecimal(CodeSpan::new(example)).unwrap().1, expected);
        }
    }

    #[test]
    fn test_number() {
        let valid_examples = ["0.1", ".2", "1e-7", "1.2e+7"];
        for example in valid_examples {
            assert_eq!(
                number(CodeSpan::new(example)).unwrap().1,
                Literal::Number(example.to_string()),
            )
        }
        let invalid_examples = [".something", "_something", "a123", "X\n0"];
        for example in invalid_examples {
            assert!(
                number(CodeSpan::new(example)).is_err(),
                "Panicked at: {example}"
            )
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
            assert_eq!(
                number_literal(CodeSpan::new(example)).unwrap().1,
                Literal::Number(example.to_string()),
            )
        }
        let invalid_examples = ["something", "\"something\"", "\"0.3\""];
        for example in invalid_examples {
            assert!(number_literal(CodeSpan::new(example)).is_err())
        }
    }

    #[test]
    fn test_integer_literal() {
        let valid_examples = ["0.1L", ".2L", "1e-7L", "1.2e+7L", "0X0L", "0x0L"];
        for example in valid_examples {
            assert_eq!(
                integer_literal(CodeSpan::new(example)).unwrap().1,
                Literal::Integer(example.to_owned()),
            )
        }
        let invalid_examples = ["123", ".something", "_something", "a123", "X\n0"];
        for example in invalid_examples {
            assert!(
                integer_literal(CodeSpan::new(example)).is_err(),
                "Panicked at: {example}"
            )
        }
    }

    #[test]
    fn test_complex_literal() {
        let valid_examples = ["0.1i", ".2i", "1e-7i", "1.2e+7i"];
        for example in valid_examples {
            assert_eq!(
                complex_literal(CodeSpan::new(example)).unwrap().1,
                Literal::Complex(example.to_owned()),
            )
        }
        let invalid_examples = ["123", ".something", "_something", "a123", "X\n0"];
        for example in invalid_examples {
            assert!(
                integer_literal(CodeSpan::new(example)).is_err(),
                "Panicked at: {example}"
            )
        }
    }

    #[test]
    fn test_string_literal() {
        // " delimited
        assert_eq!(
            string_literal(CodeSpan::new("\"Something\"")).unwrap().1,
            Literal::String(String::from("Something")),
        );
        assert_eq!(
            string_literal(CodeSpan::new("\"\\\\\\\"\"")).unwrap().1, // Printed as: "\\\""
            Literal::String(String::from("\\\\\\\"")),
        );
        assert_eq!(
            string_literal(CodeSpan::new("\"apostro\'phe\"")).unwrap().1,
            Literal::String(String::from("apostro\'phe")),
        );
        assert_eq!(
            string_literal(CodeSpan::new("\"Something\" test"))
                .unwrap()
                .1,
            Literal::String(String::from("Something"))
        );

        // ' delimited
        assert_eq!(
            string_literal(CodeSpan::new("\"Something\"")).unwrap().1,
            Literal::String(String::from("Something"))
        );
        assert_eq!(
            string_literal(CodeSpan::new("\"\\\\\\\"\"")).unwrap().1, // Printed as: "\\\""
            Literal::String(String::from("\\\\\\\"")),
        );
        assert_eq!(
            string_literal(CodeSpan::new("\'apostro\"phe\'")).unwrap().1,
            Literal::String(String::from("apostro\"phe"))
        );
        assert_eq!(
            string_literal(CodeSpan::new("\"Something\" test"))
                .unwrap()
                .1,
            Literal::String(String::from("Something"))
        );

        // new lines
        let input = CodeSpan::new(
            r#""Something 
            Something""#,
        );
        let expected = Literal::String("Something \n            Something".to_string());
        assert_eq!(string_literal(input).unwrap().1, expected);
    }
}
