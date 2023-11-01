#[derive(Debug, PartialEq)]
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
    And, // &
    Or,  // |
    // Model formulae
    ModelFormulae, // ~
    // Assignment
    // Assignment,      // <-
    // RightAssignment, // ->
    // OldAssignment,   // =
    // List indexing
    Dollar, // $
    // Sequence
    Colon, // :
    // Infix
    Infix(String), // %chars%
}

#[derive(Debug, PartialEq)]
pub enum Uop {
    Plus,  // +
    Minus, // -
    Not,   // !
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Call(Box<Expression>, Vec<Argument>), // expr(arguments)
    Uop(Uop, Box<Expression>),            // Uop expr
    Bop(Bop, Box<Expression>, Box<Expression>), // lhs Bop rhs
    MultiBop(Box<Expression>, Vec<(Bop, Box<Expression>)>), // lhs (Bop rhs)+
}

/// There are five types of constants: integer, logical, numeric, complex and string.
/// In addition, there are four special constants, NULL, NA, Inf, and NaN.
#[derive(Debug, PartialEq)]
pub enum Literal {
    // TODO: Add some of the reserved literals
    // NA NA_integer_ NA_real_ NA_complex_ NA_character_
    // ... ..1 ..2 etc.
    True,  // TRUE
    False, // FALSE
    Null,  // NULL
    Na,    // NA
    Inf,   // Inf
    NaN,   // Nan
    String(String),
    Number(String),
    Integer(String),
    Complex(String),
}

#[derive(Debug)]
pub enum Statement {
    Library(String), // library(package)
    Break,           // break
    Next,            // next
    Expressions(Vec<Expression>),
    Assignment(Vec<Expression>, Vec<Vec<Expression>>), // lhs <- rhs1 <- rhs2
    Compound(CompoundStatement),
}

#[derive(Debug)]
pub enum CompoundStatement {
    If(
        Vec<(Box<Statement>, Vec<Statement>)>,
        Option<Vec<Statement>>,
    ),
    Repeat(Box<Statement>),
    While(Box<Statement>, Box<Statement>),
    For(String, Box<Expression>, Box<Statement>),
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Named(String, Expression),
    Positional(Expression),
}
