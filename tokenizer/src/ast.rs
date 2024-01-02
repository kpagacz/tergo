#[derive(Debug, PartialEq, Clone)]
pub enum Bop {
    // Arithmetic
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %%
    Power,    // ^
    // Relational
    Greater,  // >
    Ge,       // >=
    Lower,    // <
    Le,       // <=
    Equal,    // ==
    NotEqual, // !=
    // Logical
    And,  // &
    And2, // &&
    Or,   // |
    Or2,  // ||
    // Model formulae
    ModelFormulae, // ~
    // Assignment
    Assignment,      // <-
    RightAssignment, // ->
    OldAssignment,   // =
    // Sequence
    Colon, // :
    // Infix
    Infix(String), // %chars% // TODO add support for infix
    Pipe,          // |>
    // Namespaces
    NsGet,    // ::
    NsGetInt, // :::
    // Help
    Questionmark, // ?
}

#[derive(Debug, PartialEq, Clone)]
pub enum Uop {
    Plus,         // +
    Minus,        // -
    Not,          // !
    Questionmark, // ?
}

#[derive(Debug, Clone, PartialEq)]
pub enum Na {
    Generic,
    Integer,
    Real,
    Complex,
    Character,
}

/// There are five types of constants: integer, logical, numeric, complex and string.
/// In addition, there are four special constants, NULL, NA, Inf, and NaN.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    True,        // TRUE
    False,       // FALSE
    Null,        // NULL
    Na(Na),      // NA
    Inf,         // Inf
    NaN,         // Nan
    Placeholder, // _
    ThreeDots,   // ...
    String(String),
    Number(String),
    Integer(String),
    Complex(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Call(AstNode, Vec<Argument>),                     // expr(arguments)
    Subscript(AstNode, Vec<Argument>, SubscriptType), // expr [[ sublist ]] | expr [ sublist ]
    Uop(Uop, AstNode),                                // Uop expr
    Bop(Bop, AstNode, AstNode),                       // lhs Bop rhs
    MultiBop(AstNode, Vec<(Bop, AstNode)>),           // lhs (Bop rhs)+
    // In R if statements evaluate to value, like an ordinary ternary
    // operator in other languages, so here we go...
    If(Vec<(AstNode, AstNode)>, Option<AstNode>),
    Function(FunctionDefinition), // function(args list) definition
    Block(Vec<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
    Library(String), // library(package)
    Break,           // break
    Next,            // next
    Expressions(Vec<AstNode>),
    Compound(CompoundStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompoundStatement {
    Repeat(AstNode),
    While(AstNode, AstNode),
    For(AstNode, AstNode, AstNode),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    Named(AstNode, AstNode),
    Positional(AstNode),
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SubscriptType {
    Single,
    Double,
    Dollar,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionDefinitionType {
    Default,
    Lambda,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition {
    pub arg_names: Vec<AstNode>,
    pub arg_values: Vec<Option<AstNode>>,
    pub body: AstNode,
    pub def_type: FunctionDefinitionType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    pub expr: Box<Expression>,
    pub leading_comment: Option<String>,
    pub trailing_comment: Option<String>,
}

impl AstNode {
    pub fn new(expr: Box<Expression>) -> Self {
        Self {
            expr,
            leading_comment: None,
            trailing_comment: None,
        }
    }
}
