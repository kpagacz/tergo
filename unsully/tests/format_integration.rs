use unsully::format;

#[test]
fn snippet_001() {
    let input = include_str!(concat!("./test_cases/001.R"));
    let expected = include_str!(concat!("./test_cases/001.expected"));
    assert_eq!(format(input).unwrap(), expected);
}

#[test]
fn snippet_002() {
    let input = include_str!(concat!("./test_cases/002.R"));
    let expected = include_str!(concat!("./test_cases/002.expected"));
    assert_eq!(format(input).unwrap(), expected);
}
