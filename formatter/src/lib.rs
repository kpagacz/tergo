mod code;
pub mod config;
mod format;

use crate::code::Code;
use crate::format::format_to_sdoc;
use crate::format::simple_doc_to_string;
use crate::format::DocBuffer;
use crate::format::Mode;
use log::trace;
use parser::ast::Expression;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn format_code<T: config::FormattingConfig>(
    expression: Expression,
    formatting_config: &T,
) -> String {
    let mut docs: VecDeque<_> =
        VecDeque::from([(0i32, Mode::Flat, expression.to_docs(formatting_config))]);
    trace!("Config: {}", formatting_config);
    trace!("Docs: {}", DocBuffer(&docs));
    let simple_doc = Rc::new(format_to_sdoc(0, &mut docs, formatting_config));
    trace!("Simple docs: {:?}", simple_doc);
    simple_doc_to_string(simple_doc)
}
