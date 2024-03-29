use formatter::format_code;
use parser::parse;
use tokenizer::Tokenizer;

pub fn format(input: &str) -> Result<String, String> {
    let mut tokenizer = Tokenizer::new(input);
    let commented_tokens = tokenizer.tokenize();
    eprintln!("tokens:\n{:?}", commented_tokens);
    let cst = parse(&commented_tokens)?;
    eprintln!("cst:\n{:?}", cst);
    Ok(format_code(&cst))
}
