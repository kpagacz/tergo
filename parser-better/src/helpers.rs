use crate::ast::CommentedToken;
use tokenizer::LocatedToken;

#[macro_export]
macro_rules! commented_tokens {
($($args:expr),*) => {{
      vec![
      $(
          CommentedToken::new(LocatedToken::new($args, 0, 0), &[], None),
      )*
    ]
}}
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizer::Token::*;
    #[test]
    fn commented_tokens_macro() {
        let tokens = commented_tokens![Symbol("a"), InlineComment("# Comment")];
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token.token, Symbol("a"));
        assert_eq!(tokens[1].token.token, InlineComment("# Comment"));
    }
}
