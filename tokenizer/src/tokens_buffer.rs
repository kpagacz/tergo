use crate::tokens::CommentedToken;

/// Wrapper for a buffer of tokens used for pretty printing.
#[derive(Debug)]
pub struct TokensBuffer<'a>(pub &'a [&'a CommentedToken<'a>]);

impl<'a> std::fmt::Display for TokensBuffer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.split_first() {
            None => Ok(()),
            Some((first, rest)) => {
                f.write_fmt(format_args!("{:?}", first.token))?;
                for token in rest {
                    write!(f, " {:?}", token.token)?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Token;

    use super::*;

    #[test]
    fn test_displaying_tokens() {
        let tokens = [
            CommentedToken::new(Token::Symbol("a"), 1, 0),
            CommentedToken::new(Token::Symbol("b"), 1, 1),
        ];
        let displayed = format!("{}", TokensBuffer(&[&tokens[0], &tokens[1]]));
        assert_eq!("Symbol(\"a\") Symbol(\"b\")", displayed);
    }
}
