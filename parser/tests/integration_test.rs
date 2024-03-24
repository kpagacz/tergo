use parser::{ast::CommentedToken, parse};
use tokenizer::Tokenizer;

#[test]
fn test_cases() {
    let code = include_str!("./test_cases/001.R");
    let mut tokenizer = Tokenizer::new(code);
    let tokens = tokenizer.tokenize();
    let commented_tokens = tokens.iter().map(CommentedToken::from).collect::<Vec<_>>();

    let res = parse(&commented_tokens).unwrap();
    let expected = vec![
        parser::ast::Expression::Literal(&commented_tokens[0]),
        parser::ast::Expression::Newline(&commented_tokens[1]),
        parser::ast::Expression::EOF(&commented_tokens[3]),
    ];
    assert_eq!(res, expected);
}
