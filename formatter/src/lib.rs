mod code;
pub mod config;
mod format;

use crate::code::Code;
use crate::format::format_to_sdoc;
use crate::format::simple_doc_to_string;
use crate::format::Mode;
use log::trace;
use parser::ast::Expression;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn format_code(
    expression: &[Expression],
    formatting_config: &impl config::FormattingConfig,
) -> String {
    let mut docs: VecDeque<_> = expression
        .iter()
        .map(|expr| (0, Mode::Flat, expr.to_docs(formatting_config)))
        .collect();
    trace!("Docs: {:?}", docs);
    let simple_doc = Rc::new(format_to_sdoc(0, &mut docs, formatting_config));
    trace!("Simple docs: {:?}", simple_doc);
    let mut ans = simple_doc_to_string(simple_doc);
    // Add a newline at the end of the file
    ans.push('\n');
    ans
}
