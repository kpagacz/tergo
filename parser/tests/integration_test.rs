use parser::ast::{Expression, TermExpr};
use parser::{parse, pre_parse};
use tokenizer::Tokenizer;

fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_cases() {
    log_init();
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

#[test]
fn literal_with_parentheses() {
    log_init();
    let code = include_str!("./test_cases/002.R");
    let mut tokenizer = Tokenizer::new(code);
    let mut commented_tokens = tokenizer.tokenize();

    let tokens = pre_parse(&mut commented_tokens);
    let res = parse(&tokens).unwrap();
    let expected = vec![
        Expression::Term(Box::new(TermExpr {
            pre_delimiters: Some(tokens[0]),
            term: Expression::Literal(tokens[1]),
            post_delimiters: Some(tokens[2]),
        })),
        Expression::EOF(tokens[4]),
    ];
    assert_eq!(res, expected);
}
