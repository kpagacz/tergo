use formatter;
use parser::ast::Expression;
use tokenizer::{commented_tokens, tokens::CommentedToken, Token};

struct FormatingConfigMock {
    pub line_length: i32,
    pub indent: i32,
}

impl formatter::config::FormattingConfig for FormatingConfigMock {
    fn line_length(&self) -> i32 {
        self.line_length
    }
    fn indent(&self) -> i32 {
        self.indent
    }
}

const CONFIG: FormatingConfigMock = FormatingConfigMock {
    line_length: 0,
    indent: 0,
};

#[test]
fn test_format_simple_bop() {
    let commented_tokens = commented_tokens!(Token::Plus, Token::Literal("1"), Token::Literal("2"));
    let expressions = [Expression::Bop(
        &commented_tokens[0],
        Box::new(Expression::Literal(&commented_tokens[1])),
        Box::new(Expression::Literal(&commented_tokens[2])),
    )];

    let formatted = formatter::format_code(&expressions, &CONFIG);
    assert_eq!(formatted, "1\n+\n2\n");
}
