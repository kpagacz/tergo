pub trait FormattingConfig: std::fmt::Display {
    fn line_length(&self) -> i32;
    fn indent(&self) -> i32;
    // Custom embracing behaviour: https://style.tidyverse.org/syntax.html#embracing
    fn embracing_op_no_nl(&self) -> bool;
    fn allow_nl_after_assignment(&self) -> bool;
    fn space_before_complex_rhs_in_formulas(&self) -> bool;
}
