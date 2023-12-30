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
    // List indexing
    Dollar, // $
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

/// There are five types of constants: integer, logical, numeric, complex and string.
/// In addition, there are four special constants, NULL, NA, Inf, and NaN.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    // TODO: Add some of the reserved literals
    // NA NA_integer_ NA_real_ NA_complex_ NA_character_
    // ... ..1 ..2 etc.
    True,        // TRUE
    False,       // FALSE
    Null,        // NULL
    Na,          // NA
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
    Call(Box<Expression>, Vec<Argument>), // expr(arguments)
    Subscript(Box<Expression>, Vec<Argument>, SubscriptType), // expr [[ sublist ]] | expr [ sublist ]
    Uop(Uop, Box<Expression>),                                // Uop expr
    Bop(Bop, Box<Expression>, Box<Expression>),               // lhs Bop rhs
    MultiBop(Box<Expression>, Vec<(Bop, Expression)>),        // lhs (Bop rhs)+
    // In R if statements evaluate to value, like an ordinary ternary
    // operator in other languages, so here we go...
    If(
        Vec<(Box<Expression>, Box<Expression>)>,
        Option<Box<Expression>>,
    ),
    Function(FunctionDefinition), // function(args list) definition
    Block(Vec<Expression>),
    Assignment(Box<Expression>, Box<Expression>),
    Library(String), // library(package)
    Break,           // break
    Next,            // next
    Expressions(Vec<Expression>),
    Compound(CompoundStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompoundStatement {
    Repeat(Box<Expression>),
    While(Box<Expression>, Box<Expression>),
    For(String, Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
    Named(Box<Expression>, Box<Expression>),
    Positional(Box<Expression>),
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SubscriptType {
    Single,
    Double,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition {
    pub arg_names: Vec<Expression>,
    pub arg_values: Vec<Option<Expression>>,
    pub body: Box<Expression>,
}
