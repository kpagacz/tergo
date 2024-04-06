// Implementing Wadler and https://lindig.github.io/papers/strictly-pretty-2000.pdf
use std::collections::VecDeque;
use std::rc::Rc;

use log::trace;

use crate::config::FormattingConfig;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Doc {
    Nil,
    Cons(Rc<Doc>, Rc<Doc>),
    Text(Rc<str>),
    Nest(i32, Rc<Doc>),
    Break(&'static str),
    Group(Rc<Doc>),
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
                    docs.push_front((i, Mode::Flat, Rc::clone(groupped_doc)));
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
                    trace!("Formatting a break to s");
                    let length = s.len() as i32;
                    SimpleDoc::Text(
                        Rc::from(*s),
                        Rc::new(format_to_sdoc(consumed + length, docs, config)),
                    )
                }
                (i, Mode::Break, Doc::Break(_)) => {
                    trace!("Formatting a break to a new line");
                    SimpleDoc::Line(i as usize, Rc::new(format_to_sdoc(i, docs, config)))
                }
                (i, _, Doc::Group(groupped_doc)) => {
                    trace!(
                        "Formatting a group: {groupped_doc:?} with i: {i} and consumed: {consumed}"
                    );
                    let mut cloned_docs = docs.clone();
                    cloned_docs.push_front((i, Mode::Flat, Rc::clone(groupped_doc)));
                    if fits(line_length - consumed, &mut cloned_docs) {
                        trace!("The group fits");
                        docs.pop_front();
                        docs.push_front((i, Mode::Flat, Rc::clone(groupped_doc)));
                        format_to_sdoc(consumed, docs, config)
                    } else {
                        trace!("The group does not fit");
                        docs.pop_front();
                        docs.push_front((i, Mode::Break, Rc::clone(groupped_doc)));
                        format_to_sdoc(consumed, docs, config)
                    }
                }
            }
        }
    }
}
