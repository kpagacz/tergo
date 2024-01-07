use tokenizer::tokenizer::Tokenizer;
use tokenizer::tokens::Token;

#[test]
fn symbols() {
    let examples = [
        ("TRUE", vec![Token::Symbol("TRUE")]),
        (
            "TRUE\nTRUE",
            vec![Token::Symbol("TRUE"), Token::Newline, Token::Symbol("TRUE")],
        ),
    ];
    for (example, expected_tokens) in examples {
        let mut tokenizer = Tokenizer::new(example);
        let tokens = tokenizer.tokenize();
        let tokens = tokens
            .into_iter()
            .map(|token| token.token)
            .collect::<Vec<_>>();
        assert_eq!(tokens, expected_tokens);
    }
}

#[test]
fn comments() {
    let examples = [
        (
            "# Comment\n",
            vec![Token::Comment("# Comment"), Token::Newline],
        ),
        (
            "TRUE#Comment",
            vec![Token::Symbol("TRUE"), Token::InlineComment("#Comment")],
        ),
    ];
    for (example, expected) in examples {
        let mut tokenizer = Tokenizer::new(example);
        let tokens = tokenizer.tokenize();
        let tokens = tokens
            .into_iter()
            .map(|token| token.token)
            .collect::<Vec<_>>();
        assert_eq!(tokens, expected);
    }
}

#[test]
fn ifs() {
    let examples = [(
        "if (TRUE) TRUE else FALSE",
        vec![
            Token::If,
            Token::LParen,
            Token::Symbol("TRUE"),
            Token::RParen,
            Token::Symbol("TRUE"),
            Token::Else,
            Token::Symbol("FALSE"),
        ],
    )];
    for (example, expected) in examples {
        let mut tokenizer = Tokenizer::new(example);
        let tokens = tokenizer.tokenize();
        let tokens = tokens
            .into_iter()
            .map(|token| token.token)
            .collect::<Vec<_>>();
        assert_eq!(tokens, expected);
    }
}

#[test]
fn number_literals() {
    let examples = [
        ("123", vec![Token::Literal("123")]),
        ("123.0", vec![Token::Literal("123.0")]),
        (".42e42", vec![Token::Literal(".42e42")]),
        ("1e-10", vec![Token::Literal("1e-10")]),
        ("1e+10", vec![Token::Literal("1e+10")]),
        ("1e10", vec![Token::Literal("1e10")]),
        ("0xabcdef", vec![Token::Literal("0xabcdef")]),
        ("0xabcdef.1P28", vec![Token::Literal("0xabcdef.1P28")]),
    ];
    for (example, expected) in examples {
        let mut tokenizer = Tokenizer::new(example);
        let tokens = tokenizer.tokenize();
        let tokens = tokens
            .into_iter()
            .map(|token| token.token)
            .collect::<Vec<_>>();
        assert_eq!(tokens, expected);
    }
}

#[test]
fn binary_ops() {
    let examples = [
        (
            "1+1",
            vec![Token::Literal("1"), Token::Plus, Token::Literal("1")],
        ),
        (
            "1+1-1",
            vec![
                Token::Literal("1"),
                Token::Plus,
                Token::Literal("1"),
                Token::Minus,
                Token::Literal("1"),
            ],
        ),
        (
            "1+1*1",
            vec![
                Token::Literal("1"),
                Token::Plus,
                Token::Literal("1"),
                Token::Multiply,
                Token::Literal("1"),
            ],
        ),
        (
            "1+1/1",
            vec![
                Token::Literal("1"),
                Token::Plus,
                Token::Literal("1"),
                Token::Divide,
                Token::Literal("1"),
            ],
        ),
        (
            "1+1^1",
            vec![
                Token::Literal("1"),
                Token::Plus,
                Token::Literal("1"),
                Token::Power,
                Token::Literal("1"),
            ],
        ),
        (
            "1+1%%1",
            vec![
                Token::Literal("1"),
                Token::Plus,
                Token::Literal("1"),
                Token::Modulo,
                Token::Literal("1"),
            ],
        ),
    ];
    for (example, expected) in examples {
        let mut tokenizer = Tokenizer::new(example);
        let tokens = tokenizer.tokenize();
        let tokens = tokens
            .into_iter()
            .map(|token| token.token)
            .collect::<Vec<_>>();
        assert_eq!(tokens, expected);
    }
}

#[test]
fn function_definitions() {
    let examples = [
        (
            "function() TRUE",
            vec![
                Token::Function,
                Token::LParen,
                Token::RParen,
                Token::Symbol("TRUE"),
            ],
        ),
        (
            "function(x) TRUE",
            vec![
                Token::Function,
                Token::LParen,
                Token::Symbol("x"),
                Token::RParen,
                Token::Symbol("TRUE"),
            ],
        ),
        (
            "function(x, y) TRUE",
            vec![
                Token::Function,
                Token::LParen,
                Token::Symbol("x"),
                Token::Comma,
                Token::Symbol("y"),
                Token::RParen,
                Token::Symbol("TRUE"),
            ],
        ),
        (
            "function(x, y, z) TRUE",
            vec![
                Token::Function,
                Token::LParen,
                Token::Symbol("x"),
                Token::Comma,
                Token::Symbol("y"),
                Token::Comma,
                Token::Symbol("z"),
                Token::RParen,
                Token::Symbol("TRUE"),
            ],
        ),
        (
            "function(x, y, z, ...) TRUE",
            vec![
                Token::Function,
                Token::LParen,
                Token::Symbol("x"),
                Token::Comma,
                Token::Symbol("y"),
                Token::Comma,
                Token::Symbol("z"),
                Token::Comma,
                Token::Symbol("..."),
                Token::RParen,
                Token::Symbol("TRUE"),
            ],
        ),
    ];

    for (example, expected) in examples {
        let mut tokenizer = Tokenizer::new(example);
        let tokens = tokenizer.tokenize();
        let tokens = tokens
            .into_iter()
            .map(|token| token.token)
            .collect::<Vec<_>>();
        assert_eq!(tokens, expected);
    }

    // Default argument values
    let examples = [
        (
            "function(x=1) {}",
            vec![
                Token::Function,
                Token::LParen,
                Token::Symbol("x"),
                Token::OldAssign,
                Token::Literal("1"),
                Token::RParen,
                Token::LBrace,
                Token::RBrace,
            ],
        ),
        (
            "function(x=1, y=2) {}",
            vec![
                Token::Function,
                Token::LParen,
                Token::Symbol("x"),
                Token::OldAssign,
                Token::Literal("1"),
                Token::Comma,
                Token::Symbol("y"),
                Token::OldAssign,
                Token::Literal("2"),
                Token::RParen,
                Token::LBrace,
                Token::RBrace,
            ],
        ),
    ];

    for (example, expected) in examples {
        let mut tokenizer = Tokenizer::new(example);
        let tokens = tokenizer
            .tokenize()
            .into_iter()
            .map(|token| token.token)
            .collect::<Vec<_>>();
        assert_eq!(tokens, expected);
    }
}
