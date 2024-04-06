use unsully::{config::Config, format};

fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn adds_a_newline_at_the_end() {
    log_init();
    let input = include_str!(concat!("./test_cases/001.R"));
    let expected = include_str!(concat!("./test_cases/001.expected"));
    assert_eq!(format(input, None).unwrap(), expected);

    let input = include_str!(concat!("./test_cases/002.R"));
    let expected = include_str!(concat!("./test_cases/002.expected"));
    assert_eq!(format(input, None).unwrap(), expected);
}

#[test]
fn simple_bops() {
    log_init();
    let input = include_str!(concat!("./test_cases/003.R"));
    let expected = include_str!(concat!("./test_cases/003.expected"));
    assert_eq!(format(input, None).unwrap(), expected);
}

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

#[test]
fn simple_bop_with_parenthesis() {
    log_init();
    let input = include_str!(concat!("./test_cases/004.R"));
    let expected = include_str!(concat!("./test_cases/004.expected"));
    assert_eq!(format(input, None).unwrap(), expected);
}

#[test]
fn simple_bop_with_parentheses_forced_to_break_line() {
    log_init();
    let input = include_str!(concat!("./test_cases/005.R"));
    let expected = include_str!(concat!("./test_cases/005.expected"));
    let config = Config::new(0, 4);
    assert_eq!(format(input, Some(config)).unwrap(), expected);
}

#[test]
fn simple_term_with_parentheses_forced_to_break_line() {
    log_init();
    let input = include_str!(concat!("./test_cases/006.R"));
    let expected = include_str!(concat!("./test_cases/006.expected"));
    let config = Config::new(0, 2);
    assert_eq!(format(input, Some(config)).unwrap(), expected);
}

#[test]
fn simple_bop_forced_to_break_and_indent() {
    log_init();
    let input = include_str!(concat!("./test_cases/007.R"));
    let expected = include_str!(concat!("./test_cases/007.expected"));
    let config = Config::new(2, 2);
    assert_eq!(format(input, Some(config)).unwrap(), expected);
}

#[test]
fn range_bop_one_line() {
    log_init();
    let input = include_str!(concat!("./test_cases/008.R"));
    let expected = include_str!(concat!("./test_cases/008.expected"));
    assert_eq!(format(input, None).unwrap(), expected);
}
