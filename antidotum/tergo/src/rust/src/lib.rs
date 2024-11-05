use extendr_api::prelude::*;
use tergo_lib::Config;
use tergo_lib::FunctionLineBreaks;

/// Format code
///
/// @param source_code (`character`) the R code to format
///
/// @return (`character`) the formatted code
/// @keywords internal
#[extendr]
fn format_code(
    source_code: &str,
    indent: i32,
    line_length: i32,
    embracing_op_no_nl: bool,
    allow_nl_after_assignment: bool,
    space_before_complex_rhs_in_formula: bool,
    strip_suffix_whitespace_in_function_defs: bool,
    function_line_breaks: &str,
    insert_newline_in_quote_call: bool,
) -> String {
    let function_line_breaks = if function_line_breaks == "single" {
        FunctionLineBreaks::Single
    } else if function_line_breaks == "double" {
        FunctionLineBreaks::Double
    } else {
        FunctionLineBreaks::Hanging
    };
    let config = Config::new(
        indent,
        line_length,
        embracing_op_no_nl,
        allow_nl_after_assignment,
        space_before_complex_rhs_in_formula,
        strip_suffix_whitespace_in_function_defs,
        function_line_breaks,
        insert_newline_in_quote_call,
    );
    tergo_lib::tergo_format(source_code, Some(&config)).unwrap()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod tergo;
    fn format_code;
}
