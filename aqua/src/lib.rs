pub mod tokenizer;
pub mod tokens;
pub use tokenizer::Tokenizer;
pub use tokens::Token;

#[derive(Debug)]
pub enum Error {
    UnexpectedCharacter(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::UnexpectedCharacter(context) => out.write_fmt(format_args!("{context})")),
        }
    }
}

impl std::error::Error for Error {}
