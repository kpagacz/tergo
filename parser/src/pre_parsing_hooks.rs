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
        if let Token::Comment(comment) = tokens[it].token {
            let mut comments = vec![comment];
            it += 1;
            loop {
                match tokens[it].token {
                    Token::Newline => {
                        if matches!(tokens[it - 1].token, Token::Newline) {
                            comments.push("");
                        }
                    }
                    Token::Comment(comment) => comments.push(comment),
                    _ => break,
                }
                it += 1;
            }
            tokens[it].leading_comments = Some(comments);
            tokens_without_comments.push(it);
        } else if let Token::InlineComment(comment) = tokens[it].token {
            tokens[it - 1].inline_comment = Some(comment);
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
            Some(vec!["Comment"]),
            "The length of the leading comments does not match"
        );

        // Inlined comments
        assert!(res_token.inline_comment.is_some());
        assert!(matches!(
            res_token.inline_comment.unwrap(),
            "Inline comment"
        ));
    }
}
