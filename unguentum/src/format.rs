// Implementing Wadler and https://lindig.github.io/papers/strictly-pretty-2000.pdf
use std::collections::{HashSet, VecDeque};
use std::ops::Add;
use std::rc::Rc;

use log::trace;

use crate::config::FormattingConfig;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ShouldBreak {
    Yes,
    No,
}

/// ShouldBreak is a linebreak that propagates to the parents
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GroupDocProperties(pub(crate) Rc<Doc>, pub(crate) ShouldBreak); // (doc, should parents break?)

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub(crate) enum InlineCommentPosition {
    No,
    Middle,
    End,
    InGroup,
}

impl Add for InlineCommentPosition {
    type Output = InlineCommentPosition;

    fn add(self, rhs: Self) -> Self::Output {
        use InlineCommentPosition::*;
        match (self, rhs) {
            (No, No) => No,
            (End, _) | (Middle, _) | (No, Middle) => Middle,
            (No, End) => End,
            (InGroup, position) => position,
            (No, InGroup) => No,
        }
    }
}

/// inlineCommentPosition, doc reference
#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub(crate) struct CommonProperties(pub(crate) InlineCommentPosition, pub(crate) usize);
impl Default for CommonProperties {
    fn default() -> Self {
        CommonProperties(InlineCommentPosition::No, 0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Doc {
    Nil,
    Cons(Rc<Doc>, Rc<Doc>, CommonProperties),
    Text(Rc<str>, usize, CommonProperties), // text, text length
    Nest(i32, Rc<Doc>, CommonProperties),   // indent size, doc
    // This NestIfBreak supports an important layout feature of
    // tidyverse styleguide for R, e.g.
    // test_that("something", {
    //   TRUE
    // })
    // The above piece of code:
    // * function arguments are nested by 2 by default (from test_that)
    // * closures are normally nested by 2 by default
    // * but the inside of the closure is intended only by 2
    // The content of the closure is basically indented only if the
    // group for all function arguments breaks, e.g.
    // test_that(
    //   "very very long name",
    //   {
    //     TRUE
    //   }
    // )
    NestIfBreak(i32, Rc<Doc>, CommonProperties, usize), // indent size, indented doc, props, possibly broken doc
    NestHanging(Rc<Doc>, CommonProperties),
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
            Doc::NestIfBreak(indent, body, _, watched) => {
                write!(f, "NestIfBreakRef{watched}Ind{indent}({body})")
            }
            Doc::NestHanging(body, _) => write!(f, "NestHanging({body})"),
            Doc::Break(newline) => f.write_fmt(format_args!("NL({})", newline)),
            Doc::Group(inside, common_props) => f.write_fmt(format_args!(
                "GROUP{}:CommPos{:?}:SB{:?}<{}>",
                common_props.1, common_props.0, inside.1, inside.0
            )),
        }
    }
}

pub(crate) fn query_inline_position(doc: &Doc) -> InlineCommentPosition {
    match doc {
        Doc::Nil => InlineCommentPosition::No,
        Doc::Cons(_, _, props) => props.0,
        Doc::Text(_, _, props) => props.0,
        Doc::Nest(_, _, props) => props.0,
        Doc::NestIfBreak(_, _, props, _) => props.0,
        Doc::NestHanging(_, props) => props.0,
        Doc::Break(_) => InlineCommentPosition::No,
        Doc::Group(_, props) => props.0,
    }
}

pub trait DocAlgebra {
    fn cons(self, other: Rc<Doc>) -> Rc<Doc>;
    fn to_group(self, should_break: ShouldBreak, doc_ref: &mut usize) -> Rc<Doc>;
    fn nest(self, indent: i32) -> Rc<Doc>;
    fn nest_if_break(self, indent: i32, observed_doc: usize) -> Rc<Doc>;
    fn nest_hanging(self) -> Rc<Doc>;
}

impl DocAlgebra for Rc<Doc> {
    fn cons(self, other: Rc<Doc>) -> Rc<Doc> {
        let properties = CommonProperties(
            query_inline_position(&self) + query_inline_position(&other),
            0,
        );
        Rc::new(Doc::Cons(self, other, properties))
    }

    fn to_group(self, should_break: ShouldBreak, doc_ref: &mut usize) -> Rc<Doc> {
        *doc_ref += 1;
        let properties = CommonProperties(
            match query_inline_position(&self) {
                InlineCommentPosition::Middle => InlineCommentPosition::InGroup,
                InlineCommentPosition::InGroup => InlineCommentPosition::No,
                position => position,
            },
            *doc_ref,
        );
        Rc::new(Doc::Group(
            GroupDocProperties(self, should_break),
            properties,
        ))
    }

    fn nest(self, indent: i32) -> Rc<Doc> {
        let properties = CommonProperties(query_inline_position(&self), 0);
        Rc::new(Doc::Nest(indent, self, properties))
    }

    fn nest_if_break(self, indent: i32, observed_doc: usize) -> Rc<Doc> {
        let properties = CommonProperties(query_inline_position(&self), 0);
        Rc::new(Doc::NestIfBreak(indent, self, properties, observed_doc))
    }

    fn nest_hanging(self) -> Rc<Doc> {
        let properties = CommonProperties(query_inline_position(&self), 0);
        Rc::new(Doc::NestHanging(self, properties))
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

fn fits(mut remaining_width: i32, docs: &mut VecDeque<Triple>) -> bool {
    trace!("Judging fits for {docs:?}");
    while remaining_width >= 0 {
        match docs.pop_front() {
            None => {
                trace!("Fits returned true");
                return true;
            }
            Some((indent, mode, doc)) => match (indent, mode, &*doc) {
                (_, _, Doc::Nil) => continue,
                (i, m, Doc::Cons(first, second, _)) => {
                    docs.push_front((i, m, Rc::clone(second)));
                    docs.push_front((i, m, Rc::clone(first)));
                    continue;
                }
                (i, m, Doc::Nest(step, doc, _)) => {
                    docs.push_front((i + step, m, Rc::clone(doc)));
                    continue;
                }
                (i, m, Doc::NestIfBreak(step, doc, _, _)) => {
                    docs.push_front((i + step, m, Rc::clone(doc)));
                    continue;
                }
                (i, m, Doc::NestHanging(doc, _)) => {
                    docs.push_front((i, m, Rc::clone(doc)));
                    continue;
                }
                // Special case for the embracing operator
                (_, _, Doc::Text(text, s_len, _)) if &**text == "{" => {
                    if docs.front().is_some() {
                        let (_, _, inner_doc) = docs.front().unwrap();
                        match &**inner_doc {
                            Doc::Text(inner_text, _, _) => {
                                if &**inner_text == "{" {
                                    trace!("Found embracing operator while trying to fit the line");
                                    docs.pop_front();
                                    remaining_width -= 2 * *s_len as i32;
                                    continue;
                                } else {
                                    trace!(
                                        "Found a non-embracing left brace. Returning true from fit"
                                    );
                                    return true;
                                }
                            }
                            _ => {
                                trace!("Fits returned true");
                                return true;
                            }
                        }
                    } else {
                        remaining_width -= *s_len as i32;
                        continue;
                    }
                }
                (_, _, Doc::Text(_, s_len, _)) => {
                    remaining_width -= *s_len as i32;
                    continue;
                }
                (_, Mode::Flat, Doc::Break(s)) => {
                    remaining_width -= s.len() as i32;
                    continue;
                }
                (_, Mode::Break, Doc::Break(_)) => unreachable!(),
                (i, _, Doc::Group(groupped_doc, CommonProperties(inline_comment_pos, _))) => {
                    if inline_comment_pos == &InlineCommentPosition::Middle {
                        trace!("Fits returned false due to inline comment {inline_comment_pos:?}");
                        return false;
                    } else {
                        docs.push_front((i, Mode::Flat, Rc::clone(&groupped_doc.0)));
                        continue;
                    }
                }
            },
        }
    }
    trace!("Fits returned false");
    false
}

/// `broken_docs` is a set of all the docs that are being formatted
/// with line breaks. This set is continuously being filled up during
/// execution of `format_to_sdoc`.
pub(crate) fn format_to_sdoc(
    consumed: i32,
    docs: &mut VecDeque<Triple>,
    config: &impl FormattingConfig,
    broken_docs: &mut HashSet<usize>,
) -> SimpleDoc {
    let line_length = config.line_length();
    match docs.pop_front() {
        None => SimpleDoc::Nil,
        Some(doc) => {
            let (indent, mode, doc) = doc;
            match (indent, mode, &*doc) {
                (_, _, Doc::Nil) => format_to_sdoc(consumed, docs, config, broken_docs),
                (i, m, Doc::Cons(first, second, _)) => {
                    docs.push_front((i, m, Rc::clone(second)));
                    docs.push_front((i, m, Rc::clone(first)));
                    format_to_sdoc(consumed, docs, config, broken_docs)
                }
                (i, m, Doc::Nest(step, doc, _)) => {
                    docs.push_front((i + step, m, Rc::clone(doc)));
                    format_to_sdoc(consumed, docs, config, broken_docs)
                }
                (i, m, Doc::NestIfBreak(step, doc, _, observed_doc)) => {
                    if broken_docs.contains(observed_doc) {
                        docs.push_front((i + step, m, Rc::clone(doc)));
                    } else {
                        docs.push_front((i, m, Rc::clone(doc)));
                    }
                    format_to_sdoc(consumed, docs, config, broken_docs)
                }
                (i, m, Doc::NestHanging(doc, props)) => {
                    docs.push_front((
                        i,
                        m,
                        Rc::new(Doc::Nest(consumed - i, Rc::clone(doc), *props)),
                    ));
                    format_to_sdoc(consumed, docs, config, broken_docs)
                }
                (_, _, Doc::Text(s, width, _)) => {
                    let length = *width as i32;
                    SimpleDoc::Text(
                        Rc::clone(s),
                        Rc::new(format_to_sdoc(consumed + length, docs, config, broken_docs)),
                    )
                }
                (_, Mode::Flat, Doc::Break(s)) => {
                    let length = s.len() as i32;
                    SimpleDoc::Text(
                        Rc::from(*s),
                        Rc::new(format_to_sdoc(consumed + length, docs, config, broken_docs)),
                    )
                }
                (i, Mode::Break, Doc::Break(_)) => SimpleDoc::Line(
                    i as usize,
                    Rc::new(format_to_sdoc(i, docs, config, broken_docs)),
                ),
                (i, _, Doc::Group(groupped_doc, CommonProperties(inline_comment_pos, doc_ref))) => {
                    let mut group_docs =
                        VecDeque::from([(i, Mode::Flat, Rc::clone(&groupped_doc.0))]);
                    if groupped_doc.1 == ShouldBreak::Yes
                        || matches!(inline_comment_pos, InlineCommentPosition::Middle)
                        || !fits(line_length - consumed, &mut group_docs)
                    {
                        docs.push_front((i, Mode::Break, Rc::clone(&groupped_doc.0)));
                        broken_docs.insert(*doc_ref);
                        format_to_sdoc(consumed, docs, config, broken_docs)
                    } else {
                        docs.push_front((i, Mode::Flat, Rc::clone(&groupped_doc.0)));
                        format_to_sdoc(consumed, docs, config, broken_docs)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::FunctionLineBreaks;

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

        fn space_before_complex_rhs_in_formulas(&self) -> bool {
            true
        }

        fn strip_suffix_whitespace_in_function_defs(&self) -> bool {
            true
        }

        fn function_line_breaks(&self) -> FunctionLineBreaks {
            FunctionLineBreaks::Hanging
        }

        fn insert_newline_in_quote_call(&self) -> bool {
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
        let mut s = HashSet::default();
        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config, &mut s));

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
        let mut s = HashSet::default();
        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config, &mut s));

        assert_eq!(simple_doc_to_string(sdoc), "Test\nTest2")
    }
}