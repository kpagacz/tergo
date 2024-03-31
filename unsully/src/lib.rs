pub mod config;
use config::Config;
use formatter::format_code;
use parser::{parse, pre_parse};
use tokenizer::Tokenizer;

pub fn format(input: &str, config: Option<Config>) -> Result<String, String> {
    let config = config.unwrap_or_default();
    let mut tokenizer = Tokenizer::new(input);
    let mut commented_tokens = tokenizer.tokenize();
    eprintln!("tokens:\n{:?}", commented_tokens);
    let tokens_without_comments = pre_parse(&mut commented_tokens);
    let cst = parse(&tokens_without_comments)?;
    eprintln!("cst:\n{:?}", cst);
    Ok(format_code(&cst, &config))
}
