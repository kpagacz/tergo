mod code;
mod format;

use crate::code::Code;
use crate::format::format_to_sdoc;
use crate::format::simple_doc_to_string;
use parser::ast::Expression;
use std::rc::Rc;

pub fn format_code(expression: &Expression) -> String {
    let mut docs = vec![expression.to_docs()];
    eprintln!("docs:\n{:?}", docs);
    let simple_doc = Rc::new(format_to_sdoc(0, &mut docs));
    eprintln!("sdocs:\n{:?}", simple_doc);
    simple_doc_to_string(simple_doc)
}
