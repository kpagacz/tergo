// Implementing Wadler and https://lindig.github.io/papers/strictly-pretty-2000.pdf
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Doc {
    Nil,
    Cons(Rc<Doc>, Rc<Doc>),
    Text(Rc<str>),
    Nest(i32, Rc<Doc>),
    Break(&'static str),
    Group(Vec<Triple>),
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

fn fits(current_width: i32, docs: &mut Vec<Triple>) -> bool {
    if current_width < 0 {
        false
    } else {
        match docs.pop() {
            None => true,
            Some((indent, mode, doc)) => match (indent, mode, &*doc) {
                (_, _, Doc::Nil) => fits(current_width, docs),
                (i, m, Doc::Cons(first, second)) => {
                    docs.push((i, m, Rc::clone(second)));
                    docs.push((i, m, Rc::clone(first)));
                    fits(current_width, docs)
                }
                (i, m, Doc::Nest(step, doc)) => {
                    docs.push((i + step, m, Rc::clone(doc)));
                    fits(current_width, docs)
                }
                (_, _, Doc::Text(s)) => fits(current_width - s.len() as i32, docs),
                (_, Mode::Flat, Doc::Break(s)) => fits(current_width - s.len() as i32, docs),
                (_, Mode::Break, Doc::Break(_)) => unreachable!(),
                (_, _, Doc::Group(groupped_docs)) => {
                    groupped_docs
                        .into_iter()
                        .for_each(|doc| docs.push(doc.clone()));
                    fits(current_width, docs)
                }
            },
        }
    }
}

// TODO: fix this so this is not a dumb clone, even if it just clones pointers
const LINE_LENGTH: i32 = 120;
pub(crate) fn format_to_sdoc(consumed: i32, docs: &mut Vec<Triple>) -> SimpleDoc {
    match docs.pop() {
        None => SimpleDoc::Nil,
        Some(doc) => {
            let (indent, mode, doc) = doc;
            match (indent, mode, &*doc) {
                (_, _, Doc::Nil) => format_to_sdoc(consumed, docs),
                (i, m, Doc::Cons(first, second)) => {
                    docs.push((i, m, Rc::clone(second)));
                    docs.push((i, m, Rc::clone(first)));
                    format_to_sdoc(consumed, docs)
                }
                (i, m, Doc::Nest(step, doc)) => {
                    docs.push((i + step, m, Rc::clone(doc)));
                    format_to_sdoc(consumed, docs)
                }
                (_, _, Doc::Text(s)) => {
                    let length = s.len() as i32;
                    SimpleDoc::Text(
                        Rc::clone(s),
                        Rc::new(format_to_sdoc(consumed + length, docs)),
                    )
                }
                (_, Mode::Flat, Doc::Break(s)) => {
                    let length = s.len() as i32;
                    SimpleDoc::Text(
                        Rc::from(*s),
                        Rc::new(format_to_sdoc(consumed + length, docs)),
                    )
                }
                (_, Mode::Break, Doc::Break(_)) => {
                    SimpleDoc::Line(indent as usize, Rc::new(format_to_sdoc(indent, docs)))
                }
                (_, _, Doc::Group(groupped_docs)) => {
                    let mut docs_clone = docs.clone();
                    let mut groupped_clone = groupped_docs.clone();
                    docs_clone.append(&mut groupped_clone);
                    if fits(LINE_LENGTH - consumed, &mut docs_clone) {
                        groupped_docs
                            .into_iter()
                            .map(|(i, _, doc)| (*i, Mode::Flat, Rc::clone(doc)))
                            .for_each(|doc| docs.push(doc));
                        format_to_sdoc(consumed, docs)
                    } else {
                        groupped_docs
                            .into_iter()
                            .map(|(i, _, doc)| (*i, Mode::Break, Rc::clone(doc)))
                            .for_each(|doc| docs.push(doc));
                        format_to_sdoc(consumed, docs)
                    }
                }
            }
        }
    }
}
