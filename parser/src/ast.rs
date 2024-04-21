use tokenizer::{tokens::CommentedToken, tokens_buffer::TokensBuffer};

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
    LambdaFunction(Lambda<'a>),
    IfExpression(IfExpression<'a>),
    WhileExpression(WhileExpression<'a>),
    RepeatExpression(RepeatExpression<'a>),
    FunctionCall(FunctionCall<'a>),
    SubsetExpression(SubsetExpression<'a>),
    ForLoopExpression(ForLoop<'a>),
    Break(&'a CommentedToken<'a>),
    Continue(&'a CommentedToken<'a>),
}

impl std::fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Symbol(token) => f.write_fmt(format_args!("{}", TokensBuffer(&[token]))),
            Expression::Literal(token) => f.write_fmt(format_args!("{}", TokensBuffer(&[token]))),
            Expression::Comment(token) => f.write_fmt(format_args!("{}", TokensBuffer(&[token]))),
            Expression::Term(term) => f.write_fmt(format_args!("{}", term)),
            Expression::Bop(op, left, right) => {
                f.write_fmt(format_args!("{} {} {}", left, TokensBuffer(&[op]), right))
            }
            Expression::Newline(token) => f.write_fmt(format_args!("{}", TokensBuffer(&[token]))),
            Expression::Whitespace(tokens) => f.write_fmt(format_args!("{}", TokensBuffer(tokens))),
            Expression::EOF(token) => f.write_fmt(format_args!("{}", TokensBuffer(&[token]))),
            Expression::FunctionDef(func_def) => f.write_fmt(format_args!("{}", func_def)),
            Expression::IfExpression(if_expression) => {
                f.write_fmt(format_args!("{}", if_expression))
            }
            Expression::WhileExpression(while_expression) => {
                f.write_fmt(format_args!("{}", while_expression))
            }
            Expression::RepeatExpression(repeat_expression) => {
                f.write_fmt(format_args!("{}", repeat_expression))
            }
            Expression::FunctionCall(function_call) => {
                f.write_fmt(format_args!("{}", function_call))
            }
            Expression::SubsetExpression(subset_expression) => {
                f.write_fmt(format_args!("{}", subset_expression))
            }
            Expression::ForLoopExpression(for_loop) => f.write_fmt(format_args!("{}", for_loop)),
            Expression::Break(token) | Expression::Continue(token) => {
                f.write_fmt(format_args!("{}", TokensBuffer(&[token])))
            }
            Expression::LambdaFunction(lambda) => f.write_fmt(format_args!("{}", lambda)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionsBuffer<'a>(pub &'a [Expression<'a>]);
impl std::fmt::Display for ExpressionsBuffer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Expressions: [")?;
        f.write_fmt(format_args!(
            "{}",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        ))?;
        f.write_str("]\n")
    }
}

// Term
#[derive(Debug, Clone, PartialEq)]
pub struct TermExpr<'a> {
    pub pre_delimiters: Option<&'a CommentedToken<'a>>,
    pub term: Vec<Expression<'a>>,
    pub post_delimiters: Option<&'a CommentedToken<'a>>,
}

impl<'a> TermExpr<'a> {
    pub fn new(
        pre_delimiters: Option<&'a CommentedToken<'a>>,
        term: Vec<Expression<'a>>,
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
        Self::new(None, vec![expr], None)
    }
}

impl std::fmt::Display for TermExpr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(TermExpr: {} {} {})",
            if let Some(pre_delim) = self.pre_delimiters {
                pre_delim.to_string()
            } else {
                "".to_string()
            },
            self.term
                .iter()
                .map(|e| format!("(expr: {})", e))
                .collect::<Vec<_>>()
                .join(" "),
            if let Some(post_delim) = self.post_delimiters {
                post_delim.to_string()
            } else {
                "".to_string()
            },
        ))
    }
}

// Function definition
// The comma is required due to the way the parser treats comments
// The formatter needs comments and some of them might end up squeezed into
// the comma token
#[derive(Debug, Clone, PartialEq)]
pub struct Arg<'a>(pub Expression<'a>, pub Option<Expression<'a>>); // Argument, comma

impl std::fmt::Display for Arg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))?;
        if let Some(comma) = &self.1 {
            f.write_fmt(format_args!("comma:{}", comma))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter<'a> {
    Paren(&'a CommentedToken<'a>),
    SingleBracket(&'a CommentedToken<'a>),
    DoubleBracket((&'a CommentedToken<'a>, &'a CommentedToken<'a>)),
}

impl std::fmt::Display for Delimiter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Delimiter::Paren(single) | Delimiter::SingleBracket(single) => {
                f.write_fmt(format_args!("{}", single))
            }
            Delimiter::DoubleBracket((b1, b2)) => f.write_fmt(format_args!("{}{}", b1, b2)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Args<'a> {
    pub left_delimeter: Delimiter<'a>,
    pub args: Vec<Arg<'a>>,
    pub right_delimeter: Delimiter<'a>,
}

impl<'a> Args<'a> {
    pub fn new(
        left_delimeter: Delimiter<'a>,
        args: Vec<Arg<'a>>,
        right_delimeter: Delimiter<'a>,
    ) -> Self {
        Self {
            left_delimeter,
            args,
            right_delimeter,
        }
    }
}

impl std::fmt::Display for Args<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "(Args: {} {} {})",
            self.left_delimeter,
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.right_delimeter,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition<'a> {
    pub keyword: &'a CommentedToken<'a>,
    pub arguments: Args<'a>,
    pub body: Box<Expression<'a>>,
}

impl<'a> FunctionDefinition<'a> {
    pub fn new(
        keyword: &'a CommentedToken<'a>,
        arguments: Args<'a>,
        body: Box<Expression<'a>>,
    ) -> Self {
        Self {
            keyword,
            arguments,
            body,
        }
    }
}

impl std::fmt::Display for FunctionDefinition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.keyword, self.arguments, self.body
        ))
    }
}

// If expression
#[derive(Debug, Clone, PartialEq)]
pub struct IfConditional<'a> {
    pub keyword: &'a CommentedToken<'a>,
    pub left_delimiter: &'a CommentedToken<'a>,
    pub condition: Box<Expression<'a>>,
    pub right_delimiter: &'a CommentedToken<'a>,
    pub body: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfConditional<'a> {
    pub else_keyword: &'a CommentedToken<'a>,
    pub if_conditional: IfConditional<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrailingElse<'a> {
    pub else_keyword: &'a CommentedToken<'a>,
    pub body: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression<'a> {
    pub if_conditional: IfConditional<'a>,
    pub else_ifs: Vec<ElseIfConditional<'a>>,
    pub trailing_else: Option<TrailingElse<'a>>,
}

impl std::fmt::Display for TrailingElse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.else_keyword, self.body))
    }
}

impl std::fmt::Display for IfConditional<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {} {} {}",
            self.keyword, self.left_delimiter, self.condition, self.right_delimiter, self.body
        ))
    }
}

impl std::fmt::Display for ElseIfConditional<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            self.else_keyword, self.if_conditional
        ))
    }
}

impl std::fmt::Display for IfExpression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ", self.if_conditional))?;
        for else_if in &self.else_ifs {
            f.write_fmt(format_args!("{}", else_if))?;
        }
        match &self.trailing_else {
            Some(trailing_else) => f.write_fmt(format_args!("{}", trailing_else)),
            None => Ok(()),
        }
    }
}

// While expression
#[derive(Debug, Clone, PartialEq)]
pub struct WhileExpression<'a> {
    pub while_keyword: &'a CommentedToken<'a>,
    pub condition: Box<Expression<'a>>,
    pub body: Box<Expression<'a>>,
}

impl std::fmt::Display for WhileExpression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({} {} {})",
            self.while_keyword, self.condition, self.body
        ))
    }
}

// Repeat expresssion
#[derive(Debug, Clone, PartialEq)]
pub struct RepeatExpression<'a> {
    pub repeat_keyword: &'a CommentedToken<'a>,
    pub body: Box<Expression<'a>>,
}

impl std::fmt::Display for RepeatExpression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} {})", self.repeat_keyword, self.body))
    }
}

// Function call
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall<'a> {
    pub function_ref: Box<Expression<'a>>,
    pub args: Args<'a>,
}

impl std::fmt::Display for FunctionCall<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Call({} {})", self.function_ref, self.args))
    }
}

// Subset expression
#[derive(Debug, Clone, PartialEq)]
pub struct SubsetExpression<'a> {
    pub object_ref: Box<Expression<'a>>,
    pub args: Args<'a>,
}

impl std::fmt::Display for SubsetExpression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.object_ref, self.args))
    }
}

// For loop
#[derive(Debug, Clone, PartialEq)]
pub struct ForLoop<'a> {
    pub keyword: &'a CommentedToken<'a>,
    pub left_delim: Delimiter<'a>,
    pub identifier: Box<Expression<'a>>,
    pub in_keyword: &'a CommentedToken<'a>,
    pub collection: Box<Expression<'a>>,
    pub right_delim: Delimiter<'a>,
    pub body: Box<Expression<'a>>,
}

impl std::fmt::Display for ForLoop<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "<{} {} {} {} {} {} {}>",
            self.keyword,
            self.left_delim,
            self.identifier,
            self.in_keyword,
            self.collection,
            self.right_delim,
            self.body
        ))
    }
}

// Lambda
#[derive(Debug, Clone, PartialEq)]
pub struct Lambda<'a> {
    pub keyword: &'a CommentedToken<'a>,
    pub args: Args<'a>,
    pub body: Box<Expression<'a>>,
}

impl std::fmt::Display for Lambda<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.keyword, self.args, self.body))
    }
}
