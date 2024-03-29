use parser::parse;
use tokenizer::Tokenizer;

#[test]
fn test_cases() {
    let code = include_str!("./test_cases/001.R");
    let mut tokenizer = Tokenizer::new(code);
    let commented_tokens = tokenizer.tokenize();

    let res = parse(&commented_tokens).unwrap();
    let expected = vec![
        parser::ast::Expression::Literal(&commented_tokens[0]),
        parser::ast::Expression::Whitespace(&commented_tokens[1..2]),
        parser::ast::Expression::EOF(&commented_tokens[3]),
    ];
    assert_eq!(res, expected);
}
