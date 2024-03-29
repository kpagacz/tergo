use parser::{parse, pre_parse};
use tokenizer::Tokenizer;

#[test]
fn test_cases() {
    let code = include_str!("./test_cases/001.R");
    let mut tokenizer = Tokenizer::new(code);
    let mut commented_tokens = tokenizer.tokenize();

    let tokens = pre_parse(&mut commented_tokens);
    let res = parse(&tokens).unwrap();
    let expected = vec![
        parser::ast::Expression::Literal(tokens[0]),
        parser::ast::Expression::Whitespace(&tokens[1..2]),
        parser::ast::Expression::EOF(tokens[3]),
    ];
    assert_eq!(res, expected);
}
