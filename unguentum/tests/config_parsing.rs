use tergo_formatter::config::{Config, FunctionLineBreaks};

fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn parses_the_fully_specified_config() {
    log_init();
    let full_spec = include_str!("./config_cases/full_spec.toml");
    let config: Result<Config, _> = toml::from_str(full_spec);

    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(config.indent.0 == 2);
    assert!(config.line_length.0 == 120);
    assert!(config.embracing_op_no_nl.0);
    assert!(!config.allow_nl_after_assignment.0);
    assert!(config.space_before_complex_rhs_in_formula.0);
    assert!(config.strip_suffix_whitespace_in_function_defs.0);
    assert!(config.function_line_breaks == FunctionLineBreaks::Double);
    assert!(config.insert_newline_in_quote_call.0);
    assert!(config.exclusion_list.0.is_empty());
}

#[test]
fn parses_the_partial_config() {
    log_init();
    let partial_spec = include_str!("./config_cases/partial_spec.toml");
    let config: Result<Config, _> = toml::from_str(partial_spec);

    assert!(config.is_ok(), "Error was {config:?}");
}
