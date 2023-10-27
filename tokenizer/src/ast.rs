#[derive(Debug)]
pub enum Bop {
    Plus,          // +
    Minus,         // -
    Multiply,      // *
    Divide,        // /
    Assignment,    // <-
    OldAssignment, // =
    Equal,         // ==
    Ge,            // >=
    Le,            // <=
    Greater,       // >
    Lower,         // <
}

#[derive(Debug)]
pub enum Uop {
    Plus,  // +
    Minus, // -
    Not,   // !
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    True,  // TRUE
    False, // FALSE
    Null,  // NULL
    StringLiteral(String),
    IntegerLiteral(String),
    NumberLiteral(String),
    ComplexLiteral(String),
    Identifier(String),
    Call(Box<Expression>, Vec<Argument>), // expr(arguments)
}

#[derive(Debug)]
pub enum Statement {
    Assignment(Vec<Expression>, Vec<Vec<Expression>>), // lhs <- rhs1 <- rhs2
    Bop(Bop, Box<Expression>, Box<Expression>),        // lhs Bop rhs
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Named(String, Expression),
    Positional(Expression),
}
