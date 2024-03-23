use crate::format::Triple;

pub(crate) trait Code {
    fn to_docs(&self) -> Triple;
}

use parser::ast::CommentedToken;
use parser::ast::Expression;
use tokenizer::LocatedToken;

use crate::format::Doc;
use crate::format::Mode;
use std::rc::Rc;
use tokenizer::Token;
const INDENT: i32 = 2;

impl<'a> Code for Token<'a> {
    fn to_docs(&self) -> Triple {
        match self {
            Token::Symbol(s) | Token::Literal(s) => {
                (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(*s))))
            }
            Token::Semicolon => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(";")))),
            Token::Newline => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("\n")))),
            Token::LParen => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("(")))),
            Token::RParen => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(")")))),
            Token::LBrace => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("{")))),
            Token::RBrace => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("}")))),
            Token::LSubscript => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("[")))),
            Token::RSubscript => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("]")))),
            Token::Comma => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(",")))),
            Token::Continue => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("continue")))),
            Token::Break => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("break")))),
            Token::If => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("if")))),
            Token::Else => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("else")))),
            Token::While => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("while")))),
            Token::For => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("for")))),
            Token::Repeat => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("repeat")))),
            Token::In => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("in")))),
            Token::Function => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("function")))),
            Token::Lambda => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("\\")))),
            Token::LAssign => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("<-")))),
            Token::RAssign => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("->")))),
            Token::OldAssign => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("=")))),
            Token::Equal => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("==")))),
            Token::NotEqual => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("!=")))),
            Token::LowerThan => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("<")))),
            Token::GreaterThan => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(">")))),
            Token::LowerEqual => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("<=")))),
            Token::GreaterEqual => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(">=")))),
            Token::Power => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("^")))),
            Token::Divide => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("/")))),
            Token::Multiply => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("*")))),
            Token::Minus => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("-")))),
            Token::Plus => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("+")))),
            Token::Help => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("?")))),
            Token::And => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("&&")))),
            Token::VectorizedAnd => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("&")))),
            Token::Or => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("||")))),
            Token::VectorizedOr => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("|")))),
            Token::Dollar => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("$")))),
            Token::Pipe => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("|>")))),
            Token::Modulo => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("%")))),
            Token::NsGet => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("::")))),
            Token::NsGetInt => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(":::")))),
            Token::Tilde => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("~")))),
            Token::Colon => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(":")))),
            Token::Slot => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("@")))),
            Token::Special(s) => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(*s)))),
            Token::UnaryNot => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("!")))),
            Token::InlineComment(s) => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(*s)))),
            Token::Comment(s) => (INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from(*s)))),
            Token::EOF => (0, Mode::Break, Rc::new(Doc::Break(""))),
        }
    }
}

impl Code for LocatedToken<'_> {
    fn to_docs(&self) -> Triple {
        self.token.to_docs()
    }
}

impl Code for CommentedToken<'_> {
    fn to_docs(&self) -> Triple {
        let mut docs = vec![];
        for comment in self.leading_comments {
            docs.push(comment.to_docs());
            // TODO: check if this works
            // Force a new line (I am not sure if the code already does it somewhere else)
            docs.push((INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("\n")))));
        }
        docs.push(self.token.to_docs());
        if let Some(inline) = &self.inline_comment {
            docs.push(inline.to_docs());
        }

        (INDENT, Mode::Flat, Rc::new(Doc::Group(docs)))
    }
}

impl<'a> Code for Expression<'a> {
    fn to_docs(&self) -> Triple {
        match self {
            Expression::Symbol(token) | Expression::Literal(token) | Expression::Comment(token) => {
                token.to_docs()
            }
            Expression::Term(term_expr) => {
                let (pre, term, post) = (
                    &term_expr.pre_delimiters,
                    &term_expr.term,
                    &term_expr.post_delimiters,
                );
                let mut docs = vec![];
                if let Some(pre) = pre {
                    docs.push(pre.to_docs());
                    docs.push((INDENT, Mode::Flat, Rc::new(Doc::Break(""))));
                }
                docs.push(term.to_docs());
                if let Some(post) = post {
                    docs.push((INDENT, Mode::Flat, Rc::new(Doc::Break(""))));
                    docs.push(post.to_docs());
                }
                (INDENT, Mode::Flat, Rc::new(Doc::Group(docs)))
            }
            Expression::Bop(op, lhs, rhs) => (
                INDENT,
                Mode::Flat,
                Rc::new(Doc::Group(vec![
                    lhs.to_docs(),
                    (INDENT, Mode::Flat, Rc::new(Doc::Break(" "))),
                    op.to_docs(),
                    (INDENT, Mode::Flat, Rc::new(Doc::Break(" "))),
                    rhs.to_docs(),
                ])),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use parser::ast::Expression;
    use parser::helpers::commented_tokens;
    use parser::located_tokens;
    use tokenizer::LocatedToken;
    use tokenizer::Token;

    #[test]
    fn test_symbol() {
        let located_tokens = located_tokens!(Token::Symbol("test"));
        let commented_tokens = commented_tokens(&located_tokens);
        let expression = Expression::Symbol(&commented_tokens[0]);
        let docs = expression.to_docs();

        assert_eq!(docs.0, INDENT);
        assert_eq!(docs.1, Mode::Flat);
        match &*docs.2 {
            Doc::Group(docs) => {
                assert_eq!(docs.len(), 1);
                match &*docs[0].2 {
                    Doc::Text(s) => assert_eq!(&s[..], "test"),
                    _ => panic!("Expected Doc::Text"),
                }
            }
            _ => panic!("Expected Doc::Text"),
        }
    }

    #[test]
    fn test_literal() {
        let located_tokens = located_tokens!(Token::Literal("123592035"));
        let commented_tokens = commented_tokens(&located_tokens);
        let expression = Expression::Literal(&commented_tokens[0]);
        let docs = expression.to_docs();
        assert_eq!(docs.0, INDENT);
        assert_eq!(docs.1, Mode::Flat);
        match &*docs.2 {
            Doc::Group(docs) => {
                assert_eq!(docs.len(), 1);
                match &*docs[0].2 {
                    Doc::Text(s) => assert_eq!(&s[..], "123592035"),
                    _ => panic!("Expected Doc::Text"),
                }
            }
            _ => panic!("Expected Doc::Text"),
        }
    }
}
