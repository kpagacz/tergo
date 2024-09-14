use parser::ast::Expression;
use tergo_formatter::{config, format_code};
use tokenizer::{commented_tokens, tokens::CommentedToken, Token};

struct FormatingConfigMock {
    pub line_length: i32,
    pub indent: i32,
}

impl config::FormattingConfig for FormatingConfigMock {
    fn line_length(&self) -> i32 {
        self.line_length
    }
    fn indent(&self) -> i32 {
        self.indent
    }
}

impl std::fmt::Display for FormatingConfigMock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Line length: {}, Indent: {}",
            self.line_length, self.indent
        ))
    }
}

#[test]
fn test_format_simple_bop() {
    let config: FormatingConfigMock = FormatingConfigMock {
        line_length: 0,
        indent: 0,
    };
    let commented_tokens = commented_tokens!(Token::Plus, Token::Literal("1"), Token::Literal("2"));
    let expression = Expression::Bop(
        &commented_tokens[0],
        Box::new(Expression::Literal(&commented_tokens[1])),
        Box::new(Expression::Literal(&commented_tokens[2])),
    );

    let formatted = format_code(expression, &config);
    assert_eq!(formatted, "1 +\n2");
}
