use crate::{
    ast::*,
    compound::{for_stmt, repeat, while_stmt},
    literals::{literal, string_literal},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline, none_of, satisfy, space0},
    combinator::{map, not, opt, peek, recognize},
    error::ParseError,
    multi::{fold_many0, many0, separated_list0},
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
pub fn identifier(input: &str) -> IResult<&str, Box<Expression>> {
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
    ))))(input)?;
    fn letter_digit_period_underscore(input: &str) -> IResult<&str, &str> {
        input.split_at_position_complete(|item| {
            !item.is_alphanumeric() && item != '.' && item != '_'
        })
    }

    map(
        alt((
            recognize(tag("...length")),
            recognize(tag("...elt")),
            recognize(tuple((tag(".."), digit1))),
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

/// A function definition is of the form
///
/// function ( arglist ) body
///
/// The function body is an expression, often a compound expression.
/// The arglist is a comma-separated list of items each of which can be an identifier,
/// or of the form ‘identifier = default’, or the special token ....
/// The default can be any valid expression.
///
/// Notice that function arguments unlike list tags, etc.,
/// cannot have “strange names” given as text strings.
fn function_definition(input: &str) -> IResult<&str, Box<Expression>> {
    fn three_dots(input: &str) -> IResult<&str, Box<Expression>> {
        map(tag("..."), |_| {
            Box::new(Expression::Literal(Literal::ThreeDots))
        })(input)
    }
    fn args(input: &str) -> IResult<&str, (Vec<Expression>, Vec<Option<Expression>>)> {
        map(
            separated_list0(
                tuple((tag(","), multispace0)),
                tuple((
                    alt((identifier, three_dots)),
                    opt(tuple((multispace0, tag("="), multispace0, expr))),
                    multispace0,
                )),
            ),
            |args| {
                args.into_iter().fold(
                    (vec![], vec![]),
                    |(mut arg_names, mut arg_values), (name, value, _)| {
                        arg_names.push(*name);
                        match value {
                            Some((_, _, _, value)) => {
                                arg_values.push(Some(*value));
                            }
                            None => arg_values.push(None),
                        }
                        (arg_names, arg_values)
                    },
                )
            },
        )(input)
    }
    map(
        tuple((
            alt((tag("function"), tag("\\"))),
            multispace0,
            delimited(tag("("), args, tag(")")),
            multispace0,
            expr_or_assign_or_help,
        )),
        |(def_keyword, _, args, _, body)| {
            Box::new(Expression::Function(FunctionDefinition {
                arg_names: args.0,
                arg_values: args.1,
                body,
                def_type: if def_keyword == "function" {
                    FunctionDefinitionType::Default
                } else {
                    FunctionDefinitionType::Lambda
                },
            }))
        },
    )(input)
}

/// sublist : sub
/// | sublist cr ',' sub
///
/// sub:
///  | expr
///  | SYMBOL EQ_ASSIGN
///  | SYMBOL EQ_ASSIGN expr
///  | STR_CONST EQ_ASSIGN
///  | STR_CONST EQ_ASSIGN expr
///  | NULL_CONST EQ_ASSIGN
///  | NULL_CONST EQ_ASSIGN expr
fn sublist(input: &str) -> IResult<&str, Vec<Argument>> {
    fn argument(input: &str) -> IResult<&str, Argument> {
        alt((
            map(
                tuple((
                    alt((
                        identifier,
                        map(string_literal, |literal| {
                            Box::new(Expression::Literal(literal))
                        }),
                    )),
                    multispace0,
                    nom::character::complete::char('='),
                    multispace0,
                    expr,
                )),
                |(tag, _, _, _, value)| Argument::Named(tag, value),
            ),
            map(expr, Argument::Positional),
            map(multispace0, |_| Argument::Empty),
        ))(input)
    }
    separated_list0(
        tuple((
            multispace0,
            nom::character::complete::char(','),
            multispace0,
        )),
        argument,
    )(input)
}

/// A function call takes the form of a function reference followed
/// by a comma-separated list of arguments within a set of parentheses.
///
/// function_reference ( arg1, arg2, ...... , argn )
///
/// The function reference can be either
///     an identifier (the name of the function)
///     a text string (ditto, but handy if the function has a name which is not a valid identifier)
///     an expression (which should evaluate to a function object)
///
/// Each argument can be tagged (tag=expr), or just be a simple expression.
/// It can also be empty or it can be one of the special tokens ..., ..2, etc.
///
/// A tag can be an identifier or a text string.
///
/// Examples from the R-lang docs
/// f(x)
/// g(tag = value, , 5)
/// "odd name"("strange tag" = 5, y)
/// (function(x) x^2)(5)
fn function_call(input: &str) -> IResult<&str, Box<Expression>> {
    fn function_reference(input: &str) -> IResult<&str, Box<Expression>> {
        alt((
            identifier,
            map(string_literal, |literal| {
                Box::new(Expression::Literal(literal))
            }),
            subatomic_expression,
        ))(input)
    }

    map(
        tuple((
            function_reference,
            delimited(
                nom::character::complete::char('('),
                sublist,
                nom::character::complete::char(')'),
            ),
        )),
        |(function_reference, sublist)| Box::new(Expression::Call(function_reference, sublist)),
    )(input)
}

/// R has three indexing constructs, two of which are syntactically
/// similar although with somewhat different semantics:
///
/// object [ arg1, ...... , argn ]
/// object [[ arg1, ...... , argn ]]
/// object $ something
///
/// The object can formally be any valid expression,
/// but it is understood to denote or evaluate to a subsettable object.
/// The arguments generally evaluate to numerical or character indices,
/// but other kinds of arguments are possible (notably drop = FALSE).
///
/// Here's what the grammar says about the subscripts:
/// | expr LBB sublist ']' ']'
/// | expr '[' sublist ']'
fn subscript(input: &str) -> IResult<&str, Box<Expression>> {
    map(
        tuple((
            alt((function_call, subatomic_expression)),
            alt((
                map(
                    delimited(
                        nom::character::complete::char('['),
                        sublist,
                        nom::character::complete::char(']'),
                    ),
                    |sublist| (SubscriptType::Single, sublist),
                ),
                map(delimited(tag("[["), sublist, tag("]]")), |sublist| {
                    (SubscriptType::Double, sublist)
                }),
                map(
                    preceded(nom::character::complete::char('$'), sublist),
                    |sublist| (SubscriptType::Dollar, sublist),
                ),
            )),
        )),
        |(object, (subscript_type, sublist))| {
            Box::new(Expression::Subscript(object, sublist, subscript_type))
        },
    )(input)
}

pub fn condition(input: &str) -> IResult<&str, Box<Expression>> {
    delimited(
        tuple((multispace0, nom::character::complete::char('('))),
        expr,
        tuple((
            multispace0,
            nom::character::complete::char(')'),
            multispace0,
        )),
    )(input)
}

fn if_expr(input: &str) -> IResult<&str, Box<Expression>> {
    fn one_if_arm(input: &str) -> IResult<&str, (Box<Expression>, Box<Expression>)> {
        preceded(tag("if"), tuple((condition, expr_or_assign_or_help)))(input)
    }
    map(
        tuple((
            one_if_arm,
            many0(preceded(
                tuple((space0, tag("else"), multispace0)),
                one_if_arm,
            )),
            opt(preceded(
                tuple((space0, tag("else"), multispace0)),
                expr_or_assign_or_help,
            )),
        )),
        |(first_if, other_arms, else_arm)| {
            Box::new(Expression::If(
                std::iter::once(first_if).chain(other_arms).collect(),
                else_arm,
            ))
        },
    )(input)
}

// expr_or_assign_or_help:
// expr |
// expr_or_assign_or_help EQ_ASSIGN expr_or_assign_or_help |
// expr_or_assign_or_help '?'  expr_or_assign_or_help
pub fn expr_or_assign_or_help(input: &str) -> IResult<&str, Box<Expression>> {
    println!("Expr or assign or help:{input}");
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

// expr:
// left_assignment |
// { explist }
pub fn expr(input: &str) -> IResult<&str, Box<Expression>> {
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
            map(tuple((tag("<"), peek(none_of("-")))), |_| Bop::Lower),
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
    // The order here is important.
    // Firstly we want to unwrap the expression from ( or {.
    // Secondly, we want to try matching the possibilities that have others as their prefixes (e.g.
    // function call and identifier).
    //
    // Otherwise, we might end up with an error or an infinite loop.
    alt((
        if_expr,
        while_stmt,
        repeat,
        for_stmt,
        function_definition,
        subscript,
        function_call,
        subatomic_expression,
    ))(input)
}

fn subatomic_expression(input: &str) -> IResult<&str, Box<Expression>> {
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
    fn test_identifier() {
        let valid_examples = [
            "Test",
            "t1",
            "l",
            ".something",
            ".s",
            "underscore_",
            "...length",
            "...elt",
            "..1",
            "..10",
        ];
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
    fn test_left_assignment() {
        let input = "a <- 7";
        let expected = Box::new(Expression::Bop(
            Bop::Assignment,
            Box::new(Expression::Identifier(String::from("a"))),
            Box::new(Expression::Literal(Literal::Number(7.to_string()))),
        ));
        assert_eq!(left_assignment(input), Ok(("", expected)));
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

        assert_eq!(
            expr("\"a\" + 1"),
            Ok((
                "",
                Box::new(Expression::Bop(
                    Bop::Plus,
                    Box::new(Expression::Literal(Literal::String(String::from("a")))),
                    Box::new(Expression::Literal(Literal::Number(1.to_string())))
                ))
            ))
        );
        let seven_plus_one = Box::new(Expression::Bop(
            Bop::Plus,
            Box::new(Expression::Literal(Literal::Number(7.to_string()))),
            Box::new(Expression::Literal(Literal::Number(1.to_string()))),
        ));
        assert_eq!(
            expr("(7+1)*1"),
            Ok((
                "",
                Box::new(Expression::Bop(
                    Bop::Multiply,
                    seven_plus_one,
                    Box::new(Expression::Literal(Literal::Number(1.to_string())))
                ))
            ))
        );
        let seven_plus_one = Box::new(Expression::Bop(
            Bop::Plus,
            Box::new(Expression::Literal(Literal::Number(7.to_string()))),
            Box::new(Expression::Literal(Literal::Number(1.to_string()))),
        ));
        let zero_plus_one = Box::new(Expression::Bop(
            Bop::Plus,
            Box::new(Expression::Literal(Literal::Number(0.to_string()))),
            Box::new(Expression::Literal(Literal::Number(1.to_string()))),
        ));
        assert_eq!(
            expr("(7+1)*(0+1)"),
            Ok((
                "",
                Box::new(Expression::Bop(
                    Bop::Multiply,
                    seven_plus_one,
                    zero_plus_one
                ))
            ))
        );
    }

    #[test]
    fn test_op_precedence() {
        assert_eq!(
            expr("7+1*0+1"),
            Ok((
                "",
                Box::new(Expression::MultiBop(
                    Box::new(Expression::Literal(Literal::Number(7.to_string()))),
                    vec![
                        (
                            Bop::Plus,
                            Expression::Bop(
                                Bop::Multiply,
                                Box::new(Expression::Literal(Literal::Number(1.to_string()))),
                                Box::new(Expression::Literal(Literal::Number(0.to_string())))
                            )
                        ),
                        (
                            Bop::Plus,
                            Expression::Literal(Literal::Number(1.to_string()))
                        )
                    ]
                ))
            ))
        )
    }

    #[test]
    fn test_function_definition() {
        // Different forms but the same
        let examples = [
            "function(a) {7}",
            "function(a){7}",
            "function(a)\n{7}",
            "function (a){7}",
            "function\n(a)\n{7}",
            "function(a)7",
            "function(a)\n7",
        ];
        let expected = Box::new(Expression::Function(FunctionDefinition {
            arg_names: vec![Expression::Identifier(String::from("a"))],
            arg_values: vec![None],
            body: Box::new(Expression::Literal(Literal::Number(7.to_string()))),
            def_type: FunctionDefinitionType::Default,
        }));
        for input in examples {
            assert_eq!(function_definition(input), Ok(("", expected.clone())));
        }

        // Two args
        let input = "function(a, b)\n7";
        let expected = Box::new(Expression::Function(FunctionDefinition {
            arg_names: vec![
                Expression::Identifier(String::from("a")),
                Expression::Identifier(String::from("b")),
            ],
            arg_values: vec![None, None],
            body: Box::new(Expression::Literal(Literal::Number(7.to_string()))),
            def_type: FunctionDefinitionType::Default,
        }));
        assert_eq!(function_definition(input), Ok(("", expected)));

        // Three dots in args
        let input = "function(...)\n7";
        let expected = Box::new(Expression::Function(FunctionDefinition {
            arg_names: vec![Expression::Literal(Literal::ThreeDots)],
            arg_values: vec![None],
            body: Box::new(Expression::Literal(Literal::Number(7.to_string()))),
            def_type: FunctionDefinitionType::Default,
        }));
        assert_eq!(function_definition(input), Ok(("", expected)));

        // Default values
        let input = "function(a=7, b)\n7";
        let expected = Box::new(Expression::Function(FunctionDefinition {
            arg_names: vec![
                Expression::Identifier(String::from("a")),
                Expression::Identifier(String::from("b")),
            ],
            arg_values: vec![
                Some(Expression::Literal(Literal::Number(7.to_string()))),
                None,
            ],
            body: Box::new(Expression::Literal(Literal::Number(7.to_string()))),
            def_type: FunctionDefinitionType::Default,
        }));
        assert_eq!(function_definition(input), Ok(("", expected)));
    }

    #[test]
    fn function_assignment() {
        let input = "a <- function(a) {7}";
        let expected = Box::new(Expression::Bop(
            Bop::Assignment,
            Box::new(Expression::Identifier(String::from("a"))),
            Box::new(Expression::Function(FunctionDefinition {
                arg_names: vec![Expression::Identifier(String::from("a"))],
                arg_values: vec![None],
                body: Box::new(Expression::Literal(Literal::Number(7.to_string()))),
                def_type: FunctionDefinitionType::Default,
            })),
        ));
        assert_eq!(expr(input), Ok(("", expected)));
    }

    #[test]
    fn function_calls() {
        let input = "a()";
        let expected = Box::new(Expression::Call(
            Box::new(Expression::Identifier(String::from("a"))),
            vec![Argument::Empty],
        ));
        assert_eq!(function_call(input), Ok(("", expected)));

        let input = "a(7)";
        let expected = Box::new(Expression::Call(
            Box::new(Expression::Identifier(String::from("a"))),
            vec![Argument::Positional(Box::new(Expression::Literal(
                Literal::Number("7".to_owned()),
            )))],
        ));
        assert_eq!(function_call(input), Ok(("", expected)));

        let input = "a(a = 7)";
        let expected = Box::new(Expression::Call(
            Box::new(Expression::Identifier(String::from("a"))),
            vec![Argument::Named(
                Box::new(Expression::Identifier("a".to_owned())),
                Box::new(Expression::Literal(Literal::Number("7".to_owned()))),
            )],
        ));
        assert_eq!(function_call(input), Ok(("", expected)));

        let input = "\"a\"(a = 7)";
        let expected = Box::new(Expression::Call(
            Box::new(Expression::Literal(Literal::String("a".to_owned()))),
            vec![Argument::Named(
                Box::new(Expression::Identifier("a".to_owned())),
                Box::new(Expression::Literal(Literal::Number("7".to_owned()))),
            )],
        ));
        assert_eq!(function_call(input), Ok(("", expected)));

        let input = "a(\"a\" = 7)";
        let expected = Box::new(Expression::Call(
            Box::new(Expression::Identifier(String::from("a"))),
            vec![Argument::Named(
                Box::new(Expression::Literal(Literal::String("a".to_owned()))),
                Box::new(Expression::Literal(Literal::Number("7".to_owned()))),
            )],
        ));
        assert_eq!(function_call(input), Ok(("", expected)));

        let input = "a(7, \"a\")";
        let expected = Box::new(Expression::Call(
            Box::new(Expression::Identifier(String::from("a"))),
            vec![
                Argument::Positional(Box::new(Expression::Literal(Literal::Number(
                    "7".to_owned(),
                )))),
                Argument::Positional(Box::new(Expression::Literal(Literal::String(
                    "a".to_owned(),
                )))),
            ],
        ));
        assert_eq!(function_call(input), Ok(("", expected)));

        let input = "a(7, ,7)";
        let expected = Box::new(Expression::Call(
            Box::new(Expression::Identifier(String::from("a"))),
            vec![
                Argument::Positional(Box::new(Expression::Literal(Literal::Number(
                    "7".to_owned(),
                )))),
                Argument::Empty,
                Argument::Positional(Box::new(Expression::Literal(Literal::Number(
                    "7".to_owned(),
                )))),
            ],
        ));
        assert_eq!(function_call(input), Ok(("", expected)));

        let input = "(function() 1)()";
        let expected = Box::new(Expression::Call(
            Box::new(Expression::Function(FunctionDefinition {
                arg_names: vec![],
                arg_values: vec![],
                body: Box::new(Expression::Literal(Literal::Number("1".to_owned()))),
                def_type: FunctionDefinitionType::Default,
            })),
            vec![Argument::Empty],
        ));
        assert_eq!(function_call(input), Ok(("", expected)));
    }

    #[test]
    fn subscripts() {
        let input = "a[]";
        let expected = Box::new(Expression::Subscript(
            Box::new(Expression::Identifier("a".to_owned())),
            vec![Argument::Empty],
            SubscriptType::Single,
        ));
        assert_eq!(subscript(input), Ok(("", expected)));

        let input = "a[[]]";
        let expected = Box::new(Expression::Subscript(
            Box::new(Expression::Identifier("a".to_owned())),
            vec![Argument::Empty],
            SubscriptType::Double,
        ));
        assert_eq!(subscript(input), Ok(("", expected)));

        let input = "a[[7]]";
        let expected = Box::new(Expression::Subscript(
            Box::new(Expression::Identifier("a".to_owned())),
            vec![Argument::Positional(Box::new(Expression::Literal(
                Literal::Number("7".to_owned()),
            )))],
            SubscriptType::Double,
        ));
        assert_eq!(subscript(input), Ok(("", expected)));

        let input = "(function() c(1))()[1]";
        let expected = Box::new(Expression::Subscript(
            Box::new(Expression::Call(
                Box::new(Expression::Function(FunctionDefinition {
                    arg_names: vec![],
                    arg_values: vec![],
                    body: Box::new(Expression::Call(
                        Box::new(Expression::Identifier("c".to_string())),
                        vec![Argument::Positional(Box::new(Expression::Literal(
                            Literal::Number("1".to_string()),
                        )))],
                    )),
                    def_type: FunctionDefinitionType::Default,
                })),
                vec![Argument::Empty],
            )),
            vec![Argument::Positional(Box::new(Expression::Literal(
                Literal::Number("1".to_owned()),
            )))],
            SubscriptType::Single,
        ));
        assert_eq!(subscript(input), Ok(("", expected)));

        let input = "a$a";
        let expected = Box::new(Expression::Subscript(
            Box::new(Expression::Identifier("a".to_string())),
            vec![Argument::Positional(Box::new(Expression::Identifier(
                "a".to_string(),
            )))],
            SubscriptType::Dollar,
        ));
        assert_eq!(subscript(input), Ok(("", expected)));
    }

    #[test]
    fn test_if() {
        let input = r#"if
          (TRUE)
          TRUE"#;
        let expected = Box::new(Expression::If(
            vec![(
                Box::new(Expression::Literal(Literal::True)),
                Box::new(Expression::Literal(Literal::True)),
            )],
            None,
        ));
        assert_eq!(if_expr(input), Ok(("", expected)));

        let input = r#"if 
        (FALSE) {} else 
        if (FALSE) {}"#;
        let expected = Box::new(Expression::If(
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
        ));
        assert_eq!(if_expr(input), Ok(("", expected)));

        let input = r#"if 
        (FALSE) {} else TRUE"#;
        let expected = Box::new(Expression::If(
            vec![(
                Box::new(Expression::Literal(Literal::False)),
                Box::new(Expression::Expressions(vec![])),
            )],
            Some(Box::new(Expression::Literal(Literal::True))),
        ));
        assert_eq!(if_expr(input), Ok(("", expected)));

        let input = r#"if 
        (FALSE) {} else
        {}"#;
        let expected = Box::new(Expression::If(
            vec![(
                Box::new(Expression::Literal(Literal::False)),
                Box::new(Expression::Expressions(vec![])),
            )],
            Some(Box::new(Expression::Expressions(vec![]))),
        ));
        assert_eq!(if_expr(input), Ok(("", expected)));
    }
}
