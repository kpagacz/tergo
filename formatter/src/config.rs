pub trait FormattingConfig: std::fmt::Display {
    fn line_length(&self) -> i32;
    fn indent(&self) -> i32;
}
