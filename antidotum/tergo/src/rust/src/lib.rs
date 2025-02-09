use extendr_api::prelude::*;
use std::collections::HashMap;
use tergo_lib::{Config, FunctionLineBreaks};

const ERROR: &str = "error";
const OK: &str = "success";

fn config_to_bool(
    field: &str,
    configuration: &HashMap<&str, Robj>,
    default_value: bool,
) -> std::result::Result<bool, extendr_api::List> {
    let config_value = configuration.get(field);
    let value: bool;
    if let Some(config) = config_value {
        if let Some(casted) = config.as_bool() {
            value = casted;
        } else {
            return Err(list!(
                ERROR,
                format!("{} configuration value must be a boolean.", field)
            ));
        }
    } else {
        value = default_value;
    }
    Ok(value)
}

fn config_to_integer(
    field: &str,
    configuration: &HashMap<&str, Robj>,
    default_value: i32,
) -> std::result::Result<i32, extendr_api::List> {
    let config_value = configuration.get(field);
    let value: i32;
    if let Some(config) = config_value {
        if let Some(casted) = config.as_integer() {
            value = casted;
        } else {
            return Err(list!(
                ERROR,
                format!(
                    "{} configuration value must be an integer. Did you forget about L?",
                    field
                )
            ));
        }
    } else {
        value = default_value;
    }
    Ok(value)
}

/// Format code
///
/// @param source_code (`character`) the R code to format
///
/// @return (`character`) the formatted code
/// @keywords internal
#[extendr]
#[allow(clippy::too_many_arguments)]
fn format_code(source_code: &str, configuration: extendr_api::List) -> extendr_api::List {
    let configuration = configuration.into_hashmap();
    let default_config = Config::default();
    let config = Config::new(
        match config_to_integer("indent", &configuration, default_config.indent.0) {
            Ok(value) => value,
            Err(error) => return error,
        },
        match config_to_integer("line_length", &configuration, default_config.line_length.0) {
            Ok(value) => value,
            Err(error) => return error,
        },
        match config_to_bool(
            "embracing_op_no_nl",
            &configuration,
            default_config.embracing_op_no_nl.0,
        ) {
            Ok(value) => value,
            Err(error) => return error,
        },
        match config_to_bool(
            "allow_nl_after_assignment",
            &configuration,
            default_config.allow_nl_after_assignment.0,
        ) {
            Ok(value) => value,
            Err(error) => return error,
        },
        match config_to_bool(
            "space_before_complex_rhs_in_formula",
            &configuration,
            default_config.space_before_complex_rhs_in_formula.0,
        ) {
            Ok(value) => value,
            Err(error) => return error,
        },
        match config_to_bool(
            "strip_suffix_whitespace_in_function_defs",
            &configuration,
            default_config.strip_suffix_whitespace_in_function_defs.0,
        ) {
            Ok(value) => value,
            Err(error) => return error,
        },
        match configuration.get("function_line_breaks") {
            Some(text) => match text.as_str() {
                Some("single") => FunctionLineBreaks::Single,
                Some("double") => FunctionLineBreaks::Double,
                Some("hanging") => FunctionLineBreaks::Hanging,
                _ => {
                    return list!(
                        ERROR,
                        "Unknown function line breaks in the configuration value. Allowed: single, double, hanging."
                    )
                }
            }
            None => default_config.function_line_breaks,
        },
        match config_to_bool(
            "insert_newline_in_quote_call",
            &configuration,
            default_config.insert_newline_in_quote_call.0,
        ) {
            Ok(value) => value,
            Err(error) => return error,
        },
        match configuration.get("exclusion_list") {
            Some(list) => match list.as_string_vector() {
                Some(arr) => arr,
                None => {
                    return list!(ERROR, "exclusion_list must be an array of strings.");
                }
            },
            None => default_config.exclusion_list.0,
        }
    );

    match tergo_lib::tergo_format(source_code, Some(&config)) {
        Ok(formatted_code) => {
            list!(OK, formatted_code)
        }
        Err(error) => {
            list!(ERROR, error)
        }
    }
}

/// Parse the config file and return the configuration
///
/// @param path (`character(1)`) the file path of the configuration
/// file
///
/// @return (`list`)
/// @keywords internal
#[extendr]
fn get_config(path: &str) -> extendr_api::List {
    let config = match std::fs::read_to_string(path) {
        Ok(config_file) => {
            toml::from_str::<Config>(&config_file).unwrap_or_else(|_| Config::default())
        }
        Err(_) => Config::default(),
    };

    list!(
        indent = config.indent.0,
        line_length = config.line_length.0,
        embracing_op_no_nl = config.embracing_op_no_nl.0,
        allow_nl_after_assignment = config.allow_nl_after_assignment.0,
        space_before_complex_rhs_in_formula = config.space_before_complex_rhs_in_formula.0,
        strip_suffix_whitespace_in_function_defs =
            config.strip_suffix_whitespace_in_function_defs.0,
        function_line_breaks = match config.function_line_breaks {
            FunctionLineBreaks::Hanging => "hanging",
            FunctionLineBreaks::Double => "double",
            FunctionLineBreaks::Single => "single",
        },
        insert_newline_in_quote_call = config.insert_newline_in_quote_call.0,
        exclusion_list = config.exclusion_list.0
    )
}

/// Get the default configuration
///
/// This configuration is used by the styling functions
/// if no value is provided for the configuration keys.
/// It can also serve as the base for you custom configuration.
///
/// @details
/// The configuration values:
/// * indent (`integer`) - the number of spaces to use for indentation. E.g. 2L, 4L.
/// * line_length (`integer`) - the maximum number of characters in a line. E.g. 80L, 120L.
/// * embracing_op_no_nl (`logical`) - whether to allow a newline after an embracing operator. E.g.
///   TRUE, FALSE.
/// * allow_nl_after_assignment (`logical`) - whether to allow a newline after an assignment operator.
///   E.g. TRUE, FALSE.
/// * space_before_complex_rhs_in_formula (`logical`) - whether to add a space before a complex
///   right-hand side in a formula. E.g. TRUE, FALSE.
/// * strip_suffix_whitespace_in_function_defs (`logical`) - whether to strip suffix
///   whitespace in function definitions. E.g. TRUE, FALSE.
/// * function_line_breaks (`character`) - the type of line breaks in function definitions when arguments do not
///   fit. Possible values are: "hanging", "double", "single".
/// * insert_newline_in_quote_call (`logical`) - whether to insert a newline in calls to `quote`.
///   E.g. TRUE, FALSE.
///
/// @return `list` with the default configuration
/// @export
/// @examples
/// config <- get_default_config()
/// print(config)
///
/// # Make the indent 4 spaces
/// config$indent <- 4L
///
/// # Make the maximum line length 80 characters
/// config$line_length <- 80L
///
/// # Make the function line breaks double
/// config$function_line_breaks <- "double"
#[extendr]
fn get_default_config() -> extendr_api::List {
    let config = Config::default();
    list!(
        indent = config.indent.0,
        line_length = config.line_length.0,
        embracing_op_no_nl = config.embracing_op_no_nl.0,
        allow_nl_after_assignment = config.allow_nl_after_assignment.0,
        space_before_complex_rhs_in_formula = config.space_before_complex_rhs_in_formula.0,
        strip_suffix_whitespace_in_function_defs =
            config.strip_suffix_whitespace_in_function_defs.0,
        function_line_breaks = match config.function_line_breaks {
            FunctionLineBreaks::Hanging => "hanging",
            FunctionLineBreaks::Double => "double",
            FunctionLineBreaks::Single => "single",
        },
        insert_newline_in_quote_call = config.insert_newline_in_quote_call.0,
        exclusion_list = config.exclusion_list.0
    )
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod tergo;
    fn format_code;
    fn get_config;
    fn get_default_config;
}
