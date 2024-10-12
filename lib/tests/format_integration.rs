#![allow(clippy::field_reassign_with_default)]
use tergo_lib::{config::Config, tergo_format};

fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

macro_rules! comparison_test {
    ($name:ident, $file_number:literal) => {
        #[test]
        fn $name() {
            log_init();
            let input = include_str!(concat!("test_cases/", $file_number, ".R"));
            let expected = include_str!(concat!("test_cases/", $file_number, ".expected"));
            assert_eq!(
                tergo_format(input, Some(&long_line_config())).unwrap(),
                expected
            );
        }
    };
    ($name:ident, $file_number:literal, $config:expr) => {
        #[test]
        fn $name() {
            log_init();
            let config: Config = $config;
            let input = include_str!(concat!("test_cases/", $file_number, ".R"));
            let expected = include_str!(concat!("test_cases/", $file_number, ".expected"));
            let result = tergo_format(input, Some(&config)).unwrap();
            let first_difference_line = result
                .lines()
                .zip(expected.lines())
                .enumerate()
                .filter(|(_, (result_line, expect_line))| result_line != expect_line)
                .next();
            assert!(
                result == expected,
                "Formatted text is not what expected. Result \
                 was:\n{}===\nExpected:\n{}===\n\nFirst line of difference was at line \
                 {}:\nResult: {}\nExpected: {}\n",
                result,
                expected,
                first_difference_line.unwrap().0,
                first_difference_line.unwrap().1.0,
                first_difference_line.unwrap().1.1,
            );
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
    let mut config = Config::default();
    config.allow_nl_after_assignment = true;
    config.embracing_op_no_nl = true;
    config.indent = 0;
    config.line_length = 0;
    assert_eq!(tergo_format(input, Some(&config)).unwrap(), expected);

    config.line_length = 4;
    let input = include_str!(concat!("./test_cases/003.R"));
    let expected = include_str!(concat!("./test_cases/003-3-line-length.expected"));
    assert_eq!(tergo_format(input, Some(&config)).unwrap(), expected);
}

fn short_line_config() -> Config {
    let mut config = Config::default();
    config.indent = 0;
    config.line_length = 4;
    config.embracing_op_no_nl = true;
    config.allow_nl_after_assignment = true;
    config
}

fn short_line_plus_indent() -> Config {
    let mut config = short_line_config();
    config.indent = 2;
    config
}

#[allow(clippy::field_reassign_with_default)]
fn long_line_config() -> Config {
    let mut config = Config::default();
    config.embracing_op_no_nl = true;
    config.indent = 0;
    config.line_length = 120;
    config
}

comparison_test!(simple_bop_with_parenthesis, "004");
comparison_test!(
    simple_bop_with_parentheses_forced_to_break_line,
    "005",
    short_line_config()
);
comparison_test!(
    simple_term_with_parentheses_forced_to_break_line,
    "006",
    short_line_config()
);
comparison_test!(
    simple_bop_forced_to_break_and_indent,
    "007",
    short_line_config()
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
    short_line_config()
);
comparison_test!(while_empty_loop, "027");
comparison_test!(while_single_expression_loop, "028");
comparison_test!(while_two_expressions_additional_line_breaks, "029");
comparison_test!(repeat_loop, "030");
comparison_test!(function_call_no_args, "031");
comparison_test!(function_call_one_arg, "032");
comparison_test!(function_call_multiple_args, "033");
comparison_test!(function_call_named_args, "034");
comparison_test!(function_call_multiple_calls, "035");
comparison_test!(empty_subset, "036");
comparison_test!(subset_with_three_args, "037");
comparison_test!(multiple_subset, "038");
comparison_test!(function_call_plus_subset, "039");
comparison_test!(simple_for_loop, "040");
comparison_test!(for_loop_with_multiline_body, "041");
comparison_test!(break_continue, "042");
comparison_test!(lambda_function_test, "043");
comparison_test!(indent_bop, "044", short_line_plus_indent());
comparison_test!(indent_multiline_bop, "045", short_line_plus_indent());
comparison_test!(
    indent_multiline_bop_parenthesized,
    "046",
    short_line_plus_indent()
);
comparison_test!(indent_function_def, "047", short_line_plus_indent());
comparison_test!(indent_multiline_term, "048", short_line_plus_indent());
comparison_test!(indent_conditional_no_brace, "049", short_line_plus_indent());
comparison_test!(
    indent_conditional_with_brace,
    "050",
    short_line_plus_indent()
);
comparison_test!(indent_while_multiline_body, "051", short_line_plus_indent());
comparison_test!(indent_for_loop_complex, "052", short_line_plus_indent());
comparison_test!(
    indent_bop_multiline_many_new_lines,
    "053",
    short_line_plus_indent()
);
comparison_test!(longer_example, "054");
comparison_test!(comment_shows_up, "055");
comparison_test!(
    comments_are_not_part_of_line_length,
    "056",
    long_line_config()
);
comparison_test!(comments_are_not_formatted, "057");
comparison_test!(comments_in_an_array, "058");
comparison_test!(single_leading_comment, "059");
comparison_test!(two_line_leading_comment, "060");
comparison_test!(two_line_with_short_line_config, "060", short_line_config());
comparison_test!(two_leading_comments_one_after_another, "061");
comparison_test!(comments_with_no_code_work, "062");
comparison_test!(parsing_unary_operators, "063");
comparison_test!(binary_operator_with_newline, "064");
comparison_test!(function_definition_with_indent, "065", Config::default());
comparison_test!(
    function_definition_with_args_very_long,
    "066",
    Config::default()
);
comparison_test!(
    function_definition_with_args_very_long_assigned,
    "067",
    Config::default()
);
comparison_test!(multi_bop_with_two_parts_fit_in_one_line, "068", {
    let mut config = Config::default();
    config.line_length = 3;
    config
});
comparison_test!(closure_as_a_function_argument, "069", Config::default());
comparison_test!(closure_as_a_function_argument2, "070", Config::default());
comparison_test!(
    closure_as_a_func_argument_short_line,
    "071",
    short_line_plus_indent()
);
comparison_test!(closure_as_a_function_argument3, "072", Config::default());
comparison_test!(bop_with_dollar, "073", Config::default());

// Tidyverse styleguide examples
comparison_test!(tidyverse_commas, "tidyverse_style_guide_001");
comparison_test!(tidyverse_commas2, "tidyverse_style_guide_002");
comparison_test!(tidyverse_spaces, "tidyverse_style_guide_003");
comparison_test!(
    tidyverse_spaces_if,
    "tidyverse_style_guide_004",
    Config::default()
);
comparison_test!(
    tidyverse_spaces_for_loops,
    "tidyverse_style_guide_005",
    Config::default()
);
comparison_test!(
    tidyverse_spaces_while_loops,
    "tidyverse_style_guide_006",
    Config::default()
);
comparison_test!(
    tidyverse_spaces_around_function,
    "tidyverse_style_guide_007",
    Config::default()
);
comparison_test!(tidyverse_embracing, "tidyverse_style_guide_008", {
    let mut config = Config::default();
    config.line_length = 80;
    config
});
comparison_test!(
    tidyverse_infix_operators,
    "tidyverse_style_guide_009",
    Config::default()
);
comparison_test!(
    tidyverse_infix_operators_high_precedence,
    "tidyverse_style_guide_010",
    Config::default()
);
comparison_test!(
    tidyverse_formulas_simple_rhs,
    "tidyverse_style_guide_011",
    Config::default()
);
comparison_test!(
    tidyverse_formulas_complex_rhs,
    "tidyverse_style_guide_012",
    Config::default()
);
comparison_test!(
    tidyverse_negation_operator,
    "tidyverse_style_guide_013",
    Config::default()
);
comparison_test!(
    tidyverse_help_operator,
    "tidyverse_style_guide_014",
    Config::default()
);
// https://style.tidyverse.org/syntax.html#vertical-space
comparison_test!(
    tidyverse_avoid_empty_lines,
    "tidyverse_style_guide_015",
    Config::default()
);
comparison_test!(
    tidyverse_collapse_whitespace_to_single_line,
    "tidyverse_style_guide_016",
    Config::default()
);
comparison_test!(
    tidyverse_curly_braces,
    "tidyverse_style_guide_017",
    Config::default()
);
comparison_test!(
    tidyverse_ifs,
    "tidyverse_style_guide_018",
    Config::default()
);
comparison_test!(
    tidyverse_long_argument_names,
    "tidyverse_style_guide_019",
    {
        let mut config = Config::default();
        config.line_length = 80;
        config
    }
);
comparison_test!(
    tidyverse_strings_and_quotes,
    "tidyverse_style_guide_020",
    Config::default()
);
comparison_test!(tidyverse_hanging_indent, "tidyverse_style_guide_021", {
    let mut config = Config::default();
    config.line_length = 40;
    config
});
comparison_test!(
    tidyverse_hanging_indent_with_leading_and_trailing_comment,
    "tidyverse_style_guide_022",
    {
        let mut config = Config::default();
        config.line_length = 40;
        config
    }
);
comparison_test!(
    tidyverse_formatting_function_defs_from_single,
    "tidyverse_style_guide_023",
    {
        let mut config = Config::default();
        config.line_length = 40;
        config
    }
);
comparison_test!(tidyverse_pipes, "tidyverse_style_guide_024", {
    let mut config = Config::default();
    config.line_length = 40;
    config
});
comparison_test!(
    tidyverse_pipes_with_long_funcs,
    "tidyverse_style_guide_025",
    {
        let mut config = Config::default();
        config.line_length = 40;
        config
    }
);

// Real life examples
comparison_test!(rle_0, "real_life_000");
comparison_test!(
    rle_short_pipes_fit_one_line,
    "real_life_001",
    Config::default()
);
comparison_test!(rle_collapse_whiteline, "real_life_002", Config::default());
comparison_test!(
    rle_make_line_broke_funcs_fit_one_line,
    "real_life_003",
    Config::default()
);
