pub use formatter::config::Config;
pub use formatter::config::FunctionLineBreaks;
use formatter::format_code;
use log::trace;
use parser::{
    ast::{Expression, TermExpr},
    parse, pre_parse,
};
use tokenizer::{tokens_buffer::TokensBuffer, Tokenizer};

/// Format the input code with the given configuration.
///
/// # Arguments
///
/// * `input` - The input code to format.
/// * `config` - The configuration to use for formatting.
///   If not provided, the default configuration will be used.
///   An instance of [Config].
///
/// # Returns
///
/// The formatted code.
///
/// # Example
///
/// ```rust
/// use tergo_lib::tergo_format;
/// use tergo_lib::Config;
///
/// let input = "a <- function(x, y){x+y}";
/// let config = Config::default();
///
/// let formatted = tergo_format(input, Some(&config)).unwrap();
/// ```
pub fn tergo_format(input: &str, config: Option<&Config>) -> Result<String, String> {
    let default_config = Config::default();
    let config = config.unwrap_or(&default_config);
    trace!("Formatting with config: {config}");
    let mut tokenizer = Tokenizer::new(input);
    trace!("Tokenizer created");
    let mut commented_tokens = tokenizer.tokenize();
    trace!("Tokens with comments: {commented_tokens:?}",);
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
