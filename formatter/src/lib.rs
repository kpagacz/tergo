mod code;
mod format;

use crate::code::Code;
use crate::format::format_to_sdoc;
use crate::format::simple_doc_to_string;
use parser::ast::Expression;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn format_code(expression: &[Expression]) -> String {
    let mut docs: VecDeque<_> = expression.iter().map(|expr| expr.to_docs()).collect();
    eprintln!("docs:\n{:?}", docs);
    let simple_doc = Rc::new(format_to_sdoc(0, &mut docs));
    eprintln!("sdocs:\n{:?}", simple_doc);
    let mut ans = simple_doc_to_string(simple_doc);
    // Add a newline at the end of the file
    ans.push('\n');
    ans
}
