pub trait FormattingConfig {
    fn line_length(&self) -> i32;
    fn indent(&self) -> i32;
}
