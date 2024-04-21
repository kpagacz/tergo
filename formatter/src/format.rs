// Implementing Wadler and https://lindig.github.io/papers/strictly-pretty-2000.pdf
use std::collections::VecDeque;
use std::rc::Rc;

use crate::config::FormattingConfig;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ShouldBreak {
    Yes,
    No,
}

/// ShouldBreak is a linebreak that propagates to the parents
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GroupDocProperties(pub(crate) Rc<Doc>, pub(crate) ShouldBreak); // (doc, should it break?)

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Doc {
    Nil,
    Cons(Rc<Doc>, Rc<Doc>),
    Text(Rc<str>),
    Nest(i32, Rc<Doc>),
    Break(&'static str),
    Group(GroupDocProperties),
}

impl std::fmt::Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Doc::Nil => f.write_str("Nil"),
            Doc::Cons(left, right) => f.write_fmt(format_args!("{} + {}", left, right)),
            Doc::Text(text) => f.write_fmt(format_args!("'{}'", text)),
            Doc::Nest(indent, body) => f.write_fmt(format_args!("Nest{}({})", indent, body)),
            Doc::Break(newline) => f.write_fmt(format_args!("NL({})", newline)),
            Doc::Group(inside) => f.write_fmt(format_args!("SB:{:?}<{}>", inside.1, inside.0)),
        }
    }
}

pub trait DocAlgebra {
    fn cons(self, other: Rc<Doc>) -> Rc<Doc>;
    fn to_group(self, should_break: ShouldBreak) -> Rc<Doc>;
    fn nest(self, indent: i32) -> Rc<Doc>;
}

impl DocAlgebra for Rc<Doc> {
    fn cons(self, other: Rc<Doc>) -> Rc<Doc> {
        Rc::new(Doc::Cons(self, other))
    }

    fn to_group(self, should_break: ShouldBreak) -> Rc<Doc> {
        Rc::new(Doc::Group(GroupDocProperties(self, should_break)))
    }

    fn nest(self, indent: i32) -> Rc<Doc> {
        Rc::new(Doc::Nest(indent, self))
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
                (i, m, Doc::Cons(first, second)) => {
                    docs.push_front((i, m, Rc::clone(second)));
                    docs.push_front((i, m, Rc::clone(first)));
                    fits(remaining_width, docs)
                }
                (i, m, Doc::Nest(step, doc)) => {
                    docs.push_front((i + step, m, Rc::clone(doc)));
                    fits(remaining_width, docs)
                }
                (_, _, Doc::Text(s)) => fits(remaining_width - s.len() as i32, docs),
                (_, Mode::Flat, Doc::Break(s)) => fits(remaining_width - s.len() as i32, docs),
                (_, Mode::Break, Doc::Break(_)) => unreachable!(),
                (i, _, Doc::Group(groupped_doc)) => {
                    if groupped_doc.1 == ShouldBreak::Yes {
                        return false;
                    }
                    docs.push_front((i, Mode::Flat, Rc::clone(&groupped_doc.0)));
                    fits(remaining_width, docs)
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
                (i, m, Doc::Cons(first, second)) => {
                    docs.push_front((i, m, Rc::clone(second)));
                    docs.push_front((i, m, Rc::clone(first)));
                    format_to_sdoc(consumed, docs, config)
                }
                (i, m, Doc::Nest(step, doc)) => {
                    docs.push_front((i + step, m, Rc::clone(doc)));
                    format_to_sdoc(consumed, docs, config)
                }
                (_, _, Doc::Text(s)) => {
                    let length = s.len() as i32;
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
                (i, _, Doc::Group(groupped_doc)) => {
                    let mut group_docs =
                        VecDeque::from([(i, Mode::Flat, Rc::clone(&groupped_doc.0))]);
                    if groupped_doc.1 == ShouldBreak::Yes
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
    }
    impl std::fmt::Display for MockConfig {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }

    #[test]
    fn printing_text_doc() {
        log_init();
        let mut doc = VecDeque::from([(0i32, Mode::Flat, Rc::new(Doc::Text(Rc::from("Test"))))]);
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
            Rc::new(Doc::Group(GroupDocProperties(
                Rc::new(Doc::Cons(
                    Rc::new(Doc::Text(Rc::from("Test"))),
                    Rc::new(Doc::Cons(
                        Rc::new(Doc::Break(" ")),
                        Rc::new(Doc::Text(Rc::from("Test2"))),
                    )),
                )),
                ShouldBreak::Yes,
            ))),
        )]);
        let mock_config = MockConfig {};
        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config));

        assert_eq!(simple_doc_to_string(sdoc), "Test\nTest2")
    }

    #[test]
    fn should_break_propagates_to_parents() {
        log_init();
        let mut doc = VecDeque::from([(
            0i32,
            Mode::Flat,
            Rc::new(Doc::Group(GroupDocProperties(
                Rc::new(Doc::Cons(
                    Rc::new(Doc::Text(Rc::from("Test"))),
                    Rc::new(Doc::Cons(
                        Rc::new(Doc::Break(" ")),
                        Rc::new(Doc::Group(GroupDocProperties(
                            Rc::new(Doc::Text(Rc::from("Test2"))),
                            ShouldBreak::Yes,
                        ))),
                    )),
                )),
                ShouldBreak::No,
            ))),
        )]);
        let mock_config = MockConfig {};
        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config));

        assert_eq!(simple_doc_to_string(sdoc), "Test\nTest2")
    }
}
