use tokenizer::tokens::CommentedToken;

#[macro_export]
macro_rules! commented_tokens {
    ($($args:expr),*) => {{
        vec![
        $(
            CommentedToken::new($args, 0, 0),
        )*
        ]
    }}
}
pub use commented_tokens;

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizer::Token::*;

    #[test]
    fn commented_tokens_macro() {
        let tokens = commented_tokens![Symbol("a"), InlineComment("# Comment")];
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Symbol("a"));
        assert_eq!(tokens[1].token, InlineComment("# Comment"));
    }
}
