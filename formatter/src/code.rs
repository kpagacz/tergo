use crate::{config::FormattingConfig, format::DocAlgebra};

use parser::ast::{Arg, Args, Delimiter, Expression, IfConditional, TermExpr};
use tokenizer::tokens::CommentedToken;

use crate::format::{CommonProperties, Doc, InlineCommentPosition, ShouldBreak};
use std::rc::Rc;
use tokenizer::Token;

pub(crate) trait Code {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc>;
}

impl<T> Code for Option<T>
where
    T: Code,
{
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        match self {
            Some(inner) => inner.to_docs(config),
            None => text!(""),
        }
    }
}

// Macro that creates a Doc::Group
macro_rules! group {
    ($doc:expr) => {{
        let doc: Rc<Doc> = $doc;
        doc.to_group(ShouldBreak::No)
    }};
    ($doc:expr, $should_break:expr) => {
        let doc: Rc<Doc> = $doc;
        let should_break: ShouldBreak = $should_break;
        doc.to_group($should_break)
    };
}
pub(crate) use group;

// Macro that creates a Doc::Break
macro_rules! nl {
    ($txt:expr) => {
        Rc::new(Doc::Break($txt))
    };
}
pub(crate) use nl;

// Macro that creates a Doc::Text
macro_rules! text {
    ($txt:expr) => {{
        let txt: &str = $txt;
        Rc::new(Doc::Text(
            Rc::from(txt),
            txt.len(),
            CommonProperties(InlineCommentPosition::No),
        ))
    }};
    ($txt:expr, $size:expr) => {{
        let txt: &str = $txt;
        let size: usize = $size;
        Rc::new(Doc::Text(
            Rc::from(txt),
            size,
            CommonProperties(InlineCommentPosition::No),
        ))
    }};
    ($txt:expr, $size:expr, $comment_position:expr) => {{
        let txt: &str = $txt;
        let size: usize = $size;
        let position: InlineCommentPosition = $comment_position;
        Rc::new(Doc::Text(Rc::from(txt), size, CommonProperties(position)))
    }};
}
pub(crate) use text;

impl<'a> Code for Token<'a> {
    fn to_docs(&self, _: &impl FormattingConfig) -> Rc<Doc> {
        match self {
            Token::Symbol(s) | Token::Literal(s) => text!(*s),
            Token::Semicolon => text!(";"),
            Token::Newline => text!("\n"),
            Token::LParen => text!("("),
            Token::RParen => text!(")"),
            Token::LBrace => text!("{"),
            Token::RBrace => text!("}"),
            Token::LBracket => text!("["),
            Token::RBracket => text!("]"),
            Token::Comma => text!(","),
            Token::Continue => text!("continue"),
            Token::Break => text!("break"),
            Token::If => text!("if"),
            Token::Else => text!("else"),
            Token::While => text!("while"),
            Token::For => text!("for"),
            Token::Repeat => text!("repeat"),
            Token::In => text!("in"),
            Token::Function => text!("function"),
            Token::Lambda => text!("\\"),
            Token::LAssign => text!("<-"),
            Token::RAssign => text!("->"),
            Token::OldAssign => text!("="),
            Token::Equal => text!("=="),
            Token::NotEqual => text!("!="),
            Token::LowerThan => text!("<"),
            Token::GreaterThan => text!(">"),
            Token::LowerEqual => text!("<="),
            Token::GreaterEqual => text!(">="),
            Token::Power => text!("^"),
            Token::Divide => text!("/"),
            Token::Multiply => text!("*"),
            Token::Minus => text!("-"),
            Token::Plus => text!("+"),
            Token::Help => text!("?"),
            Token::And => text!("&&"),
            Token::VectorizedAnd => text!("&"),
            Token::Or => text!("||"),
            Token::VectorizedOr => text!("|"),
            Token::Dollar => text!("$"),
            Token::Pipe => text!("|>"),
            Token::Modulo => text!("%"),
            Token::NsGet => text!("::"),
            Token::NsGetInt => text!(":::"),
            Token::Tilde => text!("~"),
            Token::Colon => text!(":"),
            Token::Slot => text!("@"),
            Token::Special(s) => text!(*s),
            Token::UnaryNot => text!("!"),
            Token::InlineComment(s) => text!(*s, 0),
            Token::Comment(s) => text!(*s),
            Token::EOF => text!(""),
        }
    }
}

impl Code for CommentedToken<'_> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        match (&self.leading_comments, self.inline_comment) {
            (None, None) => self.token.to_docs(config),
            (None, Some(inline_comment)) => self
                .token
                .to_docs(config)
                .cons(text!(" "))
                .cons(text!(inline_comment, 0, InlineCommentPosition::End)),
            (Some(leading_comments), None) => {
                let leading_comments = leading_comments
                    .iter()
                    .fold(Rc::new(Doc::Nil), |first, second| {
                        first.cons(text!(second, 0)).cons(text!("\n"))
                    });

                leading_comments.cons(self.token.to_docs(config))
            }
            (Some(leading_comments), Some(inline_comment)) => {
                let leading_comments = leading_comments
                    .iter()
                    .fold(Rc::new(Doc::Nil), |first, second| {
                        first.cons(text!(second, 0)).cons(text!("\n"))
                    });
                leading_comments
                    .cons(self.token.to_docs(config))
                    .cons(text!(" "))
                    .cons(text!(inline_comment, 0, InlineCommentPosition::End))
            }
        }
    }
}

impl Code for Delimiter<'_> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        match self {
            Delimiter::Paren(single) | Delimiter::SingleBracket(single) => single.to_docs(config),
            Delimiter::DoubleBracket((b1, b2)) => b1.to_docs(config).cons(b2.to_docs(config)),
        }
    }
}

/// Returns a Doc::Group
fn join_docs<I, F>(docs: I, separator: Rc<Doc>, should_break: ShouldBreak, _config: &F) -> Rc<Doc>
where
    I: IntoIterator<Item = Rc<Doc>>,
    F: FormattingConfig,
{
    let mut docs = docs.into_iter();
    let mut res = Rc::new(Doc::Nil);

    if let Some(first_doc) = docs.next() {
        if !matches!(*first_doc, Doc::Nil) {
            res = res.cons(first_doc);
        }
    }

    for next_doc in docs {
        if !matches!(*next_doc, Doc::Nil) {
            res = res.cons(separator.clone()).cons(nl!(" ")).cons(next_doc);
        }
    }

    res = res.to_group(should_break);
    res
}

impl<'a> Code for Expression<'a> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        match self {
            Expression::Symbol(token)
            | Expression::Literal(token)
            | Expression::Comment(token)
            | Expression::Continue(token)
            | Expression::Break(token) => token.to_docs(config),
            Expression::Term(term_expr) => match &**term_expr {
                TermExpr {
                    pre_delimiters: Some(pre_delim),
                    term,
                    post_delimiters: Some(post_delim),
                } if matches!(pre_delim.token, Token::LBrace) => {
                    if term.is_empty() {
                        pre_delim.to_docs(config).cons(post_delim.to_docs(config))
                    } else {
                        let docs = term.iter().map(|t| t.to_docs(config));
                        let inner = join_docs(docs, Rc::new(Doc::Nil), ShouldBreak::Yes, config);
                        pre_delim
                            .to_docs(config)
                            .cons(nl!(" "))
                            .cons(inner)
                            .nest(config.indent())
                            .cons(nl!(" "))
                            .cons(post_delim.to_docs(config))
                            .to_group(ShouldBreak::Yes)
                    }
                }
                TermExpr {
                    pre_delimiters: None,
                    term,
                    post_delimiters: None,
                } => {
                    let docs = term.iter().map(|t| t.to_docs(config));
                    join_docs(docs, Rc::new(Doc::Nil), ShouldBreak::Yes, config)
                }
                TermExpr {
                    pre_delimiters: Some(pre_delim),
                    term,
                    post_delimiters: Some(post_delim),
                } => {
                    if term.is_empty() {
                        pre_delim.to_docs(config).cons(post_delim.to_docs(config))
                    } else {
                        let docs = term.iter().map(|t| t.to_docs(config));
                        let inner = join_docs(docs, Rc::new(Doc::Nil), ShouldBreak::No, config);
                        pre_delim
                            .to_docs(config)
                            .cons(nl!(""))
                            .cons(inner)
                            .nest(config.indent())
                            .cons(nl!(""))
                            .cons(post_delim.to_docs(config))
                            .to_group(ShouldBreak::No)
                    }
                }
                _ => panic!("Term with not matching delimiters found"),
            },
            Expression::Unary(op, expr) => op.to_docs(config).cons(expr.to_docs(config)),
            Expression::Bop(op, lhs, rhs) => match op.token {
                Token::LAssign
                | Token::RAssign
                | Token::OldAssign
                | Token::Equal
                | Token::NotEqual
                | Token::LowerThan
                | Token::GreaterThan
                | Token::LowerEqual
                | Token::GreaterEqual
                | Token::Power
                | Token::Divide
                | Token::Multiply
                | Token::Minus
                | Token::Plus
                | Token::Help
                | Token::And
                | Token::VectorizedAnd
                | Token::Or
                | Token::VectorizedOr
                | Token::Pipe
                | Token::Modulo
                | Token::Tilde
                | Token::Special(_) => lhs
                    .to_docs(config)
                    .cons(text!(" "))
                    .cons(op.to_docs(config))
                    .cons(nl!(" ").cons(rhs.to_docs(config)).nest(config.indent()))
                    .to_group(ShouldBreak::No),
                Token::Dollar | Token::NsGet | Token::NsGetInt | Token::Colon | Token::Slot => lhs
                    .to_docs(config)
                    .cons(op.to_docs(config))
                    .cons(rhs.to_docs(config))
                    .to_group(ShouldBreak::No),
                _ => panic!(
                    "Got a not a binary operator token inside a binary expression when \
                     formatting. Token: {:?}",
                    &op.token
                ),
            },
            Expression::Newline(_) => Rc::new(Doc::Break("\n")),
            Expression::EOF(eof) => eof.to_docs(config),
            Expression::Whitespace(_) => text!(""),
            Expression::FunctionDef(function_def) => {
                let (keyword, args, body) = (
                    function_def.keyword,
                    &function_def.arguments,
                    &function_def.body,
                );
                let args_doc = join_docs(
                    args.args.iter().map(|arg| {
                        arg.0.to_docs(config).cons(
                            arg.1
                                .as_ref()
                                .map(|sep| sep.to_docs(config))
                                .unwrap_or(Rc::new(Doc::Nil)),
                        )
                    }),
                    Rc::new(Doc::Nil),
                    ShouldBreak::No,
                    config,
                );
                let args_group = args
                    .left_delimeter
                    .to_docs(config)
                    .cons(nl!(""))
                    .cons(args_doc)
                    .nest(config.indent())
                    .cons(nl!(""))
                    .cons(args.right_delimeter.to_docs(config))
                    .to_group(ShouldBreak::No);
                keyword
                    .to_docs(config)
                    .cons(args_group)
                    .cons(text!(" "))
                    .cons(body.to_docs(config))
                    .to_group(ShouldBreak::No)
            }
            Expression::IfExpression(if_expression) => {
                let (if_conditional, else_ifs, trailing_else) = (
                    &if_expression.if_conditional,
                    &if_expression.else_ifs,
                    &if_expression.trailing_else,
                );

                let if_conditional_to_docs = |if_conditional: &IfConditional<'_>| {
                    let (keyword, left_delim, condition, right_delim, body) = (
                        if_conditional.keyword,
                        if_conditional.left_delimiter,
                        &if_conditional.condition,
                        if_conditional.right_delimiter,
                        &if_conditional.body,
                    );
                    let condition_docs = left_delim
                        .to_docs(config)
                        .cons(nl!(""))
                        .cons(condition.to_docs(config))
                        .nest(config.indent())
                        .cons(nl!(""))
                        .cons(right_delim.to_docs(config))
                        .to_group(ShouldBreak::No);
                    keyword
                        .to_docs(config)
                        .cons(text!(" "))
                        .cons(condition_docs)
                        .cons(text!(" "))
                        .cons(body.to_docs(config))
                };
                let mut docs = if_conditional_to_docs(if_conditional);
                for else_if in else_ifs {
                    let (else_keyword, conditional) =
                        (else_if.else_keyword, &else_if.if_conditional);
                    docs = docs
                        .cons(text!(" "))
                        .cons(else_keyword.to_docs(config))
                        .cons(text!(" "))
                        .cons(if_conditional_to_docs(conditional));
                }
                if let Some(trailing_else) = trailing_else {
                    let (else_keyword, body) = (&trailing_else.else_keyword, &trailing_else.body);
                    docs = docs
                        .cons(text!(" "))
                        .cons(else_keyword.to_docs(config))
                        .cons(text!(" "))
                        .cons(body.to_docs(config));
                }
                docs
            }
            Expression::WhileExpression(while_expression) => {
                let (keyword, condition, body) = (
                    &while_expression.while_keyword,
                    &while_expression.condition,
                    &while_expression.body,
                );
                keyword
                    .to_docs(config)
                    .cons(text!(" "))
                    .cons(condition.to_docs(config))
                    .cons(text!(" "))
                    .cons(body.to_docs(config))
                    .to_group(ShouldBreak::No)
            }
            Expression::RepeatExpression(repeat_expression) => {
                let (keyword, body) = (&repeat_expression.repeat_keyword, &repeat_expression.body);
                group!(keyword.to_docs(config).cons(body.to_docs(config)))
            }
            Expression::FunctionCall(function_call) => {
                let (function_ref, args) = (&function_call.function_ref, &function_call.args);
                function_ref
                    .to_docs(config)
                    .cons(args.to_docs(config))
                    .to_group(ShouldBreak::No)
            }
            Expression::SubsetExpression(subset_expression) => {
                let (object_ref, args) = (&subset_expression.object_ref, &subset_expression.args);
                group!(object_ref.to_docs(config).cons(args.to_docs(config)))
            }
            Expression::ForLoopExpression(for_loop) => {
                let (keyword, left_delim, identifier, in_keyword, collection, right_delim, body) = (
                    &for_loop.keyword,
                    &for_loop.left_delim,
                    &for_loop.identifier,
                    &for_loop.in_keyword,
                    &for_loop.collection,
                    &for_loop.right_delim,
                    &for_loop.body,
                );
                keyword
                    .to_docs(config)
                    .cons(text!(" "))
                    .cons(left_delim.to_docs(config))
                    .cons(nl!(""))
                    .cons(identifier.to_docs(config))
                    .cons(text!(" "))
                    .cons(in_keyword.to_docs(config))
                    .cons(nl!(" "))
                    .cons(collection.to_docs(config))
                    .nest(config.indent())
                    .cons(nl!(""))
                    .cons(right_delim.to_docs(config))
                    .to_group(ShouldBreak::No)
                    .cons(text!(" "))
                    .cons(body.to_docs(config))
                    .to_group(ShouldBreak::No)
            }
            Expression::LambdaFunction(lambda) => {
                let (keyword, args, body) = (&lambda.keyword, &lambda.args, &lambda.body);
                keyword
                    .to_docs(config)
                    .cons(args.to_docs(config))
                    .cons(text!(" "))
                    .cons(body.to_docs(config))
                    .to_group(ShouldBreak::No)
            }
        }
    }
}

impl Code for Args<'_> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        let mut docs = self.left_delimeter.to_docs(config).cons(nl!(""));
        let mut it = self.args.iter();
        if let Some(arg) = it.next() {
            docs = docs.cons(arg.to_docs(config));
        }
        for arg in it {
            docs = docs.cons(nl!(" "));
            docs = docs.cons(arg.to_docs(config));
        }
        docs.nest(config.indent())
            .cons(nl!(""))
            .cons(self.right_delimeter.to_docs(config))
            .to_group(ShouldBreak::No)
    }
}
impl Code for Arg<'_> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        if let Some(comma) = &self.1 {
            self.0.to_docs(config).cons(comma.to_docs(config))
        } else {
            self.0.to_docs(config)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::format::{format_to_sdoc, simple_doc_to_string, Mode};

    use super::*;

    struct MockConfig;

    impl FormattingConfig for MockConfig {
        fn line_length(&self) -> i32 {
            120
        }
        fn indent(&self) -> i32 {
            0
        }
    }
    impl std::fmt::Display for MockConfig {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
    use std::collections::VecDeque;

    #[test]
    fn joining_docs_with_newlines_produces_newlines() {
        let docs = [text!("test"), text!("test2")];
        let mock_config = MockConfig {};
        let mut doc = VecDeque::from([(
            0,
            Mode::Flat,
            join_docs(docs, Rc::new(Doc::Nil), ShouldBreak::Yes, &mock_config),
        )]);

        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config));

        assert_eq!(simple_doc_to_string(sdoc), "test\ntest2")
    }

    #[test]
    fn joinin_docs_with_newlines_does_nothing_for_just_one_doc() {
        let docs = [text!("test")];
        let mock_config = MockConfig {};
        let mut doc = VecDeque::from([(
            0,
            Mode::Flat,
            join_docs(docs, Rc::new(Doc::Nil), ShouldBreak::No, &mock_config),
        )]);

        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config));

        assert_eq!(simple_doc_to_string(sdoc), "test")
    }
}
