// Implementing Wadler and https://lindig.github.io/papers/strictly-pretty-2000.pdf
use std::collections::VecDeque;
use std::ops::Add;
use std::rc::Rc;

use log::trace;

use crate::config::FormattingConfig;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ShouldBreak {
    Yes,
    No,
}

/// ShouldBreak is a linebreak that propagates to the parents
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GroupDocProperties(pub(crate) Rc<Doc>, pub(crate) ShouldBreak); // (doc, should parents break?)

#[derive(Debug, Clone, PartialEq, Copy)]
pub(crate) enum InlineCommentPosition {
    No,
    Middle,
    End,
}

impl Add for InlineCommentPosition {
    type Output = InlineCommentPosition;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (InlineCommentPosition::No, InlineCommentPosition::No) => InlineCommentPosition::No,
            (InlineCommentPosition::No, InlineCommentPosition::Middle) => {
                InlineCommentPosition::Middle
            }
            (InlineCommentPosition::No, InlineCommentPosition::End) => InlineCommentPosition::End,
            (InlineCommentPosition::Middle, _) => InlineCommentPosition::Middle,
            (InlineCommentPosition::End, _) => InlineCommentPosition::Middle,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CommonProperties(pub(crate) InlineCommentPosition);
impl Default for CommonProperties {
    fn default() -> Self {
        CommonProperties(InlineCommentPosition::No)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Doc {
    Nil,
    Cons(Rc<Doc>, Rc<Doc>, CommonProperties),
    Text(Rc<str>, usize, CommonProperties), // text, text length
    Nest(i32, Rc<Doc>, CommonProperties),   // indent size, doc
    Break(&'static str),
    Group(GroupDocProperties, CommonProperties),
}

impl std::fmt::Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Doc::Nil => f.write_str("Nil"),
            Doc::Cons(left, right, _) => f.write_fmt(format_args!("{} + {}", left, right)),
            Doc::Text(text, _, _) => f.write_fmt(format_args!("'{}'", text)),
            Doc::Nest(indent, body, _) => f.write_fmt(format_args!("Nest{}({})", indent, body)),
            Doc::Break(newline) => f.write_fmt(format_args!("NL({})", newline)),
            Doc::Group(inside, _) => f.write_fmt(format_args!("SB:{:?}<{}>", inside.1, inside.0)),
        }
    }
}

pub(crate) fn query_inline_position(doc: &Doc) -> InlineCommentPosition {
    match doc {
        Doc::Nil => InlineCommentPosition::No,
        Doc::Cons(_, _, props) => props.0,
        Doc::Text(_, _, props) => props.0,
        Doc::Nest(_, _, props) => props.0,
        Doc::Break(_) => InlineCommentPosition::No,
        Doc::Group(_, props) => props.0,
    }
}

pub trait DocAlgebra {
    fn cons(self, other: Rc<Doc>) -> Rc<Doc>;
    fn to_group(self, should_break: ShouldBreak) -> Rc<Doc>;
    fn nest(self, indent: i32) -> Rc<Doc>;
}

impl DocAlgebra for Rc<Doc> {
    fn cons(self, other: Rc<Doc>) -> Rc<Doc> {
        let properties =
            CommonProperties(query_inline_position(&self) + query_inline_position(&other));
        Rc::new(Doc::Cons(self, other, properties))
    }

    fn to_group(self, should_break: ShouldBreak) -> Rc<Doc> {
        let properties = CommonProperties(query_inline_position(&self));
        Rc::new(Doc::Group(
            GroupDocProperties(self, should_break),
            properties,
        ))
    }

    fn nest(self, indent: i32) -> Rc<Doc> {
        let properties = CommonProperties(query_inline_position(&self));
        Rc::new(Doc::Nest(indent, self, properties))
    }
}

pub(crate) struct DocBuffer<'a>(pub(crate) &'a VecDeque<(i32, Mode, Rc<Doc>)>);

impl std::fmt::Display for DocBuffer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for doc in self.0 {
            f.write_fmt(format_args!("{}, ", doc.2))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum SimpleDoc {
    Nil,
    Text(Rc<str>, Rc<SimpleDoc>),
    Line(usize, Rc<SimpleDoc>),
}

pub(crate) fn simple_doc_to_string(doc: Rc<SimpleDoc>) -> String {
    match &*doc {
        SimpleDoc::Nil => "".to_string(),
        SimpleDoc::Text(s, doc) => format!("{s}{}", simple_doc_to_string(Rc::clone(doc))),
        SimpleDoc::Line(indent, doc) => format!(
            "\n{:width$}{}",
            "",
            simple_doc_to_string(Rc::clone(doc)),
            width = indent
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Mode {
    Flat,
    Break,
}

pub(crate) type Triple = (i32, Mode, Rc<Doc>);

fn fits(remaining_width: i32, docs: &mut VecDeque<Triple>) -> bool {
    if remaining_width < 0 {
        false
    } else {
        match docs.pop_front() {
            None => true,
            Some((indent, mode, doc)) => match (indent, mode, &*doc) {
                (_, _, Doc::Nil) => fits(remaining_width, docs),
                (i, m, Doc::Cons(first, second, CommonProperties(inline_comment_pos))) => {
                    if inline_comment_pos == &InlineCommentPosition::Middle {
                        false
                    } else {
                        docs.push_front((i, m, Rc::clone(second)));
                        docs.push_front((i, m, Rc::clone(first)));
                        fits(remaining_width, docs)
                    }
                }
                (i, m, Doc::Nest(step, doc, _)) => {
                    docs.push_front((i + step, m, Rc::clone(doc)));
                    fits(remaining_width, docs)
                }
                (_, _, Doc::Text(_, s_len, _)) => fits(remaining_width - *s_len as i32, docs),
                (_, Mode::Flat, Doc::Break(s)) => fits(remaining_width - s.len() as i32, docs),
                (_, Mode::Break, Doc::Break(_)) => unreachable!(),
                (i, _, Doc::Group(groupped_doc, CommonProperties(inline_comment_pos))) => {
                    if inline_comment_pos == &InlineCommentPosition::Middle {
                        trace!("Fits false for {groupped_doc:?}");
                        false
                    } else {
                        docs.push_front((i, Mode::Flat, Rc::clone(&groupped_doc.0)));
                        let f = fits(remaining_width, docs);
                        trace!("Remainging width: {remaining_width} Fits {f} for {groupped_doc:?}");
                        f
                    }
                }
            },
        }
    }
}

pub(crate) fn format_to_sdoc(
    consumed: i32,
    docs: &mut VecDeque<Triple>,
    config: &impl FormattingConfig,
) -> SimpleDoc {
    let line_length = config.line_length();
    match docs.pop_front() {
        None => SimpleDoc::Nil,
        Some(doc) => {
            let (indent, mode, doc) = doc;
            match (indent, mode, &*doc) {
                (_, _, Doc::Nil) => format_to_sdoc(consumed, docs, config),
                (i, m, Doc::Cons(first, second, _)) => {
                    docs.push_front((i, m, Rc::clone(second)));
                    docs.push_front((i, m, Rc::clone(first)));
                    format_to_sdoc(consumed, docs, config)
                }
                (i, m, Doc::Nest(step, doc, _)) => {
                    docs.push_front((i + step, m, Rc::clone(doc)));
                    format_to_sdoc(consumed, docs, config)
                }
                (_, _, Doc::Text(s, width, _)) => {
                    let length = *width as i32;
                    SimpleDoc::Text(
                        Rc::clone(s),
                        Rc::new(format_to_sdoc(consumed + length, docs, config)),
                    )
                }
                (_, Mode::Flat, Doc::Break(s)) => {
                    let length = s.len() as i32;
                    SimpleDoc::Text(
                        Rc::from(*s),
                        Rc::new(format_to_sdoc(consumed + length, docs, config)),
                    )
                }
                (i, Mode::Break, Doc::Break(_)) => {
                    SimpleDoc::Line(i as usize, Rc::new(format_to_sdoc(i, docs, config)))
                }
                (i, _, Doc::Group(groupped_doc, CommonProperties(inline_comment_pos))) => {
                    let mut group_docs =
                        VecDeque::from([(i, Mode::Flat, Rc::clone(&groupped_doc.0))]);
                    if groupped_doc.1 == ShouldBreak::Yes
                        || matches!(inline_comment_pos, InlineCommentPosition::Middle)
                        || !fits(line_length - consumed, &mut group_docs)
                    {
                        docs.push_front((i, Mode::Break, Rc::clone(&groupped_doc.0)));
                        format_to_sdoc(consumed, docs, config)
                    } else {
                        docs.push_front((i, Mode::Flat, Rc::clone(&groupped_doc.0)));
                        format_to_sdoc(consumed, docs, config)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    struct MockConfig;

    impl FormattingConfig for MockConfig {
        fn line_length(&self) -> i32 {
            120
        }
        fn indent(&self) -> i32 {
            0
        }
        fn embracing_op_no_nl(&self) -> bool {
            true
        }

        fn allow_nl_after_assignment(&self) -> bool {
            true
        }
    }
    impl std::fmt::Display for MockConfig {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }

    #[test]
    fn printing_text_doc() {
        log_init();
        let mut doc = VecDeque::from([(
            0i32,
            Mode::Flat,
            Rc::new(Doc::Text(Rc::from("Test"), 4, CommonProperties::default())),
        )]);
        let mock_config = MockConfig {};
        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config));

        assert_eq!(simple_doc_to_string(sdoc), "Test")
    }

    #[test]
    fn should_break_breaks_even_when_fits_the_line() {
        log_init();
        let mut doc = VecDeque::from([(
            0i32,
            Mode::Flat,
            Rc::new(Doc::Group(
                GroupDocProperties(
                    Rc::new(Doc::Cons(
                        Rc::new(Doc::Text(Rc::from("Test"), 4, CommonProperties::default())),
                        Rc::new(Doc::Cons(
                            Rc::new(Doc::Break(" ")),
                            Rc::new(Doc::Text(Rc::from("Test2"), 5, CommonProperties::default())),
                            CommonProperties::default(),
                        )),
                        CommonProperties::default(),
                    )),
                    ShouldBreak::Yes,
                ),
                CommonProperties::default(),
            )),
        )]);
        let mock_config = MockConfig {};
        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config));

        assert_eq!(simple_doc_to_string(sdoc), "Test\nTest2")
    }
}
