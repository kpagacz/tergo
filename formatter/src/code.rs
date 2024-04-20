use crate::config::FormattingConfig;

use log::trace;
use parser::ast::{Arg, Args, Expression, IfConditional, TermExpr};
use tokenizer::tokens::CommentedToken;

use crate::format::{Doc, GroupDocProperties, ShouldBreak};
use std::rc::Rc;
use tokenizer::Token;

pub(crate) trait Code {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc>;
}

// Macro that creates a Doc::Group
macro_rules! group {
    ($doc:expr) => {
        Rc::new(Doc::Group(GroupDocProperties($doc, ShouldBreak::No)))
    };
    ($doc:expr, $should_break: expr) => {
        Rc::new(Doc::Group(GroupDocProperties($doc, $should_break)))
    };
}

// Macro that creates a Doc::Nest
macro_rules! nest {
    ($indent:expr, $doc:expr) => {
        Rc::new(Doc::Nest($indent, $doc))
    };
}

// Macro that creates a Doc::Cons
macro_rules! cons {
    ($first:expr, $second:expr) => {
        Rc::new(Doc::Cons($first, $second))
    };
}

// Macro that creates a Doc::Break
macro_rules! nl {
    ($txt:expr) => {
        Rc::new(Doc::Break($txt))
    };
}

// Macro that creates a Doc::Text
macro_rules! text {
    ($txt: expr) => {
        Rc::new(Doc::Text(Rc::from($txt)))
    };
}

// Macro that surrounds a doc with parentheses
macro_rules! delimited_doc {
    ($doc:expr, $ldelim: expr, $rdelim: expr) => {
        Rc::new(Doc::Group(GroupDocProperties(
            Rc::new(Doc::Cons(
                $ldelim,
                Rc::new(Doc::Cons(
                    Rc::new(Doc::Break("")),
                    Rc::new(Doc::Cons(
                        $doc,
                        Rc::new(Doc::Cons(Rc::new(Doc::Break("")), $rdelim)),
                    )),
                )),
            )),
            ShouldBreak::No,
        )))
    };
}

// TODO: Make this a macro
pub(crate) fn with_optional_break(
    first_doc: Rc<Doc>,
    second_doc: Rc<Doc>,
    break_text: &'static str,
) -> Rc<Doc> {
    cons!(cons!(first_doc, nl!(break_text)), second_doc)
}

impl<'a> Code for Token<'a> {
    fn to_docs(&self, _: &impl FormattingConfig) -> Rc<Doc> {
        match self {
            Token::Symbol(s) | Token::Literal(s) => Rc::new(Doc::Text(Rc::from(*s))),
            Token::Semicolon => Rc::new(Doc::Text(Rc::from(";"))),
            Token::Newline => Rc::new(Doc::Text(Rc::from("\n"))),
            Token::LParen => Rc::new(Doc::Text(Rc::from("("))),
            Token::RParen => Rc::new(Doc::Text(Rc::from(")"))),
            Token::LBrace => Rc::new(Doc::Text(Rc::from("{"))),
            Token::RBrace => Rc::new(Doc::Text(Rc::from("}"))),
            Token::LSubscript => Rc::new(Doc::Text(Rc::from("["))),
            Token::RSubscript => Rc::new(Doc::Text(Rc::from("]"))),
            Token::Comma => Rc::new(Doc::Text(Rc::from(","))),
            Token::Continue => Rc::new(Doc::Text(Rc::from("continue"))),
            Token::Break => Rc::new(Doc::Text(Rc::from("break"))),
            Token::If => Rc::new(Doc::Text(Rc::from("if"))),
            Token::Else => Rc::new(Doc::Text(Rc::from("else"))),
            Token::While => Rc::new(Doc::Text(Rc::from("while"))),
            Token::For => Rc::new(Doc::Text(Rc::from("for"))),
            Token::Repeat => Rc::new(Doc::Text(Rc::from("repeat"))),
            Token::In => Rc::new(Doc::Text(Rc::from("in"))),
            Token::Function => Rc::new(Doc::Text(Rc::from("function"))),
            Token::Lambda => Rc::new(Doc::Text(Rc::from("\\"))),
            Token::LAssign => Rc::new(Doc::Text(Rc::from("<-"))),
            Token::RAssign => Rc::new(Doc::Text(Rc::from("->"))),
            Token::OldAssign => Rc::new(Doc::Text(Rc::from("="))),
            Token::Equal => Rc::new(Doc::Text(Rc::from("=="))),
            Token::NotEqual => Rc::new(Doc::Text(Rc::from("!="))),
            Token::LowerThan => Rc::new(Doc::Text(Rc::from("<"))),
            Token::GreaterThan => Rc::new(Doc::Text(Rc::from(">"))),
            Token::LowerEqual => Rc::new(Doc::Text(Rc::from("<="))),
            Token::GreaterEqual => Rc::new(Doc::Text(Rc::from(">="))),
            Token::Power => Rc::new(Doc::Text(Rc::from("^"))),
            Token::Divide => Rc::new(Doc::Text(Rc::from("/"))),
            Token::Multiply => Rc::new(Doc::Text(Rc::from("*"))),
            Token::Minus => Rc::new(Doc::Text(Rc::from("-"))),
            Token::Plus => Rc::new(Doc::Text(Rc::from("+"))),
            Token::Help => Rc::new(Doc::Text(Rc::from("?"))),
            Token::And => Rc::new(Doc::Text(Rc::from("&&"))),
            Token::VectorizedAnd => Rc::new(Doc::Text(Rc::from("&"))),
            Token::Or => Rc::new(Doc::Text(Rc::from("||"))),
            Token::VectorizedOr => Rc::new(Doc::Text(Rc::from("|"))),
            Token::Dollar => Rc::new(Doc::Text(Rc::from("$"))),
            Token::Pipe => Rc::new(Doc::Text(Rc::from("|>"))),
            Token::Modulo => Rc::new(Doc::Text(Rc::from("%"))),
            Token::NsGet => Rc::new(Doc::Text(Rc::from("::"))),
            Token::NsGetInt => Rc::new(Doc::Text(Rc::from(":::"))),
            Token::Tilde => Rc::new(Doc::Text(Rc::from("~"))),
            Token::Colon => Rc::new(Doc::Text(Rc::from(":"))),
            Token::Slot => Rc::new(Doc::Text(Rc::from("@"))),
            Token::Special(s) => Rc::new(Doc::Text(Rc::from(*s))),
            Token::UnaryNot => Rc::new(Doc::Text(Rc::from("!"))),
            Token::InlineComment(s) => Rc::new(Doc::Text(Rc::from(*s))),
            Token::Comment(s) => Rc::new(Doc::Text(Rc::from(*s))),
            Token::EOF => Rc::new(Doc::Break("")),
        }
    }
}

impl Code for CommentedToken<'_> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        // let mut docs = VecDeque::new();
        // // for comment in self.leading_comments {
        //     docs.push_back(comment.to_docs());
        //     // TODO: check if this works
        //     // Force a new line (I am not sure if the code already does it somewhere else)
        //     docs.push_back((INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("\n")))));
        // }
        // docs.push_back(self.token.to_docs(config));
        // // if let Some(inline) = &self.inline_comment {
        //     docs.push_back(inline.to_docs());
        // }

        self.token.to_docs(config)
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
            res = Rc::new(Doc::Cons(res, first_doc));
        }
    }

    for next_doc in docs {
        res = Rc::new(Doc::Cons(
            res,
            cons!(Rc::clone(&separator), Rc::new(Doc::Break(" "))),
        ));
        res = Rc::new(Doc::Cons(res, next_doc));
    }

    res = Rc::new(Doc::Group(GroupDocProperties(res, should_break)));
    trace!("joined_docs to: {res:?}");
    res
}

/// Returns the docs of the term and the suffix after the last potential line break
fn term_expression_to_docs_with_prefix<Config>(
    term: &TermExpr,
    prefix_docs: Rc<Doc>,
    should_break: ShouldBreak,
    config: &Config,
) -> (Rc<Doc>, Rc<Doc>)
where
    Config: FormattingConfig,
{
    let (pre_delim, xprs, post_delim) = (term.pre_delimiters, &term.term, term.post_delimiters);
    let mut suffix = Rc::new(Doc::Nil);
    let mut docs: Rc<Doc>;
    match (pre_delim, xprs, post_delim) {
        (Some(pre), xprs, Some(post)) if xprs.is_empty() => {
            docs = cons!(pre.to_docs(config), post.to_docs(config));
            docs = cons!(prefix_docs, docs);
        }
        (Some(pre), xprs, Some(post)) if matches!(pre.token, Token::LBrace) => {
            // Brace-delimited terms - put a space between the delimiters and the body
            trace!("to_docs for the term with curly brace expressions: {xprs:?}");
            let body_doc = join_docs(
                xprs.iter().map(|t| t.to_docs(config)),
                Rc::new(Doc::Nil),
                should_break,
                config,
            );
            let pre = cons!(prefix_docs, cons!(pre.to_docs(config), nl!(" ")));
            docs = cons!(pre, body_doc);
            suffix = post.to_docs(config);
        }
        (Some(pre), xprs, Some(post)) => {
            // Do not put the space between the delimiters and the body
            docs = delimited_doc!(
                join_docs(
                    xprs.iter().map(|t| t.to_docs(config)),
                    Rc::new(Doc::Nil),
                    should_break,
                    config
                ),
                cons!(prefix_docs, pre.to_docs(config)),
                Rc::new(Doc::Nil)
            );
            suffix = post.to_docs(config);
        }
        _ => panic!("A term without matching delimiteres encountered"),
    };

    (docs, suffix)
}

impl<'a> Code for Expression<'a> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        let indent = config.indent();

        match self {
            Expression::Symbol(token) | Expression::Literal(token) | Expression::Comment(token) => {
                token.to_docs(config)
            }
            Expression::Term(term_expr) => {
                match term_expr.pre_delimiters {
                    Some(pre_delim) if matches!(pre_delim.token, Token::LBrace) => {
                        let (body, suffix) = term_expression_to_docs_with_prefix(term_expr, Rc::new(Doc::Nil), ShouldBreak::Yes, config);
                        if matches!(&*suffix, Doc::Nil) {
                            group!(body)
                        } else {
                            group!(cons!(cons!(body, nl!(" ")), suffix), ShouldBreak::Yes)
                        }
                    },
                    _ => {
                        let (body, suffix) = term_expression_to_docs_with_prefix(term_expr, Rc::new(Doc::Nil), ShouldBreak::No, config);
                        cons!(body, suffix)
                    },

                }
            }
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
                | Token::Special(_) => {
                    group!(nest!(
                        indent,
                        with_optional_break(
                            cons!(cons!(lhs.to_docs(config), text!(" ")), op.to_docs(config)),
                            rhs.to_docs(config),
                            " "
                        )
                    ))
                },
                Token::Dollar | Token::NsGet | Token::NsGetInt | Token::Colon | Token::Slot => {
                    group!(nest!(
                        indent,
                        with_optional_break(
                            cons!(cons!(lhs.to_docs(config), text!("")), op.to_docs(config)),
                            rhs.to_docs(config),
                            ""
                        )
                    ))
                },
                _ => panic!("Got a not a binary operator token inside a binary expression when formatting. Token: {:?}", &op.token)
            },
            Expression::Newline(_) => Rc::new(Doc::Break("\n")),
            Expression::EOF(_) => Rc::new(Doc::Nil),
            Expression::Whitespace(_) => Rc::new(Doc::Break("\n")),
            Expression::FunctionDef(function_def) =>  {
                // function(<potential_break>args) {<hard_break>body<hard_break>}
               let (_, args, body) = (function_def.keyword, &function_def.arguments, &function_def.body) ;
                let keyword = cons!(text!("function"), args.left_delimeter.to_docs(config));
                let args_doc = join_docs(
                    args.args.iter().map(|arg| cons!(arg.0.to_docs(config), arg.1.as_ref().map(|sep| sep.to_docs(config)).unwrap_or(Rc::new(Doc::Nil)))),
                    Rc::new(Doc::Nil),
                    ShouldBreak::No,
                    config
                );
                let args_with_delimiter = delimited_doc!(args_doc, Rc::new(Doc::Nil), cons!(args.right_delimeter.to_docs(config), cons!(text!(" "), nl!(""))));
                let body_doc = body.to_docs(config);

                let keyword_plus_args_part = group!(cons!(keyword, args_with_delimiter));
                group!(cons!(keyword_plus_args_part, body_doc), ShouldBreak::No)
            },
            Expression::IfExpression(if_expression) => {
                let (if_conditional, else_ifs, trailing_else) = (&if_expression.if_conditional, &if_expression.else_ifs, &if_expression.trailing_else);
                let mut docs = Rc::new(Doc::Nil);
                let (conditional_docs, mut conditional_suffix) = if_conditional_to_docs(Rc::new(Doc::Nil), if_conditional, config);
                docs = cons!(docs, conditional_docs);
                // if (<potential_break>condition<potential_break>) [{]<potential_break>
                // body<potential_break>
                // } else if(<potential_break>...) {
                // } else if(...) { ... 
                for else_if in else_ifs {
                let (&keyword, body) = (&else_if.else_keyword, &else_if.if_conditional);
                    let prefix = cons!(cons!(conditional_suffix, text!(" ")), keyword.to_docs(config));
                    let (else_if_docs, new_suffix) = if_conditional_to_docs(prefix, body, config);
                    docs = cons!(docs, else_if_docs);
                    conditional_suffix = new_suffix;
                }
                if let Some(trailing_else) = trailing_else {
                    match trailing_else.body.as_ref() {
                        Expression::Term(term_expr) => {
                            let trailing_else_prefix = cons!(
                                cons!(conditional_suffix, text!(" ")),
                                cons!(trailing_else.else_keyword.to_docs(config), text!(" "))
                            );
                            let (body_docs, suffix) = term_expression_to_docs_with_prefix(
                                term_expr,
                                trailing_else_prefix,
                                ShouldBreak::Yes,
                                config
                           );
                            if !matches!(&*suffix, Doc::Nil) {
                                docs = cons!(docs, group!(cons!(body_docs, nl!(""))));
                            } else {
                                docs = cons!(docs, group!(body_docs));
                            }

                            conditional_suffix = suffix;
                        }
                        _ => {
                            conditional_suffix = cons!(conditional_suffix, trailing_else.body.to_docs(config));
                        }
                    }
                }
                docs = cons!(docs, conditional_suffix);

                docs
            }
            Expression::WhileExpression(while_expression) => {
                let (keyword, condition, body) = (&while_expression.while_keyword, &while_expression.condition, &while_expression.body);
                    group!(cons!(cons!(cons!(
                        keyword.to_docs(config), condition.to_docs(config)), text!(" ")), body.to_docs(config)
                    ))
            }
            Expression::RepeatExpression(repeat_expression) => {
                let (keyword, body) = (&repeat_expression.repeat_keyword, &repeat_expression.body);
                group!(cons!(keyword.to_docs(config), body.to_docs(config)))
            }
            Expression::FunctionCall(function_call) => {
                let (function_ref, args) = (&function_call.function_ref, &function_call.args);
                group!(cons!(function_ref.to_docs(config), args.to_docs(config)))
            }
        }
    }
}

/// Returns the docs of the if conditional and the suffix after the last potential line break
fn if_conditional_to_docs(
    prefix_docs: Rc<Doc>,
    if_conditional: &IfConditional,
    config: &impl FormattingConfig,
) -> (Rc<Doc>, Rc<Doc>) {
    let mut docs = cons!(
        cons!(
            if !matches!(prefix_docs.as_ref(), Doc::Nil) {
                cons!(prefix_docs, text!(" "))
            } else {
                prefix_docs
            },
            cons!(if_conditional.keyword.to_docs(config), text!(" "))
        ),
        if_conditional.left_delimiter.to_docs(config)
    );
    docs = cons!(docs, nl!(""));
    docs = cons!(
        docs,
        cons!(if_conditional.condition.to_docs(config), nl!(""))
    );
    docs = group!(docs);
    let mut next_group = cons!(if_conditional.right_delimiter.to_docs(config), text!(" "));
    let mut suffix = Rc::new(Doc::Nil);
    match if_conditional.body.as_ref() {
        Expression::Term(term_expr) => {
            let (body, new_suffix) = term_expression_to_docs_with_prefix(
                term_expr,
                next_group,
                ShouldBreak::Yes,
                config,
            );
            next_group = body;
            if !matches!(&*new_suffix, Doc::Nil) {
                next_group = cons!(next_group, nl!(""));
            }
            suffix = new_suffix;
        }
        _ => {
            next_group = cons!(next_group, if_conditional.body.to_docs(config));
        }
    }
    docs = cons!(docs, group!(next_group));
    (docs, suffix)
}

impl Code for Args<'_> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        let mut docs = cons!(self.left_delimeter.to_docs(config), nl!(""));
        let mut it = self.args.iter();
        if let Some(arg) = it.next() {
            docs = cons!(docs, arg.to_docs(config));
        }
        for arg in it {
            docs = cons!(docs, nl!(" "));
            docs = cons!(docs, arg.to_docs(config));
        }
        docs = cons!(docs, self.right_delimeter.to_docs(config));
        group!(docs)
    }
}
impl Code for Arg<'_> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        if let Some(comma) = &self.1 {
            cons!(self.0.to_docs(config), comma.to_docs(config))
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
        let docs = [
            Rc::new(Doc::Text(Rc::from("test"))),
            Rc::new(Doc::Text(Rc::from("test2"))),
        ];
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
        let docs = [Rc::new(Doc::Text(Rc::from("test")))];
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
