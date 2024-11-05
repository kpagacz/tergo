use serde::Deserialize;

pub trait FormattingConfig: std::fmt::Display + Clone {
    fn line_length(&self) -> i32;
    fn indent(&self) -> i32;
    // Custom embracing behaviour: https://style.tidyverse.org/syntax.html#embracing
    fn embracing_op_no_nl(&self) -> bool;
    fn allow_nl_after_assignment(&self) -> bool;
    fn space_before_complex_rhs_in_formulas(&self) -> bool;
    fn strip_suffix_whitespace_in_function_defs(&self) -> bool;
    fn function_line_breaks(&self) -> FunctionLineBreaks;
    fn insert_newline_in_quote_call(&self) -> bool;
}

#[derive(Debug, Clone, Copy, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FunctionLineBreaks {
    #[default]
    Hanging,
    Double,
    Single,
}

#[derive(Debug, Clone, Copy, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub indent: Indent,
    #[serde(default)]
    pub line_length: LineLength,
    /// Embracing operator {{ }} does not have line breaks
    #[serde(default)]
    pub embracing_op_no_nl: EmbracingOpNoNl,
    /// To allow new lines after assignment or not
    /// Ex. a <-
    ///       TRUE
    /// or
    /// a <- TRUE
    #[serde(default)]
    pub allow_nl_after_assignment: AllowNlAfterAssignment,
    /// Whether to put a space before complex right hand sides of
    /// the formula operator. Example:
    /// ~ a + b, but ~a
    #[serde(default)]
    pub space_before_complex_rhs_in_formula: SpaceBeforeComplexRhsInFormulas,
    /// Whether to keep the whitespace before the ending
    /// bracket of a function definition in cases such as this:
    /// function() {
    ///   TRUE
    ///
    /// }
    #[serde(default)]
    pub strip_suffix_whitespace_in_function_defs: StripSuffixWhitespaceInFunctionDefs,
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
    #[serde(default)]
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
    #[serde(default)]
    pub insert_newline_in_quote_call: InsertNewlineInQuoteCall,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Indent(pub i32);
impl Default for Indent {
    fn default() -> Self {
        Self(2)
    }
}
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct LineLength(pub i32);
impl Default for LineLength {
    fn default() -> Self {
        Self(120)
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct EmbracingOpNoNl(pub bool);
impl Default for EmbracingOpNoNl {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
pub struct AllowNlAfterAssignment(pub bool);

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct SpaceBeforeComplexRhsInFormulas(pub bool);
impl Default for SpaceBeforeComplexRhsInFormulas {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct StripSuffixWhitespaceInFunctionDefs(pub bool);
impl Default for StripSuffixWhitespaceInFunctionDefs {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct InsertNewlineInQuoteCall(pub bool);
impl Default for InsertNewlineInQuoteCall {
    fn default() -> Self {
        Self(true)
    }
}

impl FormattingConfig for Config {
    fn line_length(&self) -> i32 {
        self.line_length.0
    }

    fn indent(&self) -> i32 {
        self.indent.0
    }

    fn embracing_op_no_nl(&self) -> bool {
        self.embracing_op_no_nl.0
    }

    fn allow_nl_after_assignment(&self) -> bool {
        self.allow_nl_after_assignment.0
    }

    fn space_before_complex_rhs_in_formulas(&self) -> bool {
        self.space_before_complex_rhs_in_formula.0
    }

    fn strip_suffix_whitespace_in_function_defs(&self) -> bool {
        self.strip_suffix_whitespace_in_function_defs.0
    }

    fn function_line_breaks(&self) -> FunctionLineBreaks {
        self.function_line_breaks
    }

    fn insert_newline_in_quote_call(&self) -> bool {
        self.insert_newline_in_quote_call.0
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "indent: {} line_length: {} allow_nl_after_assignment: {}",
            self.indent.0, self.line_length.0, self.allow_nl_after_assignment.0
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
            indent: Indent(indent),
            line_length: LineLength(line_length),
            embracing_op_no_nl: EmbracingOpNoNl(embracing_op_no_nl),
            allow_nl_after_assignment: AllowNlAfterAssignment(allow_nl_after_assignment),
            space_before_complex_rhs_in_formula: SpaceBeforeComplexRhsInFormulas(
                space_before_complex_rhs_in_formula,
            ),
            strip_suffix_whitespace_in_function_defs: StripSuffixWhitespaceInFunctionDefs(
                strip_suffix_whitespace_in_function_defs,
            ),
            function_line_breaks,
            insert_newline_in_quote_call: InsertNewlineInQuoteCall(insert_newline_in_quote_call),
        }
    }
}
