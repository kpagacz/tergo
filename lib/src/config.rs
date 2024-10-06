use formatter::config::FormattingConfig;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Config {
    pub indent: i32,
    pub line_length: i32,
    pub embracing_op_no_nl: bool,
    pub allow_nl_after_assignment: bool,
    pub space_before_complex_rhs_in_formula: bool,
    pub strip_suffix_whitespace_in_function_defs: bool,
}

impl FormattingConfig for Config {
    fn line_length(&self) -> i32 {
        self.line_length
    }

    fn indent(&self) -> i32 {
        self.indent
    }

    fn embracing_op_no_nl(&self) -> bool {
        self.embracing_op_no_nl
    }

    fn allow_nl_after_assignment(&self) -> bool {
        self.allow_nl_after_assignment
    }

    fn space_before_complex_rhs_in_formulas(&self) -> bool {
        self.space_before_complex_rhs_in_formula
    }

    fn strip_suffix_whitespace_in_function_defs(&self) -> bool {
        self.strip_suffix_whitespace_in_function_defs
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indent: 2,
            line_length: 120,
            embracing_op_no_nl: true,
            allow_nl_after_assignment: false,
            space_before_complex_rhs_in_formula: true,
            strip_suffix_whitespace_in_function_defs: true,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "indent: {} line_length: {} allow_nl_after_assignment: {}",
            self.indent, self.line_length, self.allow_nl_after_assignment
        ))
    }
}

impl Config {
    pub fn new(
        indent: i32,
        line_length: i32,
        embracing_op_no_nl: bool,
        allow_nl_after_assignment: bool,
        space_before_complex_rhs_in_formula: bool,
        strip_suffix_whitespace_in_function_defs: bool,
    ) -> Self {
        Self {
            indent,
            line_length,
            embracing_op_no_nl,
            allow_nl_after_assignment,
            space_before_complex_rhs_in_formula,
            strip_suffix_whitespace_in_function_defs,
        }
    }
}
