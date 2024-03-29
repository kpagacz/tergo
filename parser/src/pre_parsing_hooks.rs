// use tokenizer::tokens::CommentedToken;
// use tokenizer::Token;
//
// /// This function aims to squeeze the comments into the tokens, so the
// /// parser doesn't have to worry about comments.
// ///
// /// It achieves this by attaching all the comments that precede
// /// a token, and the inline comments that follow a token to the token itself.
// /// Thus, all the comments are attached to non-comment tokens.
// /// The comments are then unfurled in the formatting stage.
// pub fn pre_parse<'a, 'b>(tokens: &'a [CommentedToken<'b>]) -> Vec<CommentedToken<'b>> {
//     let mut commented_tokens = Vec::new();
//
//     let mut it = 0;
//     while it < tokens.len() {
//         match tokens[it].token {
//             Token::Comment(_) => {
//                 let comments_start = it;
//                 while matches!(tokens[it].token, Token::Comment(_))
//                     || matches!(tokens[it].token, Token::Newline)
//                 {
//                     it += 1;
//                 }
//                 let commented_token = if it + 1 < tokens.len()
//                     && matches!(tokens[it + 1].token, Token::InlineComment(_))
//                 {
//                     it += 1;
//                     CommentedToken::new(
//                         &tokens[it],
//                         &tokens[comments_start..it - 1],
//                         Some(&tokens[it]),
//                     )
//                 } else {
//                     CommentedToken::new(&tokens[it], &tokens[comments_start..it], None)
//                 };
//                 commented_tokens.push(commented_token);
//             }
//             _ => {
//                 let commented_token = if it + 1 < tokens.len()
//                     && matches!(tokens[it + 1].token, Token::InlineComment(_))
//                 {
//                     it += 1;
//                     CommentedToken::new(&tokens[it], &[], Some(&tokens[it]))
//                 } else {
//                     CommentedToken::new(&tokens[it], &[], None)
//                 };
//                 commented_tokens.push(commented_token);
//             }
//         }
//         it += 1;
//     }
//
//     commented_tokens
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_pre_parse() {
//         let tokens = vec![
//             LocatedToken::new(Token::Comment("Comment"), 0, 0),
//             LocatedToken::new(Token::Newline, 0, 0),
//             LocatedToken::new(Token::Symbol("7"), 0, 0),
//             LocatedToken::new(Token::InlineComment("Inline comment"), 0, 0),
//         ];
//         let commented_tokens = pre_parse(&tokens);
//         assert!(commented_tokens.len() == 1);
//         let res_token = &commented_tokens[0];
//
//         // Comments
//         assert_eq!(
//             res_token.leading_comments.len(),
//             2,
//             "The length of the leading comments does not match"
//         );
//         assert!(matches!(
//             res_token.leading_comments[0].token,
//             Token::Comment(_)
//         ));
//
//         // Inlined comments
//         assert!(res_token.inline_comment.is_some());
//         assert!(matches!(
//             res_token.inline_comment.unwrap().token,
//             Token::InlineComment(_)
//         ));
//     }
// }
