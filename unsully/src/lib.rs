use formatter::format_code;
use parser::ast::CommentedToken;
use parser::parse;
use tokenizer::Tokenizer;

pub fn format(input: &str) -> Result<String, String> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let commented_tokens = tokens.iter().map(CommentedToken::from).collect::<Vec<_>>();
    eprintln!("tokens:\n{:?}", commented_tokens);
    let cst = parse(&commented_tokens)?;
    eprintln!("cst:\n{:?}", cst);
    Ok(format_code(&cst))
}
