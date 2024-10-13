use formatter::config::{FormattingConfig, FunctionLineBreaks};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Config {
    pub indent: i32,
    pub line_length: i32,
    /// Embracing operator {{ }} does not have line breaks
    pub embracing_op_no_nl: bool,
    /// To allow new lines after assignment or not
    /// Ex. a <-
    ///       TRUE
    /// or
    /// a <- TRUE
    pub allow_nl_after_assignment: bool,
    /// Whether to put a space before complex right hand sides of
    /// the formula operator. Example:
    /// ~ a + b, but ~a
    pub space_before_complex_rhs_in_formula: bool,
    /// Whether to keep the whitespace before the ending
    /// bracket of a function definition
    pub strip_suffix_whitespace_in_function_defs: bool,
    /// The type of line breaking inside function definitions'
    /// arguments. Example:
    /// Single:
    /// function(
    ///   a
    /// ) {}
    /// Double:
    /// function(
    ///     a
    /// ) {}
    /// Hanging:
    /// function(a,
    ///          b) {}
    pub function_line_breaks: FunctionLineBreaks,
    /// Whether to insert the new line after
    /// the opening parenthesis of a call to quote
    /// for very long calls. Example:
    /// quote(a <- function(call) {
    ///   TRUE
    ///   TRUE
    /// })
    /// vs
    /// quote(
    ///   a <- function(call) {
    ///     TRUE
    ///     TRUE
    ///   }
    /// )
    pub insert_newline_in_quote_call: bool,
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

    fn function_line_breaks(&self) -> FunctionLineBreaks {
        self.function_line_breaks
    }

    fn insert_newline_in_quote_call(&self) -> bool {
        self.insert_newline_in_quote_call
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
            function_line_breaks: FunctionLineBreaks::Hanging,
            insert_newline_in_quote_call: true,
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

#[allow(clippy::too_many_arguments)]
impl Config {
    pub fn new(
        indent: i32,
        line_length: i32,
        embracing_op_no_nl: bool,
        allow_nl_after_assignment: bool,
        space_before_complex_rhs_in_formula: bool,
        strip_suffix_whitespace_in_function_defs: bool,
        function_line_breaks: FunctionLineBreaks,
        insert_newline_in_quote_call: bool,
    ) -> Self {
        Self {
            indent,
            line_length,
            embracing_op_no_nl,
            allow_nl_after_assignment,
            space_before_complex_rhs_in_formula,
            strip_suffix_whitespace_in_function_defs,
            function_line_breaks,
            insert_newline_in_quote_call,
        }
    }
}
