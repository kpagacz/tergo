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

#[derive(Debug, Clone, Copy, Deserialize, Default, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionLineBreaks {
    #[default]
    Hanging,
    Double,
    Single,
}

/// The configuration for `tergo`.
///
/// This configuration can also read from a TOML file.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    /// The number of characters to use for one level of indentation.
    ///
    /// Default: 2.
    #[serde(default)]
    pub indent: Indent,

    /// Tha maximum number of characters in a line of the formatted
    /// code. `tergo` will ensure lines do not exceed this number
    /// if possible.
    ///
    /// Default: 120.
    #[serde(default)]
    pub line_length: LineLength,

    /// A logical flag to determine whether to suppress line
    /// breaks for embracing operator `{{}}`.
    ///
    /// If true, the formatter outputs the following code:
    ///
    /// ```R
    ///  data |>
    ///    group_by({{ by }})
    /// ````
    ///
    /// instead of inserting a new line after each `{`.
    ///
    /// Default: true.
    #[serde(default)]
    pub embracing_op_no_nl: EmbracingOpNoNl,

    /// A logical flag indicating whether to insert new lines after
    /// the assignment operator `<-` in cases where the code
    /// does not fit a single line (so a line break is needed somewhere).
    ///
    /// The formatter outputs the following:
    ///
    /// ```R
    /// a <- TRUE # for allow_nl_after_assignment = false
    /// # or
    /// a <-
    ///   TRUE # for allow_nl_after_assignment = true
    /// ```
    ///
    /// in cases where the code does not fit in a single line.
    ///
    /// Default: false.
    #[serde(default)]
    pub allow_nl_after_assignment: AllowNlAfterAssignment,

    /// A logical flag indicating whether to put a space before complex right hand sides of
    /// the formula operator. Example:
    ///
    /// ```R
    /// # If space_before_complex_rhs_in_formula = true
    /// ~ a + b
    /// ~a
    ///
    /// # If space_before_complex_rhs_in_formula = false
    /// ~a + b
    /// ~a
    /// ```
    ///
    /// Default: true.
    #[serde(default)]
    pub space_before_complex_rhs_in_formula: SpaceBeforeComplexRhsInFormulas,

    /// A logical flag indicating whether to keep the whitespace before the ending
    /// bracket of a function definition in cases such as this:
    ///
    /// ```R
    /// function() {
    ///   TRUE
    ///
    /// }
    /// ```
    ///
    /// If true, `tergo` will remove the whitespace:
    ///
    /// ```R
    /// function() {
    ///   TRUE
    /// }
    /// ```
    ///
    /// Default: true.
    #[serde(default)]
    pub strip_suffix_whitespace_in_function_defs: StripSuffixWhitespaceInFunctionDefs,

    /// The type of line breaking inside function definitions'
    /// arguments. Possible values are: `single`, `double`, `hanging`.
    /// Single puts a single level of indent for the arguments, double
    /// puts a double level of indent and hanging puts the arguments
    /// in the same column as the first argument to the function.
    ///
    /// Examples:
    ///
    /// ```R
    /// # Single:
    /// function(
    ///   a
    /// ) {}
    ///
    /// # Double:
    /// function(
    ///     a
    /// ) {}
    ///
    /// # Hanging:
    /// function(a,
    ///          b) {}
    /// ```
    ///
    /// Default: `hanging`.
    #[serde(default)]
    pub function_line_breaks: FunctionLineBreaks,

    /// A logical flag indicating whether to insert a new line after
    /// the opening parenthesis of a call to quote for very long calls.
    ///
    /// Examples:
    ///
    /// ```R
    /// # If insert_newline_in_quote_call = false
    /// quote(a <- function(call) {
    ///   TRUE
    ///   TRUE
    /// })
    ///
    /// # vs
    ///
    /// # If insert_newline_in_quote_call = true
    /// quote(
    ///   a <- function(call) {
    ///     TRUE
    ///     TRUE
    ///   }
    /// )
    /// ```
    ///
    /// Default: true.
    #[serde(default)]
    pub insert_newline_in_quote_call: InsertNewlineInQuoteCall,

    /// A list of file paths to exclude from formatting.
    ///
    /// The file paths are relative to the directory
    /// in which `tergo` is run.
    ///
    /// Example values:
    ///
    /// exclusion_list = ["./balnea",
    /// "./aqua",
    /// "./scopa",
    /// "./spongia",
    /// "./tergo",
    /// "./unguentum",
    /// "./antidotum/tergo/R/extendr-wrappers.R",
    /// "./target"]
    #[serde(default)]
    pub exclusion_list: ExclusionList,
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

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ExclusionList(pub Vec<String>);

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
        exclusion_list: Vec<String>,
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
            exclusion_list: ExclusionList(exclusion_list),
        }
    }
}
