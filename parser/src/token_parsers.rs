use crate::Input;
use nom::IResult;
use tokenizer::tokens::CommentedToken;
use tokenizer::Token::*;

macro_rules! token_parser {
    ($name:ident, $token:pat) => {
        pub(crate) fn $name<'a, 'b: 'a>(
            input: Input<'a, 'b>,
        ) -> IResult<Input<'a, 'b>, &'b CommentedToken<'a>> {
            log::trace!("Trying to parse: {}", stringify!($name));
            match input {
                [token @ CommentedToken { token: $token, .. }, rest @ ..] => Ok((rest, token)),
                _ => Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                ))),
            }
        }
    };
}

token_parser!(symbol, Symbol(_));
token_parser!(literal, Literal(_));
token_parser!(semicolon, Semicolon);
token_parser!(newline, Newline);
token_parser!(lparen, LParen);
token_parser!(rparen, RParen);
token_parser!(lbrace, LBrace);
token_parser!(rbrace, RBrace);
// token_parser!(lsubscript, LSubscript);
// token_parser!(rsubscript, RSubscript);
token_parser!(comma, Comma);

// Reserved
// token_parser!(continue_token, Continue);
// token_parser!(break_token, Break);

// Compound
token_parser!(if_token, If);
token_parser!(else_token, Else);
token_parser!(while_token, While);
// token_parser!(for_token, For);
// token_parser!(repeat, Repeat);
// token_parser!(in_token, In);
token_parser!(function, Function);
// token_parser!(lambda, Lambda);

// Binary operators
// token_parser!(lassign, LAssign);
// token_parser!(rassign, RAssign);
// token_parser!(old_assign, OldAssign);
// token_parser!(equal, Equal);
// token_parser!(not_equal, NotEqual);
// token_parser!(lower_than, LowerThan);
// token_parser!(greater_than, GreaterThan);
// token_parser!(lower_equal, LowerEqual);
// token_parser!(greater_equal, GreaterEqual);
// token_parser!(power, Power);
// token_parser!(divide, Divide);
// token_parser!(multiply, Multiply);
// token_parser!(minus, Minus);
// token_parser!(plus, Plus);
// token_parser!(help, Help);
// token_parser!(and, And);
// token_parser!(vectorized_and, VectorizedAnd);
// token_parser!(or, Or);
// token_parser!(vectorized_or, VectorizedOr);
// token_parser!(dollar, Dollar);
// token_parser!(pipe, Pipe);
// token_parser!(modulo, Modulo);
// token_parser!(ns_get, NsGet);
// token_parser!(ns_get_int, NsGetInt);
// token_parser!(colon, Colon);

// Unary operators
// token_parser!(unary_not, UnaryNot);

// Comments
// token_parser!(inline_comment, InlineComment(_));
// token_parser!(comment, Comment(_));

// EOF
token_parser!(eof, EOF);

#[cfg(test)]
mod tests {
    use tokenizer::tokens::commented_tokens;

    use super::*;

    #[test]
    fn symbols() {
        let examples = [commented_tokens!(Symbol("a"))];

        for tokens in &examples {
            let tokens: Vec<_> = tokens.iter().collect();
            let res = symbol(&tokens).unwrap().1;
            assert_eq!(res, tokens[0]);
        }
    }

    #[test]
    fn literals() {
        let examples = [commented_tokens!(Literal("a"))];

        for tokens in &examples {
            let tokens: Vec<_> = tokens.iter().collect();
            let res = literal(&tokens).unwrap().1;
            assert_eq!(res, tokens[0]);
        }
    }

    #[test]
    fn test_eof() {
        let examples_ = commented_tokens!(EOF);
        let examples: Vec<_> = examples_.iter().collect();
        let res = eof(&examples).unwrap().1;
        assert_eq!(res, examples[0]);
    }
}
