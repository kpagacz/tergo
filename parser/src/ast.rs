use tokenizer::tokens::CommentedToken;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Symbol(&'a CommentedToken<'a>),
    Literal(&'a CommentedToken<'a>),
    Comment(&'a CommentedToken<'a>),
    Term(Box<TermExpr<'a>>),
    Bop(
        &'a CommentedToken<'a>,
        Box<Expression<'a>>,
        Box<Expression<'a>>,
    ),
    Newline(&'a CommentedToken<'a>),
    Whitespace(&'a [&'a CommentedToken<'a>]),
    EOF(&'a CommentedToken<'a>),
    FunctionDef(FunctionDefinition<'a>),
}

// Term
#[derive(Debug, Clone, PartialEq)]
pub struct TermExpr<'a> {
    pub pre_delimiters: Option<&'a CommentedToken<'a>>,
    pub term: Option<Expression<'a>>,
    pub post_delimiters: Option<&'a CommentedToken<'a>>,
}

impl<'a> TermExpr<'a> {
    pub fn new(
        pre_delimiters: Option<&'a CommentedToken<'a>>,
        term: Option<Expression<'a>>,
        post_delimiters: Option<&'a CommentedToken<'a>>,
    ) -> Self {
        Self {
            pre_delimiters,
            term,
            post_delimiters,
        }
    }
}

impl<'a> From<Expression<'a>> for TermExpr<'a> {
    fn from(expr: Expression<'a>) -> Self {
        Self::new(None, Some(expr), None)
    }
}

// Function definition
// The comma is required due to the way the parser treats comments
// The formatter needs comments and some of them might end up squeezed into
// the comma token
#[derive(Debug, Clone, PartialEq)]
pub struct Arg<'a>(pub Expression<'a>, pub Option<Expression<'a>>); // Argument, comma

#[derive(Debug, Clone, PartialEq)]
pub struct Args<'a> {
    pub left_delimeter: Box<Expression<'a>>,
    pub args: Vec<Arg<'a>>,
    pub right_delimeter: Box<Expression<'a>>,
}

impl<'a> Args<'a> {
    pub fn new(
        left_delimeter: Box<Expression<'a>>,
        args: Vec<Arg<'a>>,
        right_delimeter: Box<Expression<'a>>,
    ) -> Self {
        Self {
            left_delimeter,
            args,
            right_delimeter,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition<'a> {
    pub arguments: Args<'a>,
    pub body: Vec<Expression<'a>>,
}

impl<'a> FunctionDefinition<'a> {
    pub fn new(arguments: Args<'a>, body: Vec<Expression<'a>>) -> Self {
        Self { arguments, body }
    }
}
