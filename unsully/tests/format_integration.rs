use unsully::format;

#[test]
fn adds_a_newline_at_the_end() {
    let input = include_str!(concat!("./test_cases/001.R"));
    let expected = include_str!(concat!("./test_cases/001.expected"));
    assert_eq!(format(input).unwrap(), expected);

    let input = include_str!(concat!("./test_cases/002.R"));
    let expected = include_str!(concat!("./test_cases/002.expected"));
    assert_eq!(format(input).unwrap(), expected);
}

#[test]
fn simple_bops() {
    let input = include_str!(concat!("./test_cases/003.R"));
    let expected = include_str!(concat!("./test_cases/003.expected"));
    assert_eq!(format(input).unwrap(), expected);
}
