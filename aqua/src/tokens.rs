/// Representation of a token.
///
/// This represents a single token in an R program along with the line on which it occurs
/// and the column offset. Additionally, it stores the comments that are associated
/// with the token.
#[derive(Debug, Clone)]
pub struct CommentedToken<'a> {
    /// The actual token stored in this struct.
    pub token: Token<'a>,
    /// The column offset of the start of this token.
    pub offset: usize,
    /// Preceding comments.
    pub leading_comments: Option<Vec<&'a str>>,
    /// Trailing inline comment.
    pub inline_comment: Option<&'a str>,
}

impl<'a> Deref for CommentedToken<'a> {
    type Target = Token<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl<'a> CommentedToken<'a> {
    pub fn new(token: Token<'a>, offset: usize) -> Self {
        Self {
            token,
            offset,
            leading_comments: None,
            inline_comment: None,
        }
    }

    pub fn with_comments(
        token: Token<'a>,
        offset: usize,
        leading_comments: Option<Vec<&'a str>>,
        inline_comment: Option<&'a str>,
    ) -> Self {
        Self {
            token,
            offset,
            leading_comments,
            inline_comment,
        }
    }
}

/// When comparing two tokens, only the token itself is compared.
/// The line and offset are ignored.
impl<'a> PartialEq for CommentedToken<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

impl<'a> std::fmt::Display for CommentedToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.token))
    }
}

/// This represents all the different token types encountered
/// in an R program.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Symbol(&'a str),
    Literal(&'a str),
    Semicolon,
    Newline,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,

    // Reserved
    Continue,
    Break,
    Stop,

    // Compound
    If,
    Else,
    While,
    For,
    Repeat,
    In,
    Function,
    Lambda,

    // Binary operators
    LAssign,
    SuperAssign,
    ColonAssign,
    RAssign,
    OldAssign,
    Equal,
    NotEqual,
    LowerThan,
    GreaterThan,
    LowerEqual,
    GreaterEqual,
    Power,
    Divide,
    Multiply,
    Minus,
    Plus,
    Help,
    And,
    VectorizedAnd,
    Or,
    VectorizedOr,
    Dollar,
    Pipe,
    Modulo,
    NsGet,
    NsGetInt,
    Tilde,
    Colon,
    Slot,
    Special(&'a str),

    // Unary operators
    UnaryNot,

    // Comments
    InlineComment(&'a str),
    Comment(&'a str),

    // EOF
    EOF,
}

#[macro_export]
macro_rules! commented_tokens {
    ($($args:expr),*) => {{
        vec![
        $(
            CommentedToken::new($args, 0),
        )*
        ]
    }}
}
use std::ops::Deref;

pub use commented_tokens;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token::*;

    #[test]
    fn commented_tokens_macro() {
        let tokens = commented_tokens![Symbol("a"), InlineComment("# Comment")];
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Symbol("a"));
        assert_eq!(tokens[1].token, InlineComment("# Comment"));
    }

    #[test]
    fn test_display() {
        let token = CommentedToken::new(Token::Symbol("a"), 0);
        let displayed = format!("{}", token);
        assert_eq!("Symbol(\"a\")", displayed);
    }
}
