pub mod config;
use config::Config;
use formatter::format_code;
use log::trace;
use parser::{parse, pre_parse};
use tokenizer::{tokens_buffer::TokensBuffer, Tokenizer};

pub fn format(input: &str, config: Option<Config>) -> Result<String, String> {
    let config = config.unwrap_or_default();
    trace!("Formatting with config: {config}");
    let mut tokenizer = Tokenizer::new(input);
    let mut commented_tokens = tokenizer.tokenize();
    let tokens_without_comments = pre_parse(&mut commented_tokens);
    trace!(
        "Tokens without comments: {}",
        TokensBuffer(&tokens_without_comments)
    );
    let cst = parse(&tokens_without_comments)?;
    trace!("CST: {:?}", cst);
    Ok(format_code(&cst, &config))
}
