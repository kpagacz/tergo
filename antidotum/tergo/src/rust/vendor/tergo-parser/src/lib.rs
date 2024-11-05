pub mod ast;
pub(crate) mod compound;
pub(crate) mod expressions;
pub mod parser;
pub(crate) mod pre_parsing_hooks;
pub use parser::parse;
pub use pre_parsing_hooks::pre_parse;
use tokenizer::tokens::CommentedToken;
pub(crate) mod program;
pub(crate) mod token_parsers;
pub(crate) mod whitespace;

type Input<'a, 'b> = &'b [&'a CommentedToken<'a>];
