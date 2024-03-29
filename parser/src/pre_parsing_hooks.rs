use tokenizer::tokens::CommentedToken;
use tokenizer::tokens::Token;

/// This function aims to squeeze the comments into the tokens, so the
/// parser doesn't have to worry about comments.
///
/// It achieves this by attaching all the comments that precede
/// a token, and the inline comments that follow a token to the token itself.
/// Thus, all the comments are attached to non-comment tokens.
/// The comments are then unfurled in the formatting stage.
pub fn pre_parse<'a>(tokens: &'a mut [CommentedToken<'a>]) -> Vec<&'a CommentedToken<'a>> {
    let mut it = 0;
    let mut tokens_without_comments = vec![];
    while it < tokens.len() {
        if let Token::Comment(_) = tokens[it].token {
            let start = it;
            while matches!(tokens[it].token, Token::Comment(_))
                || matches!(tokens[it].token, Token::Newline)
            {
                it += 1;
            }

            tokens[it].leading_comments = Some((start, it));
            tokens_without_comments.push(it);
        } else if let Token::InlineComment(_) = tokens[it].token {
            tokens[it - 1].inline_comment = Some(it);
        } else {
            tokens_without_comments.push(it);
        }
        it += 1;
    }

    tokens_without_comments
        .into_iter()
        .map(|id| &tokens[id])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizer::tokens::commented_tokens;

    #[test]
    fn test_pre_parse() {
        let mut tokens = commented_tokens![
            Token::Comment("Comment"),
            Token::Newline,
            Token::Symbol("7"),
            Token::InlineComment("Inline comment")
        ];
        let commented_tokens = pre_parse(&mut tokens);
        assert!(commented_tokens.len() == 1);
        let res_token = commented_tokens[0];

        // Comments
        assert_eq!(
            res_token.leading_comments,
            Some((0, 2)),
            "The length of the leading comments does not match"
        );

        // Inlined comments
        assert!(res_token.inline_comment.is_some());
        assert!(matches!(res_token.inline_comment.unwrap(), 3));
    }
}
