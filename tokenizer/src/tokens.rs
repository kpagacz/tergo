/// Representation of a token.
///
/// This represents a single token in an R program along with the line on which it occurs
/// and the column offset. Additionally, it store the comments that are associated
/// with the token.
#[derive(Debug, Clone)]
pub struct CommentedToken<'a> {
    /// The actual token stored in this struct.
    pub token: Token<'a>,
    /// The line of the start of this token.
    pub line: u32,
    /// The column offset of the start of this token.
    pub offset: usize,
    /// Preceding comments
    pub leading_comments: &'a [CommentedToken<'a>],
    /// Trailing inline comment
    pub inline_comment: Option<&'a CommentedToken<'a>>,
}

impl<'a> CommentedToken<'a> {
    pub fn new(token: Token<'a>, line: u32, offset: usize) -> Self {
        Self {
            token,
            line,
            offset,
            leading_comments: &[],
            inline_comment: None,
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
    LSubscript,
    RSubscript,
    Comma,

    // Reserved
    Continue,
    Break,

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
