use crate::format::CommonProperties;
use crate::{
    config::{FormattingConfig, FunctionLineBreaks},
    format::DocAlgebra,
};

use parser::ast::{Arg, Args, Delimiter, Expression, IfConditional, TermExpr};
use tokenizer::tokens::CommentedToken;

use crate::format::{Doc, InlineCommentPosition, ShouldBreak};
use std::{ops::Deref, rc::Rc};
use tokenizer::Token;

pub(crate) trait Code {
    fn to_docs(&self, config: &impl FormattingConfig, doc_ref: &mut usize) -> Rc<Doc>;
}

impl<T> Code for Option<T>
where
    T: Code,
{
    fn to_docs(&self, config: &impl FormattingConfig, doc_ref: &mut usize) -> Rc<Doc> {
        match self {
            Some(inner) => inner.to_docs(config, doc_ref),
            None => text!(""),
        }
    }
}

pub(crate) trait CodeWithoutLeadingComments {
    fn to_docs_without_leading_comments(
        &self,
        config: &impl FormattingConfig,
        doc_ref: &mut usize,
    ) -> Rc<Doc>;
}

impl<T> CodeWithoutLeadingComments for Option<T>
where
    T: CodeWithoutLeadingComments,
{
    fn to_docs_without_leading_comments(
        &self,
        config: &impl FormattingConfig,
        doc_ref: &mut usize,
    ) -> Rc<Doc> {
        match self {
            Some(code) => code.to_docs_without_leading_comments(config, doc_ref),
            None => Rc::new(Doc::Nil),
        }
    }
}

/// Returns the inline comments separately from the rest
/// of the commented token.
/// There is no whitespace between the token and the inline comment.
/// If the inline comment is None, the second element is None.
pub(crate) trait DocAlgebraWithSeparateComments {
    fn to_docs_with_separate_comments(
        &self,
        config: &impl FormattingConfig,
        doc_ref: &mut usize,
    ) -> (Rc<Doc>, Option<Rc<Doc>>);
}

impl<T> DocAlgebraWithSeparateComments for Option<T>
where
    T: DocAlgebraWithSeparateComments,
{
    fn to_docs_with_separate_comments(
        &self,
        config: &impl FormattingConfig,
        doc_ref: &mut usize,
    ) -> (Rc<Doc>, Option<Rc<Doc>>) {
        match self {
            Some(code) => code.to_docs_with_separate_comments(config, doc_ref),
            None => (Rc::new(Doc::Nil), None),
        }
    }
}

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
            CommonProperties(InlineCommentPosition::No, 0),
        ))
    }};
    ($txt:expr, $size:expr) => {{
        let txt: &str = $txt;
        let size: usize = $size;
        Rc::new(Doc::Text(
            Rc::from(txt),
            size,
            CommonProperties(InlineCommentPosition::No, 0),
        ))
    }};
    ($txt:expr, $size:expr, $comment_position:expr) => {{
        let txt: &str = $txt;
        let size: usize = $size;
        let position: InlineCommentPosition = $comment_position;
        Rc::new(Doc::Text(
            Rc::from(txt),
            size,
            CommonProperties(position, 0),
        ))
    }};
}
pub(crate) use text;

// Macro that creates a HardBreak
macro_rules! hardbreak {
    () => {{ Rc::new(Doc::HardBreak) }};
}
pub(crate) use hardbreak;

impl Code for Token<'_> {
    fn to_docs(&self, _: &impl FormattingConfig, _: &mut usize) -> Rc<Doc> {
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
            Token::Stop => text!("stop"),
            Token::If => text!("if"),
            Token::Else => text!("else"),
            Token::While => text!("while"),
            Token::For => text!("for"),
            Token::Repeat => text!("repeat"),
            Token::In => text!("in"),
            Token::Function => text!("function"),
            Token::Lambda => text!("\\"),
            Token::LAssign => text!("<-"),
            Token::SuperAssign => text!("<<-"),
            Token::ColonAssign => text!(":="),
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
            Token::Modulo => text!("%%"),
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
    fn to_docs(&self, config: &impl FormattingConfig, doc_ref: &mut usize) -> Rc<Doc> {
        match (&self.leading_comments, self.inline_comment) {
            (None, None) => self.token.to_docs(config, doc_ref),
            (None, Some(inline_comment)) => self
                .token
                .to_docs(config, doc_ref)
                .cons(text!(" "))
                .cons(text!(inline_comment, 0, InlineCommentPosition::End))
                .cons(hardbreak!()),
            (Some(leading_comments), None) => {
                let mut leading_comments_it = leading_comments.iter();
                let mut leading_comments = text!(
                    leading_comments_it.next().unwrap(),
                    0,
                    InlineCommentPosition::End
                );
                for comment in leading_comments_it {
                    leading_comments = leading_comments.cons(nl!("")).cons(text!(
                        comment,
                        0,
                        InlineCommentPosition::End
                    ));
                }
                let leading_comments = leading_comments
                    .nest_hanging()
                    .to_group(ShouldBreak::Yes, doc_ref);
                leading_comments
                    .cons(nl!(""))
                    .cons(
                        self.token
                            .to_docs(config, doc_ref)
                            .to_group(ShouldBreak::No, doc_ref),
                    )
                    .to_group(ShouldBreak::Yes, doc_ref)
            }
            (Some(leading_comments), Some(inline_comment)) => {
                let mut leading_comments_it = leading_comments.iter();
                let mut leading_comments = text!(leading_comments_it.next().unwrap());
                for comment in leading_comments_it {
                    leading_comments = leading_comments.cons(nl!("")).cons(text!(comment, 0));
                }
                let leading_comments = leading_comments
                    .nest_hanging()
                    .to_group(ShouldBreak::Yes, doc_ref);
                leading_comments
                    .cons(nl!(""))
                    .cons(
                        self.token
                            .to_docs(config, doc_ref)
                            .cons(text!(" "))
                            .cons(text!(inline_comment, 0, InlineCommentPosition::End))
                            .cons(hardbreak!()),
                    )
                    .to_group(ShouldBreak::Propagate, doc_ref)
            }
        }
    }
}

impl CodeWithoutLeadingComments for CommentedToken<'_> {
    fn to_docs_without_leading_comments(
        &self,
        config: &impl FormattingConfig,
        doc_ref: &mut usize,
    ) -> Rc<Doc> {
        match self.inline_comment {
            None => self.token.to_docs(config, doc_ref),
            Some(inline_comment) => self
                .token
                .to_docs(config, doc_ref)
                .cons(text!(" "))
                .cons(text!(inline_comment, 0, InlineCommentPosition::End)),
        }
    }
}

impl DocAlgebraWithSeparateComments for CommentedToken<'_> {
    fn to_docs_with_separate_comments(
        &self,
        config: &impl FormattingConfig,
        doc_ref: &mut usize,
    ) -> (Rc<Doc>, Option<Rc<Doc>>) {
        match (&self.leading_comments, self.inline_comment) {
            (None, None) => (self.token.to_docs(config, doc_ref), None),
            (None, Some(inline_comment)) => (
                self.token.to_docs(config, doc_ref),
                Some(text!(inline_comment, 0, InlineCommentPosition::End)),
            ),
            (Some(leading_comments), None) => {
                let mut leading_comments_it = leading_comments.iter();
                let mut leading_comments = text!(
                    leading_comments_it.next().unwrap(),
                    0,
                    InlineCommentPosition::End
                );
                for comment in leading_comments_it {
                    leading_comments = leading_comments.cons(nl!("")).cons(text!(
                        comment,
                        0,
                        InlineCommentPosition::End
                    ));
                }
                let leading_comments = leading_comments
                    .nest_hanging()
                    .to_group(ShouldBreak::Yes, doc_ref);
                (
                    leading_comments
                        .cons(nl!(""))
                        .cons(
                            self.token
                                .to_docs(config, doc_ref)
                                .to_group(ShouldBreak::No, doc_ref),
                        )
                        .to_group(ShouldBreak::Yes, doc_ref),
                    None,
                )
            }
            (Some(leading_comments), Some(inline_comment)) => {
                let mut leading_comments_it = leading_comments.iter();
                let mut leading_comments = text!(leading_comments_it.next().unwrap());
                for comment in leading_comments_it {
                    leading_comments = leading_comments.cons(nl!("")).cons(text!(comment, 0));
                }
                let leading_comments = leading_comments
                    .nest_hanging()
                    .to_group(ShouldBreak::Yes, doc_ref);
                (
                    leading_comments
                        .cons(nl!(""))
                        .cons(self.token.to_docs(config, doc_ref)),
                    Some(text!(inline_comment, 0, InlineCommentPosition::End)),
                )
            }
        }
    }
}

impl Code for Delimiter<'_> {
    fn to_docs(&self, config: &impl FormattingConfig, doc_ref: &mut usize) -> Rc<Doc> {
        match self {
            Delimiter::Paren(single) | Delimiter::SingleBracket(single) => {
                single.to_docs(config, doc_ref)
            }
            Delimiter::DoubleBracket((b1, b2)) => b1
                .to_docs(config, doc_ref)
                .cons(b2.to_docs(config, doc_ref)),
        }
    }
}

impl DocAlgebraWithSeparateComments for Delimiter<'_> {
    fn to_docs_with_separate_comments(
        &self,
        config: &impl FormattingConfig,
        doc_ref: &mut usize,
    ) -> (Rc<Doc>, Option<Rc<Doc>>) {
        match self {
            Delimiter::Paren(token) | Delimiter::SingleBracket(token) => {
                token.to_docs_with_separate_comments(config, doc_ref)
            }
            Delimiter::DoubleBracket((b1, b2)) => {
                let first = b1.to_docs(config, doc_ref);
                let (second, comment) = b2.to_docs_with_separate_comments(config, doc_ref);
                (first.cons(second), comment)
            }
        }
    }
}

/// Returns a Doc::Group
fn join_docs<I, F>(
    docs: I,
    separator: Rc<Doc>,
    should_break: ShouldBreak,
    _config: &F,
    doc_ref: &mut usize,
) -> Rc<Doc>
where
    I: IntoIterator<Item = Rc<Doc>>,
    F: FormattingConfig,
{
    join_docs_ungroupped(docs, separator, _config).to_group(should_break, doc_ref)
}

/// Returns a Doc::Cons
fn join_docs_ungroupped<I, F>(docs: I, separator: Rc<Doc>, _config: &F) -> Rc<Doc>
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

    res
}

impl Code for Expression<'_> {
    fn to_docs(&self, config: &impl FormattingConfig, doc_ref: &mut usize) -> Rc<Doc> {
        match self {
            Expression::Symbol(token)
            | Expression::Literal(token)
            | Expression::Comment(token)
            | Expression::Continue(token)
            | Expression::Break(token) => token.to_docs(config, doc_ref),
            Expression::Term(term_expr) => match &**term_expr {
                // Special case for the embracing operator
                // {{ }} which should not break
                TermExpr {
                    pre_delimiters: Some(pre_delim),
                    term,
                    post_delimiters: Some(post_delim),
                } if config.embracing_op_no_nl()
                    && matches!(pre_delim.token, Token::LBrace)
                    && term.len() == 1
                    && matches!(term[0], Expression::Term { .. }) =>
                {
                    match &term[0] {
                        Expression::Term(inner_term_expr) => {
                            if inner_term_expr
                                .pre_delimiters
                                .is_some_and(|delim| matches!(delim.token, Token::LBrace))
                            {
                                let inner_docs: Vec<_> = inner_term_expr
                                    .term
                                    .iter()
                                    .map(|t| t.to_docs(config, doc_ref))
                                    .collect();
                                let inner_docs = join_docs(
                                    inner_docs,
                                    Rc::new(Doc::Nil),
                                    ShouldBreak::No,
                                    config,
                                    doc_ref,
                                );
                                pre_delim
                                    .to_docs(config, doc_ref)
                                    .cons(
                                        inner_term_expr
                                            .pre_delimiters
                                            .as_ref()
                                            .expect(
                                                "Already checked this pre delimiter to be an l \
                                                 brace",
                                            )
                                            .to_docs(config, doc_ref),
                                    )
                                    .cons(text!(" "))
                                    .cons(inner_docs)
                                    .cons(text!(" "))
                                    .cons(
                                        inner_term_expr
                                            .post_delimiters
                                            .as_ref()
                                            .unwrap()
                                            .to_docs(config, doc_ref),
                                    )
                                    .cons(post_delim.to_docs(config, doc_ref))
                                    .to_group(ShouldBreak::No, doc_ref)
                            } else {
                                let docs: Vec<_> =
                                    term.iter().map(|t| t.to_docs(config, doc_ref)).collect();
                                let inner = join_docs(
                                    docs,
                                    Rc::new(Doc::Nil),
                                    ShouldBreak::No,
                                    config,
                                    doc_ref,
                                );
                                pre_delim
                                    .to_docs(config, doc_ref)
                                    .cons(nl!(" ").cons(inner).nest(config.indent()))
                                    .cons(nl!(" "))
                                    .cons(post_delim.to_docs(config, doc_ref))
                                    .to_group(ShouldBreak::Propagate, doc_ref)
                            }
                        }
                        _ => unreachable!("Already checked that term[0] is a Term"),
                    }
                }
                // Normal { }
                TermExpr {
                    pre_delimiters: Some(pre_delim),
                    term,
                    post_delimiters: Some(post_delim),
                } if matches!(pre_delim.token, Token::LBrace) => {
                    if term.is_empty() {
                        pre_delim
                            .to_docs(config, doc_ref)
                            .cons(nl!(""))
                            .nest(config.indent())
                            .cons(post_delim.to_docs(config, doc_ref))
                            .to_group(ShouldBreak::No, doc_ref)
                    } else {
                        let docs = term
                            .iter()
                            .map(|t| {
                                t.to_docs(config, doc_ref)
                                    .to_group(ShouldBreak::No, doc_ref)
                            })
                            .collect::<Vec<_>>();
                        let inner = join_docs(
                            docs,
                            Rc::new(Doc::Nil),
                            ShouldBreak::Propagate,
                            config,
                            doc_ref,
                        );
                        delimited_content_to_docs(
                            pre_delim,
                            inner,
                            post_delim,
                            config,
                            doc_ref,
                            ShouldBreak::Propagate,
                        )
                    }
                }
                TermExpr {
                    pre_delimiters: None,
                    term,
                    post_delimiters: None,
                } => {
                    let docs = term
                        .iter()
                        .map(|t| {
                            t.to_docs(config, doc_ref)
                                .to_group(ShouldBreak::No, doc_ref)
                        })
                        .collect::<Vec<_>>();
                    join_docs(
                        docs,
                        Rc::new(Doc::Nil),
                        ShouldBreak::Propagate,
                        config,
                        doc_ref,
                    )
                }
                TermExpr {
                    pre_delimiters: Some(pre_delim),
                    term,
                    post_delimiters: Some(post_delim),
                } => {
                    if term.is_empty() {
                        pre_delim
                            .to_docs(config, doc_ref)
                            .cons(post_delim.to_docs(config, doc_ref))
                    } else if term.len() == 1 && matches!(term[0], Expression::Term(..)) {
                        // Special case for these scenarios
                        // ({
                        //   TRUE
                        //   # Comment
                        // })
                        // In these cases we delegate the line breaks to the inner term.
                        let docs = term
                            .iter()
                            .map(|t| t.to_docs(config, doc_ref))
                            .collect::<Vec<_>>();
                        let inner =
                            join_docs(docs, Rc::new(Doc::Nil), ShouldBreak::No, config, doc_ref);
                        pre_delim
                            .to_docs(config, doc_ref)
                            .cons(inner)
                            .cons(post_delim.to_docs(config, doc_ref))
                            .to_group(ShouldBreak::No, doc_ref)
                    } else {
                        let docs = term
                            .iter()
                            .map(|t| t.to_docs(config, doc_ref))
                            .collect::<Vec<_>>();
                        let inner =
                            join_docs(docs, Rc::new(Doc::Nil), ShouldBreak::No, config, doc_ref);
                        delimited_content_to_docs(
                            pre_delim,
                            inner,
                            post_delim,
                            config,
                            doc_ref,
                            ShouldBreak::No,
                        )
                    }
                }
                _ => panic!("Term with not matching delimiters found"),
            },
            Expression::Unary(op, expr) => op
                .to_docs(config, doc_ref)
                .cons(expr.to_docs(config, doc_ref)),
            Expression::Bop(op, lhs, rhs) => match op.token {
                Token::OldAssign | Token::LAssign | Token::ColonAssign | Token::SuperAssign
                    if !config.allow_nl_after_assignment() =>
                {
                    lhs.to_docs(config, doc_ref)
                        .cons(text!(" "))
                        .cons(op.to_docs(config, doc_ref))
                        .cons(text!(" "))
                        .cons(rhs.to_docs(config, doc_ref).nest(config.indent()))
                }
                Token::RAssign
                | Token::Equal
                | Token::NotEqual
                | Token::LowerThan
                | Token::GreaterThan
                | Token::LowerEqual
                | Token::GreaterEqual
                | Token::Divide
                | Token::Multiply
                | Token::Minus
                | Token::Plus
                | Token::And
                | Token::VectorizedAnd
                | Token::Or
                | Token::VectorizedOr
                | Token::Pipe
                | Token::Modulo
                | Token::Tilde
                | Token::Special(_) => lhs
                    .to_docs(config, doc_ref)
                    .cons(text!(" "))
                    .cons(op.to_docs(config, doc_ref))
                    .to_group(ShouldBreak::No, doc_ref)
                    .cons(
                        nl!(" ")
                            .cons(rhs.to_docs(config, doc_ref))
                            .nest(config.indent()),
                    ),
                Token::Dollar
                | Token::NsGet
                | Token::NsGetInt
                | Token::Colon
                | Token::Slot
                | Token::Power
                | Token::Help => lhs
                    .to_docs(config, doc_ref)
                    .cons(op.to_docs(config, doc_ref))
                    .cons(rhs.to_docs(config, doc_ref).nest(config.indent())),
                _ => panic!(
                    "Got a not a binary operator token inside a binary expression when \
                     formatting. Token: {:?}",
                    &op
                ),
            },
            Expression::Formula(tilde, term) => tilde
                .to_docs(config, doc_ref)
                .cons(if matches!(**term, Expression::Symbol(_)) {
                    text!("")
                } else {
                    text!(" ")
                })
                .cons(term.to_docs(config, doc_ref)),
            Expression::Newline(_) => Rc::new(Doc::Break("\n")),
            Expression::EOF(eof) => eof.to_docs(config, doc_ref),
            Expression::Whitespace(_) => text!(""),
            Expression::FunctionDef(function_def) => {
                let (keyword, args, body) = (
                    function_def.keyword,
                    &function_def.arguments,
                    &function_def.body,
                );

                let after_right_delim_doc = if args.right_delimeter.is_inline_commented() {
                    nl!(" ")
                } else {
                    text!(" ")
                };

                match config.function_line_breaks() {
                    FunctionLineBreaks::Hanging => {
                        let args_doc = join_docs_ungroupped(
                            args.args.iter().map(|arg| {
                                arg.to_docs(config, doc_ref)
                                    .to_group(ShouldBreak::No, doc_ref)
                            }),
                            Rc::new(Doc::Nil),
                            config,
                        );
                        let args_group = args
                            .left_delimeter
                            .to_docs(config, doc_ref)
                            .cons(args_doc.nest_hanging())
                            .cons(
                                args.right_delimeter
                                    .to_docs(config, doc_ref)
                                    .cons(after_right_delim_doc)
                                    .to_group(ShouldBreak::No, doc_ref),
                            );
                        keyword
                            .to_docs(config, doc_ref)
                            .cons(args_group.to_group(ShouldBreak::No, doc_ref))
                            .cons(
                                body.to_docs(config, doc_ref)
                                    .to_group(ShouldBreak::No, doc_ref),
                            )
                            .to_group(ShouldBreak::No, doc_ref)
                    }
                    FunctionLineBreaks::Double => {
                        let args_doc = join_docs_ungroupped(
                            args.args.iter().map(|arg| {
                                arg.to_docs(config, doc_ref)
                                    .to_group(ShouldBreak::No, doc_ref)
                            }),
                            Rc::new(Doc::Nil),
                            config,
                        );
                        let args_group = args
                            .left_delimeter
                            .to_docs(config, doc_ref)
                            .cons(nl!(""))
                            .cons(args_doc)
                            .nest(2 * config.indent())
                            .cons(nl!(""))
                            .cons(
                                args.right_delimeter
                                    .to_docs(config, doc_ref)
                                    .cons(after_right_delim_doc)
                                    .to_group(ShouldBreak::No, doc_ref),
                            )
                            .to_group(ShouldBreak::No, doc_ref);
                        keyword
                            .to_docs(config, doc_ref)
                            .cons(args_group)
                            .cons(body.to_docs(config, doc_ref))
                            .to_group(ShouldBreak::No, doc_ref)
                    }
                    FunctionLineBreaks::Single => {
                        let args_doc = join_docs_ungroupped(
                            args.args.iter().map(|arg| {
                                arg.to_docs(config, doc_ref)
                                    .to_group(ShouldBreak::No, doc_ref)
                            }),
                            Rc::new(Doc::Nil),
                            config,
                        );
                        let args_group = args
                            .left_delimeter
                            .to_docs(config, doc_ref)
                            .cons(nl!(""))
                            .cons(args_doc)
                            .nest(config.indent())
                            .cons(nl!(""))
                            .cons(
                                args.right_delimeter
                                    .to_docs(config, doc_ref)
                                    .cons(after_right_delim_doc)
                                    .to_group(ShouldBreak::No, doc_ref),
                            )
                            .to_group(ShouldBreak::No, doc_ref);
                        keyword
                            .to_docs(config, doc_ref)
                            .cons(args_group)
                            .cons(body.to_docs(config, doc_ref))
                            .to_group(ShouldBreak::No, doc_ref)
                    }
                }
            }
            Expression::IfExpression(if_expression) => {
                let (if_conditional, else_ifs, trailing_else) = (
                    &if_expression.if_conditional,
                    &if_expression.else_ifs,
                    &if_expression.trailing_else,
                );

                let if_conditional_to_docs =
                    |if_conditional: &IfConditional<'_>, doc_ref: &mut usize| {
                        let (keyword, left_delim, condition, right_delim, body) = (
                            if_conditional.keyword,
                            if_conditional.left_delimiter,
                            &if_conditional.condition,
                            if_conditional.right_delimiter,
                            &if_conditional.body,
                        );
                        let condition_docs = left_delim
                            .to_docs(config, doc_ref)
                            .cons(nl!(""))
                            .cons(condition.to_docs(config, doc_ref))
                            .nest(config.indent())
                            .cons(nl!(""))
                            .to_group(ShouldBreak::No, doc_ref);
                        keyword
                            .to_docs(config, doc_ref)
                            .cons(text!(" "))
                            .cons(condition_docs)
                            .cons(
                                right_delim
                                    .to_docs(config, doc_ref)
                                    .cons(nl!(" "))
                                    .to_group(ShouldBreak::No, doc_ref),
                            )
                            .cons(
                                body.to_docs(config, doc_ref)
                                    .to_group(ShouldBreak::No, doc_ref),
                            )
                    };
                let mut docs = if_conditional_to_docs(if_conditional, doc_ref);
                for else_if in else_ifs {
                    let (else_keyword, conditional) =
                        (else_if.else_keyword, &else_if.if_conditional);
                    docs = docs
                        .cons(text!(" "))
                        .cons(
                            else_keyword
                                .to_docs(config, doc_ref)
                                .cons(if else_keyword.inline_comment.is_some() {
                                    nl!(" ")
                                } else {
                                    text!(" ")
                                })
                                .to_group(ShouldBreak::No, doc_ref),
                        )
                        .cons(
                            if_conditional_to_docs(conditional, doc_ref)
                                .to_group(ShouldBreak::No, doc_ref),
                        );
                }
                if let Some(trailing_else) = trailing_else {
                    let (else_keyword, body) = (&trailing_else.else_keyword, &trailing_else.body);
                    docs = docs
                        .cons(text!(" "))
                        .cons(
                            else_keyword
                                .to_docs(config, doc_ref)
                                .cons(if else_keyword.inline_comment.is_some() {
                                    nl!(" ")
                                } else {
                                    text!(" ")
                                })
                                .to_group(ShouldBreak::No, doc_ref),
                        )
                        .cons(
                            body.to_docs(config, doc_ref)
                                .to_group(ShouldBreak::No, doc_ref),
                        );
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
                    .to_docs(config, doc_ref)
                    .cons(text!(" "))
                    .cons(condition.to_docs(config, doc_ref))
                    .cons(text!(" "))
                    .cons(body.to_docs(config, doc_ref))
                    .to_group(ShouldBreak::No, doc_ref)
            }
            Expression::RepeatExpression(repeat_expression) => {
                let (keyword, body) = (&repeat_expression.repeat_keyword, &repeat_expression.body);
                let is_body_lbraced = if let Expression::Term(term_expr) = &**body {
                    let pre_delimiters = &term_expr.pre_delimiters;
                    pre_delimiters.is_some_and(|delimiter| matches!(delimiter.token, Token::LBrace))
                } else {
                    false
                };
                if is_body_lbraced {
                    keyword
                        .to_docs(config, doc_ref)
                        .cons(text!(" "))
                        .cons(body.to_docs(config, doc_ref))
                        .to_group(ShouldBreak::No, doc_ref)
                } else {
                    keyword
                        .to_docs(config, doc_ref)
                        .cons(body.to_docs(config, doc_ref))
                        .to_group(ShouldBreak::No, doc_ref)
                }
            }
            Expression::FunctionCall(function_call) => {
                let (function_ref, args) = (&function_call.function_ref, &function_call.args);
                let is_function_ref_quote = {
                    if let Expression::Symbol(token) = function_ref.as_ref() {
                        if let Token::Symbol(text) = &token.token {
                            *text == "quote"
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                let inner_docs = args.to_docs(config, doc_ref);
                if is_function_ref_quote && args.args.len() == 1 {
                    if let Arg::Proper(arg, _) = args.args.first().unwrap() {
                        if arg
                            .as_ref()
                            .is_some_and(|arg| !is_closure_with_brackets(arg))
                            && has_forced_line_breaks(&inner_docs, false)
                        {
                            // Special case for the quote function call
                            // in such cases:
                            // quote(a <- function() {
                            //   TRUE
                            //   TRUE
                            // })
                            // It should be
                            // quote(
                            //   a <- function() {
                            //     TRUE
                            //     TRUE
                            //   }
                            // )
                            // One of the few cases it makes some miniscule
                            // sense to have more indent
                            function_ref.to_docs(config, doc_ref).cons(inner_docs)
                        } else {
                            function_ref.to_docs(config, doc_ref).cons(inner_docs)
                        }
                    } else {
                        function_ref.to_docs(config, doc_ref).cons(inner_docs)
                    }
                } else {
                    function_ref.to_docs(config, doc_ref).cons(inner_docs)
                }
            }
            Expression::SubsetExpression(subset_expression) => {
                let (object_ref, args) = (&subset_expression.object_ref, &subset_expression.args);
                object_ref
                    .to_docs(config, doc_ref)
                    .cons(args.to_docs(config, doc_ref))
                    .to_group(ShouldBreak::No, doc_ref)
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
                let is_body_bracketed_expression =
                    is_expression_bracketed_term_or_function_def(&Some(body));
                // I want this to break like this:
                // for (
                //   m in something
                // ) {
                //
                // }
                // for ( # comment
                //   m in something
                // ) # comment
                // { }
                // and
                // for (m in something) # comment
                // {}
                // So the inside of the parentheses should be
                // one group because I don't want breaks
                // outside of the parentheses to influence
                // the inside.
                keyword
                    .to_docs(config, doc_ref)
                    .cons(text!(" "))
                    .cons(
                        left_delim
                            .to_docs(config, doc_ref)
                            .cons(nl!(""))
                            .cons(identifier.to_docs(config, doc_ref))
                            .cons(text!(" "))
                            .cons(in_keyword.to_docs(config, doc_ref))
                            .cons(nl!(" "))
                            .cons(collection.to_docs(config, doc_ref))
                            .nest(config.indent())
                            .cons(nl!(""))
                            .to_group(ShouldBreak::No, doc_ref),
                    )
                    .cons(right_delim.to_docs(config, doc_ref))
                    // The below needs to be nl!(" ") in case
                    // the body is not a bracketed expression
                    .cons(if is_body_bracketed_expression {
                        text!(" ")
                    } else {
                        nl!(" ")
                    })
                    .cons(
                        body.to_docs(config, doc_ref)
                            .to_group(ShouldBreak::No, doc_ref),
                    )
                    .to_group(ShouldBreak::No, doc_ref)
            }
            Expression::LambdaFunction(lambda) => {
                let (keyword, args, body) = (&lambda.keyword, &lambda.args, &lambda.body);
                keyword
                    .to_docs(config, doc_ref)
                    .cons(
                        args.to_docs(config, doc_ref)
                            .to_group(ShouldBreak::No, doc_ref),
                    )
                    .cons(text!(" "))
                    .cons(body.to_docs(config, doc_ref))
                    .to_group(ShouldBreak::No, doc_ref)
            }
            Expression::MultiBop(lhs, other) => {
                assert!(!other.is_empty());
                let mut last_op: Option<&CommentedToken> = None;
                let mut acc_rhs: Rc<Doc> = Rc::new(Doc::Nil);
                for (op, rhs) in other.iter().rev() {
                    match last_op {
                        Some(last_op_token) => match last_op_token.token {
                            Token::OldAssign
                            | Token::LAssign
                            | Token::ColonAssign
                            | Token::SuperAssign
                                if !config.allow_nl_after_assignment()
                                    && last_op_token.inline_comment.is_none() =>
                            {
                                acc_rhs = rhs
                                    .to_docs(config, doc_ref)
                                    .cons(text!(" "))
                                    .cons(last_op_token.to_docs(config, doc_ref))
                                    .cons(text!(" "))
                                    .cons(acc_rhs);
                                last_op = Some(op);
                            }
                            Token::OldAssign
                            | Token::LAssign
                            | Token::ColonAssign
                            | Token::SuperAssign
                                if !config.allow_nl_after_assignment()
                                    && last_op_token.inline_comment.is_some() =>
                            {
                                acc_rhs = rhs
                                    .to_docs(config, doc_ref)
                                    .cons(text!(" "))
                                    .cons(last_op_token.to_docs(config, doc_ref))
                                    .cons(nl!(" "))
                                    .cons(acc_rhs.nest(config.indent()));
                                last_op = Some(op);
                            }
                            Token::OldAssign
                            | Token::LAssign
                            | Token::ColonAssign
                            | Token::SuperAssign
                            | Token::RAssign
                            | Token::Equal
                            | Token::NotEqual
                            | Token::LowerThan
                            | Token::GreaterThan
                            | Token::LowerEqual
                            | Token::GreaterEqual
                            | Token::Divide
                            | Token::Multiply
                            | Token::Minus
                            | Token::Plus
                            | Token::And
                            | Token::VectorizedAnd
                            | Token::Or
                            | Token::VectorizedOr
                            | Token::Pipe
                            | Token::Modulo
                            | Token::Tilde
                            | Token::Special(_) => {
                                acc_rhs = rhs
                                    .to_docs(config, doc_ref)
                                    .cons(text!(" "))
                                    .cons(last_op_token.to_docs(config, doc_ref))
                                    .to_group(ShouldBreak::No, doc_ref)
                                    .cons(nl!(" "))
                                    .cons(acc_rhs);
                                last_op = Some(op);
                            }
                            Token::Dollar
                            | Token::NsGet
                            | Token::NsGetInt
                            | Token::Colon
                            | Token::Slot
                            | Token::Power
                            | Token::Help => {
                                acc_rhs = rhs
                                    .to_docs(config, doc_ref)
                                    .cons(last_op_token.to_docs(config, doc_ref))
                                    .cons(acc_rhs);
                                last_op = Some(op);
                            }
                            _ => panic!(
                                "Got a not a binary operator token inside a binary expression when \
                     formatting. Token: {:?}",
                                &op.token
                            ),
                        },
                        None => {
                            last_op = Some(op);
                            acc_rhs = rhs
                                .to_docs(config, doc_ref)
                                .to_group(ShouldBreak::No, doc_ref);
                        }
                    }
                }
                if let Some(last_op) = last_op {
                    match last_op.token {
                        Token::OldAssign
                        | Token::LAssign
                        | Token::ColonAssign
                        | Token::SuperAssign
                            if !config.allow_nl_after_assignment()
                                && last_op.inline_comment.is_none() =>
                        {
                            lhs.to_docs(config, doc_ref)
                                .cons(text!(" "))
                                .cons(last_op.to_docs(config, doc_ref))
                                .cons(text!(" "))
                                .cons(acc_rhs)
                                .to_group(ShouldBreak::No, doc_ref)
                        }
                        Token::OldAssign
                        | Token::LAssign
                        | Token::ColonAssign
                        | Token::SuperAssign
                            if !config.allow_nl_after_assignment()
                                && last_op.inline_comment.is_some() =>
                        {
                            lhs.to_docs(config, doc_ref)
                                .cons(text!(" "))
                                .cons(last_op.to_docs(config, doc_ref))
                                .cons(nl!(" "))
                                .cons(acc_rhs.nest(config.indent()))
                                .to_group(ShouldBreak::No, doc_ref)
                        }
                        Token::OldAssign
                        | Token::LAssign
                        | Token::ColonAssign
                        | Token::SuperAssign
                        | Token::RAssign
                        | Token::Equal
                        | Token::NotEqual
                        | Token::LowerThan
                        | Token::GreaterThan
                        | Token::LowerEqual
                        | Token::GreaterEqual
                        | Token::Divide
                        | Token::Multiply
                        | Token::Minus
                        | Token::Plus
                        | Token::And
                        | Token::VectorizedAnd
                        | Token::Or
                        | Token::VectorizedOr
                        | Token::Pipe
                        | Token::Modulo
                        | Token::Tilde
                        | Token::Special(_) => lhs
                            .to_docs(config, doc_ref)
                            .cons(text!(" "))
                            .cons(last_op.to_docs(config, doc_ref))
                            .to_group(ShouldBreak::No, doc_ref)
                            .cons(nl!(" ").cons(acc_rhs).nest(config.indent()))
                            .to_group(ShouldBreak::No, doc_ref),
                        Token::Dollar
                        | Token::NsGet
                        | Token::NsGetInt
                        | Token::Colon
                        | Token::Slot
                        | Token::Power
                        | Token::Help => lhs
                            .to_docs(config, doc_ref)
                            .cons(last_op.to_docs(config, doc_ref))
                            .cons(acc_rhs)
                            .to_group(ShouldBreak::No, doc_ref),
                        _ => panic!(
                            "Got a not a binary operator token inside a binary expression when \
                     formatting. Token: {:?}",
                            &last_op
                        ),
                    }
                } else {
                    unreachable!("There's always the rhs")
                }
            }
        }
    }
}

impl Code for Args<'_> {
    fn to_docs(&self, config: &impl FormattingConfig, doc_ref: &mut usize) -> Rc<Doc> {
        let mut observed_doc = *doc_ref;
        // Hoist up the comment, so it's not part of the args group
        // This prevents line breaks in these situations:
        // c(1, 2, 3) # Comment
        //
        // We want the above instead of:
        // c(
        //   1,
        //   2,
        //   3
        // ) # Comment
        //
        // The latter might happen because the inline comment
        // is followed by a hard break, but at the same time
        // it should not impact the fits calculations of the line.
        let (right_delim, inline_comment) = self
            .right_delimeter
            .to_docs_with_separate_comments(config, doc_ref);
        match self.args.split_last() {
            Some((last_arg, other_args)) => {
                let other_args = other_args
                    .iter()
                    .map(|arg| {
                        arg.to_docs(config, doc_ref)
                            .to_group(ShouldBreak::No, doc_ref)
                    })
                    .collect::<Vec<_>>();
                let last_arg = std::iter::once(match &last_arg {
                    Arg::Proper(expression, _)
                        if is_expression_bracketed_term_or_function_def(&expression.as_ref()) =>
                    {
                        last_arg
                            .to_docs(config, doc_ref)
                            .to_group(ShouldBreak::No, doc_ref)
                            .nest(-config.indent())
                            .nest_if_break(config.indent(), observed_doc + 1)
                            .fits_until_l_bracket()
                    }
                    _ => last_arg
                        .to_docs(config, doc_ref)
                        .to_group(ShouldBreak::No, doc_ref),
                });
                let inside_delims = other_args
                .into_iter()
                .chain(last_arg)
                .reduce(|first, second| first.cons(nl!(" ")).cons(second))
                .expect(
                    "There is at least last_arg doc, otherwise we should be in the None match arm",
                )
                .to_group(ShouldBreak::No, &mut observed_doc);
                if let Some(inline) = inline_comment {
                    self.left_delimeter
                        .to_docs(config, doc_ref)
                        .cons(nl!("").cons(inside_delims).nest(config.indent()))
                        .cons(nl!(""))
                        .cons(right_delim)
                        .to_group(ShouldBreak::No, doc_ref)
                        .cons(text!(" "))
                        .cons(inline)
                } else {
                    self.left_delimeter
                        .to_docs(config, doc_ref)
                        .cons(nl!("").cons(inside_delims).nest(config.indent()))
                        .cons(nl!(""))
                        .cons(right_delim)
                }
            }
            None => match self.right_delimeter {
                Delimiter::SingleBracket(commented_token) | Delimiter::Paren(commented_token) => {
                    if commented_token.leading_comments.is_some() {
                        self.left_delimeter
                            .to_docs(config, doc_ref)
                            .cons(nl!("").nest(config.indent()))
                            .cons(self.right_delimeter.to_docs(config, doc_ref))
                            .to_group(ShouldBreak::Yes, doc_ref)
                    } else {
                        self.left_delimeter
                            .to_docs(config, doc_ref)
                            .cons(self.right_delimeter.to_docs(config, doc_ref))
                    }
                }
                Delimiter::DoubleBracket((first_commented_token, _)) => {
                    if first_commented_token.leading_comments.is_some() {
                        self.left_delimeter
                            .to_docs(config, doc_ref)
                            .cons(nl!("").nest(config.indent()))
                            .cons(self.right_delimeter.to_docs(config, doc_ref))
                            .to_group(ShouldBreak::Yes, doc_ref)
                    } else {
                        self.left_delimeter
                            .to_docs(config, doc_ref)
                            .cons(self.right_delimeter.to_docs(config, doc_ref))
                    }
                }
            },
        }
    }
}

impl Code for Arg<'_> {
    fn to_docs(&self, config: &impl FormattingConfig, doc_ref: &mut usize) -> Rc<Doc> {
        match self {
            Arg::Proper(expr, comma) => {
                if let Some(comma) = comma {
                    expr.to_docs(config, doc_ref)
                        .cons(comma.to_docs(config, doc_ref))
                } else {
                    expr.to_docs(config, doc_ref)
                }
            }
            Arg::EmptyEqual(arg_name, equal_sign, comma) => arg_name
                .to_docs(config, doc_ref)
                .cons(text!(" "))
                .cons(equal_sign.to_docs(config, doc_ref))
                .cons(text!(" "))
                .cons(comma.to_docs(config, doc_ref)),
        }
    }
}

fn is_expression_bracketed_term_or_function_def(expr: &Option<&Expression>) -> bool {
    expr.as_ref().is_some_and(|expr| match expr {
        Expression::Term(term) => {
            term.pre_delimiters
                .is_some_and(|pre_delim| matches!(pre_delim.token, Token::LBrace))
                && !is_term_embracing_op(term)
        }
        Expression::FunctionDef(_) => true,
        _ => false,
    })
}

fn is_term_embracing_op(term: &TermExpr) -> bool {
    if let Some(pre_delim) = term.pre_delimiters {
        if matches!(pre_delim.token, Token::LBrace)
            && !term.term.is_empty()
            && matches!(term.term[0], Expression::Term(_))
        {
            let first_expr = &term.term[0];
            if let Expression::Term(inner_term) = first_expr {
                return inner_term
                    .pre_delimiters
                    .is_some_and(|pre_delim| matches!(pre_delim.token, Token::LBrace));
            }
        }
    }
    false
}

/// Forced line breaks are line breaks inside a group
/// with ShouldBreak::Yes
fn has_forced_line_breaks(doc: &Rc<Doc>, inside_a_group_with_should_break: bool) -> bool {
    match doc.deref() {
        Doc::Nil => false,
        Doc::Cons(first, second, _) => {
            has_forced_line_breaks(first, inside_a_group_with_should_break)
                || has_forced_line_breaks(second, inside_a_group_with_should_break)
        }
        Doc::Text(_, _, _) => false,
        Doc::Nest(_, inner, _) => has_forced_line_breaks(inner, inside_a_group_with_should_break),
        Doc::NestIfBreak(_, inner, _, _) => {
            has_forced_line_breaks(inner, inside_a_group_with_should_break)
        }
        Doc::NestHanging(inner, _) => {
            has_forced_line_breaks(inner, inside_a_group_with_should_break)
        }
        Doc::FitsUntilLBracket(inner, _) => {
            has_forced_line_breaks(inner, inside_a_group_with_should_break)
        }
        Doc::Break(_) => inside_a_group_with_should_break,
        Doc::Group(group_props, _) => has_forced_line_breaks(
            &group_props.0,
            matches!(group_props.1, ShouldBreak::Yes)
                || matches!(group_props.1, ShouldBreak::Propagate),
        ),
        Doc::HardBreak => true,
    }
}

/// Delimited content requires special care with comments at the end of it...
fn delimited_content_to_docs(
    left_delim: &CommentedToken<'_>,
    inner: Rc<Doc>,
    right_delim: &CommentedToken<'_>,
    config: &impl FormattingConfig,
    doc_ref: &mut usize,
    should_break: ShouldBreak,
) -> Rc<Doc> {
    let nl = || match left_delim.token {
        Token::LParen => nl!(""),
        Token::LBrace => nl!(" "),
        _ => unreachable!("Non parenthesis argument as the delimiter"),
    };
    if let Some(right_delim_leading_comments) = &right_delim.leading_comments {
        let mut leading_comments_it = right_delim_leading_comments.iter();
        let mut leading_comments = text!(leading_comments_it.next().unwrap());
        for comment in leading_comments_it {
            leading_comments = leading_comments.cons(nl!("")).cons(text!(comment, 0));
        }
        let leading_comments = leading_comments
            .nest_hanging()
            .to_group(ShouldBreak::Yes, doc_ref);
        left_delim
            .to_docs(config, doc_ref)
            .cons(
                nl().cons(inner)
                    .cons(nl!(""))
                    .cons(leading_comments)
                    .nest(config.indent()),
            )
            .cons(nl())
            .cons(right_delim.to_docs_without_leading_comments(config, doc_ref))
            .to_group(ShouldBreak::Yes, doc_ref)
    } else {
        left_delim
            .to_docs(config, doc_ref)
            .cons(nl().cons(inner).nest(config.indent()))
            .cons(nl())
            .cons(right_delim.to_docs_without_leading_comments(config, doc_ref))
            .to_group(should_break, doc_ref)
    }
}

fn is_closure_with_brackets(expr: &Expression) -> bool {
    if let Expression::Term(term) = expr {
        term.pre_delimiters
            .is_some_and(|pre_delim| matches!(pre_delim.token, Token::LBrace))
    } else {
        false
    }
}
