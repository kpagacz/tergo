use formatter::config::FormattingConfig;

#[derive(Debug, Clone, Copy)]
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

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "indent: {} line_length: {}",
            self.indent, self.line_length
        ))
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
