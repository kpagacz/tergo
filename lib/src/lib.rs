pub mod config;
use config::Config;
use formatter::format_code;
use log::trace;
use parser::{
    ast::{Expression, TermExpr},
    parse, pre_parse,
};
use tokenizer::{tokens_buffer::TokensBuffer, Tokenizer};

pub fn tergo_format(input: &str, config: Option<&Config>) -> Result<String, String> {
    let default_config = Config::default();
    let config = config.unwrap_or(&default_config);
    trace!("Formatting with config: {config}");
    let mut tokenizer = Tokenizer::new(input);
    let mut commented_tokens = tokenizer.tokenize();
    let tokens_without_comments = pre_parse(&mut commented_tokens);
    trace!(
        "Tokens without comments: {}",
        TokensBuffer(&tokens_without_comments)
    );
    let cst = parse(&tokens_without_comments)?;
    let top_node = Expression::Term(Box::new(TermExpr::new(None, cst, None)));
    trace!("CST: {:?}", top_node);
    Ok(format_code(top_node, config))
}
