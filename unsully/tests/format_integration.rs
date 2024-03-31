use unsully::{config::Config, format};

#[test]
fn adds_a_newline_at_the_end() {
    // let input = include_str!(concat!("./test_cases/001.R"));
    // let expected = include_str!(concat!("./test_cases/001.expected"));
    // assert_eq!(format(input, None).unwrap(), expected);

    // let input = include_str!(concat!("./test_cases/002.R"));
    // let expected = include_str!(concat!("./test_cases/002.expected"));
    // assert_eq!(format(input, None).unwrap(), expected);
}

#[test]
fn simple_bops() {
    // let input = include_str!(concat!("./test_cases/003.R"));
    // let expected = include_str!(concat!("./test_cases/003.expected"));
    // assert_eq!(format(input, None).unwrap(), expected);
}

#[test]
fn simple_bops_indents_and_new_lines() {
    let input = include_str!(concat!("./test_cases/003.R"));
    let expected = include_str!(concat!("./test_cases/003.expected"));
    let config = Config::new(0, 0);
    assert_eq!(format(input, Some(config)).unwrap(), expected);
}
