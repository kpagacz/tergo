use crate::ast::CommentedToken;
use tokenizer::LocatedToken;

macro_rules! located_tokens {
    ($($args:expr),*) => {{
        vec![
        $(
            LocatedToken::new($args, 0, 0),
        )*
        ]
    }}
}
pub(crate) use located_tokens;

pub(crate) fn commented_tokens<'a>(
    located_tokens: &'a [LocatedToken<'a>],
) -> Vec<CommentedToken<'a>> {
    located_tokens
        .iter()
        .map(|token| CommentedToken::new(token, &[], None))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizer::Token::*;
    #[test]
    fn commented_tokens_macro() {
        let located_tokens = located_tokens![Symbol("a"), InlineComment("# Comment")];
        let tokens = commented_tokens(&located_tokens);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token.token, Symbol("a"));
        assert_eq!(tokens[1].token.token, InlineComment("# Comment"));
    }
}
