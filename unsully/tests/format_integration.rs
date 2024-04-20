use unsully::{config::Config, format};

fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

macro_rules! comparison_test {
    ($name: ident, $file_number: literal) => {
        #[test]
        fn $name() {
            log_init();
            let input = include_str!(concat!("test_cases/", $file_number, ".R"));
            let expected = include_str!(concat!("test_cases/", $file_number, ".expected"));
            assert_eq!(format(input, None).unwrap(), expected);
        }
    };
    ($name: ident, $file_number: literal, $config: ident) => {
        #[test]
        fn $name() {
            log_init();
            let input = include_str!(concat!("test_cases/", $file_number, ".R"));
            let expected = include_str!(concat!("test_cases/", $file_number, ".expected"));
            assert_eq!(format(input, Some($config())).unwrap(), expected);
        }
    };
}

comparison_test!(adds_a_newline_at_the_end, "001");
comparison_test!(adds_a_newline_at_the_end2, "002");
comparison_test!(simple_bops, "003");

#[test]
fn simple_bops_indents_and_new_lines() {
    log_init();
    let input = include_str!(concat!("./test_cases/003.R"));
    let expected = include_str!(concat!("./test_cases/003-0-line-length.expected"));
    let config = Config::new(0, 0);
    assert_eq!(format(input, Some(config)).unwrap(), expected);

    let config = Config::new(0, 4);
    let input = include_str!(concat!("./test_cases/003.R"));
    let expected = include_str!(concat!("./test_cases/003-3-line-length.expected"));
    assert_eq!(format(input, Some(config)).unwrap(), expected);
}

fn short_line_config() -> Config {
    Config::new(0, 4)
}

comparison_test!(simple_bop_with_parenthesis, "004");
comparison_test!(
    simple_bop_with_parentheses_forced_to_break_line,
    "005",
    short_line_config
);
comparison_test!(
    simple_term_with_parentheses_forced_to_break_line,
    "006",
    short_line_config
);
comparison_test!(
    simple_bop_forced_to_break_and_indent,
    "007",
    short_line_config
);
comparison_test!(range_bop_one_line, "008");
comparison_test!(parenthesized_bop_one_line, "009");
comparison_test!(simple_function_definition, "010");
comparison_test!(function_definition_no_args_one_expression, "011");
comparison_test!(function_definition_no_args_two_expressions, "012");
comparison_test!(function_definition_one_arg_no_body, "013");
comparison_test!(function_definition_tw0_arg_no_body, "014");
comparison_test!(function_definition_one_default_arg_no_body, "015");
comparison_test!(function_definition_three_args_multiline_body, "016");
comparison_test!(simple_conditional, "017");
comparison_test!(conditional_with_one_expression_in_body, "018");
comparison_test!(conditional_with_two_expression_in_body, "019");
comparison_test!(conditional_with_empty_trailing_else, "020");
comparison_test!(conditional_with_one_expr_trailing_else, "021");
comparison_test!(conditional_with_one_expr_and_one_expr_trailing_else, "022");
comparison_test!(conditional_with_if_else, "023");
comparison_test!(conditional_with_if_if_else_and_trailing_else, "024");
comparison_test!(term_with_braces, "025");
comparison_test!(
    conditional_with_if_if_else_and_trailing_else_short_lines,
    "026",
    short_line_config
);
comparison_test!(while_empty_loop, "027");
comparison_test!(while_single_expression_loop, "028");
comparison_test!(while_two_expressions_additional_line_breaks, "029");
comparison_test!(repeat_loop, "030");
