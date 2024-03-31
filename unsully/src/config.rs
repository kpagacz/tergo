use formatter::config::FormattingConfig;

pub struct Config {
    indent: i32,
    line_length: i32,
}

impl FormattingConfig for Config {
    fn line_length(&self) -> i32 {
        self.line_length
    }

    fn indent(&self) -> i32 {
        self.indent
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indent: 2,
            line_length: 120,
        }
    }
}

impl Config {
    pub fn new(indent: i32, line_length: i32) -> Self {
        Self {
            indent,
            line_length,
        }
    }
}
