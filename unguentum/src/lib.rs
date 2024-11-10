mod code;
pub mod config;
mod format;
pub(crate) mod post_format_hooks;
pub(crate) mod pre_format_hooks;

use crate::code::Code;
use crate::format::format_to_sdoc;
use crate::format::simple_doc_to_string;
use crate::format::DocBuffer;
use crate::format::Mode;
use log::trace;
use parser::ast::Expression;
use post_format_hooks::trim_line_endings;
use post_format_hooks::trim_trailing_line;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn format_code<T: config::FormattingConfig>(
    mut expression: Expression,
    formatting_config: &T,
) -> String {
    // Pre formatting hooks
    let mut pre_format: Vec<fn(&mut Expression<'_>)> = vec![];
    if formatting_config.strip_suffix_whitespace_in_function_defs() {
        pre_format.push(pre_format_hooks::remove_trailing_whitespace_from_function_defs);
    }

    for hook in pre_format {
        hook(&mut expression);
    }

    // Doc stage
    let mut doc_ref = 0usize;
    let mut docs: VecDeque<_> = VecDeque::from([(
        0i32,
        Mode::Flat,
        expression.to_docs(formatting_config, &mut doc_ref),
    )]);
    trace!("Config: {}", formatting_config);
    trace!("Docs: {}", DocBuffer(&docs));

    // Simple docs stage
    use std::collections::HashSet;
    let mut broken_docs = HashSet::default();
    let simple_doc = Rc::new(format_to_sdoc(
        0,
        &mut docs,
        formatting_config,
        &mut broken_docs,
    ));
    trace!("Simple docs: {:?}", simple_doc);

    // Printing to string
    let mut formatted = simple_doc_to_string(simple_doc);

    // Post-format hooks
    let post_format_hooks = vec![trim_line_endings, trim_trailing_line];
    for hook in post_format_hooks {
        formatted = hook(formatted);
    }

    formatted
}
