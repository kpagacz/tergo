use nom::{
    bytes::complete::is_not,
    character::complete::{newline, space0},
    combinator::{map, recognize},
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{
    ast::{AstNode, Expression},
    helpers::CodeSpan,
};

pub fn inline_comment(input: CodeSpan) -> IResult<CodeSpan, CodeSpan> {
    map(
        recognize(tuple((nom::character::complete::char('#'), is_not("\n")))),
        |comment: CodeSpan| CodeSpan::new(comment.trim_end()),
    )(input)
}

pub fn comments(input: CodeSpan) -> IResult<CodeSpan, AstNode> {
    map(
        many1(delimited(space0, inline_comment, newline)),
        |comments| {
            AstNode::new(Box::new(Expression::Comments(
                comments
                    .into_iter()
                    .map(|comment| comment.to_string())
                    .collect(),
            )))
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline() {
        let examples = [
            (
                r#"#Comment
        TRUE"#,
                "TRUE",
                "#Comment",
            ),
            ("# Comment with spaces\n", "", "# Comment with spaces"),
        ];

        for (input, rest, expected) in examples {
            let input = CodeSpan::new(input);
            let res = inline_comment(input).unwrap();
            assert_eq!(&res.0[..].trim_start(), &rest);
            assert_eq!(&res.1[..], expected);
        }
    }
}
