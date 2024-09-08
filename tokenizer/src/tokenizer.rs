use log::trace;

use crate::tokens::{
    CommentedToken,
    Token::{self, *},
};

/// Tokenizer for an R program.
///
/// Transforms an R program into an array of language tokens.
pub struct Tokenizer<'a> {
    line: u32,
    offset: usize,
    it: usize,
    source: Vec<char>,
    raw_source: &'a str,
}

const SYMBOL_ENDING: [char; 29] = [
    ' ', '(', ')', '{', '}', '#', ';', '\n', '\t', '\r', '+', '-', '/', '\\', '%', '*', '^', '!',
    '&', '|', '<', '>', '=', ',', '[', ']', '$', '`', '"',
];
impl<'a> Tokenizer<'a> {
    /// Returns a new tokenizer from an R program.
    ///
    /// # Arguments
    ///
    /// * `input` - the R program to tokenize
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer::tokenizer::Tokenizer;
    ///
    /// let r_program = r#"
    /// a <- 7
    /// print(a + 1)
    /// "#;
    /// let mut tokenizer = Tokenizer::new(r_program);
    /// let tokens = tokenizer.tokenize();
    /// println!("{tokens:?}");
    /// ```
    ///
    pub fn new(input: &'a str) -> Self {
        Self {
            line: 0,
            offset: 0,
            it: 0,
            source: input.chars().collect::<Vec<_>>(),
            raw_source: input,
        }
    }

    /// Returns an array of tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer::tokenizer::Tokenizer;
    ///
    /// let r_program = r#"
    /// a <- 7
    /// print(a + 1)
    /// "#;
    /// let mut tokenizer = Tokenizer::new(r_program);
    /// let tokens = tokenizer.tokenize();
    /// println!("{tokens:?}");
    /// ```
    ///
    pub fn tokenize(&mut self) -> Vec<CommentedToken> {
        let mut tokens = vec![];
        while self.it < self.source.len() {
            match self.source[self.it] {
                ' ' | '\t' => {
                    self.next();
                }
                '\r' => {
                    self.next();
                    self.push_token(Newline, &mut tokens);
                    self.next_line();
                }
                '\n' => {
                    self.push_token(Newline, &mut tokens);
                    self.next_line();
                }
                ';' => {
                    self.push_token(Semicolon, &mut tokens);
                    self.next();
                }
                ',' => {
                    self.push_token(Comma, &mut tokens);
                    self.next();
                }
                '(' => {
                    self.push_token(LParen, &mut tokens);
                    self.next();
                }
                ')' => {
                    self.push_token(RParen, &mut tokens);
                    self.next();
                }
                '{' => {
                    self.push_token(LBrace, &mut tokens);
                    self.next();
                }
                '}' => {
                    self.push_token(RBrace, &mut tokens);
                    self.next();
                }
                '[' => {
                    self.push_token(LBracket, &mut tokens);
                    self.next();
                }
                ']' => {
                    self.push_token(RBracket, &mut tokens);
                    self.next();
                }
                '\'' | '\"' => {
                    self.string_literal(&mut tokens);
                    self.next();
                }
                '*' => {
                    let next_char = self.lookahead().expect("Script does not end on '*'");
                    match next_char {
                        // That's undocumented, but it actually works...
                        '*' => {
                            self.push_token(Power, &mut tokens);
                            self.next();
                        }
                        _ => self.push_token(Multiply, &mut tokens),
                    }
                    self.next();
                }
                '/' => {
                    self.push_token(Divide, &mut tokens);
                    self.next();
                }
                '^' => {
                    self.push_token(Power, &mut tokens);
                    self.next();
                }
                '+' => {
                    self.push_token(Plus, &mut tokens);
                    self.next();
                }
                '?' => {
                    self.push_token(Help, &mut tokens);
                    self.next();
                }
                '<' => {
                    let next_char = self.lookahead().expect("Script does not end on '<'");
                    match next_char {
                        '-' => {
                            self.push_token(LAssign, &mut tokens);
                            self.next();
                        }
                        '=' => {
                            self.push_token(LowerEqual, &mut tokens);
                            self.next();
                        }
                        _ => self.push_token(LowerThan, &mut tokens),
                    }
                    self.next();
                }
                '>' => {
                    let next_char = self.lookahead().expect("Script does not end on '>'");
                    match next_char {
                        '=' => {
                            self.push_token(GreaterEqual, &mut tokens);
                            self.next();
                        }
                        _ => {
                            self.push_token(GreaterThan, &mut tokens);
                        }
                    }
                    self.next();
                }
                '|' => {
                    let next_char = self.lookahead().expect("Script does not end on '|'");
                    match next_char {
                        '|' => {
                            self.push_token(VectorizedOr, &mut tokens);
                            self.next();
                        }
                        '>' => {
                            self.push_token(Pipe, &mut tokens);
                            self.next();
                        }
                        _ => self.push_token(Or, &mut tokens),
                    }
                    self.next();
                }
                '&' => {
                    let next_char = self.lookahead().expect("Script does not end on '&'");
                    match next_char {
                        '&' => {
                            self.push_token(VectorizedAnd, &mut tokens);
                            self.next();
                        }
                        _ => self.push_token(And, &mut tokens),
                    }
                    self.next();
                }
                '=' => {
                    let next_char = self.lookahead().expect("Script does not end on '='");
                    match next_char {
                        '=' => {
                            self.push_token(Equal, &mut tokens);
                            self.next();
                        }
                        _ => self.push_token(OldAssign, &mut tokens),
                    }
                    self.next();
                }
                '$' => {
                    self.push_token(Dollar, &mut tokens);
                    self.next();
                }
                '-' => {
                    let next_char = self.lookahead().expect("Script does not end on '-'");
                    match next_char {
                        '>' => {
                            self.push_token(RAssign, &mut tokens);
                            self.next();
                        }
                        _ => self.push_token(Minus, &mut tokens),
                    }
                    self.next();
                }
                '!' => {
                    self.next();
                    match self.source[self.it..] {
                        ['=', ..] => {
                            self.push_token(NotEqual, &mut tokens);
                            self.next();
                        }
                        _ => self.push_token(UnaryNot, &mut tokens),
                    }
                }
                '.' => {
                    let next_char = self.lookahead().expect("Script does not end on '.'");
                    match next_char {
                        'a'..='z' | 'A'..='Z' => {
                            self.identifier(&mut tokens);
                        }
                        '0'..='9' => {
                            self.number_literal(&mut tokens);
                        }
                        _ => {
                            eprintln!(
                                "Found not alphabetic and non-numeric character after a dot. \
                                 Treating it as an identifier."
                            );
                            self.identifier(&mut tokens);
                        }
                    }
                }
                '`' | '_' => {
                    self.identifier(&mut tokens);
                }
                '%' => {
                    let next_char = self.lookahead().expect("Script does not end on '%'");
                    match next_char {
                        '%' => {
                            self.push_token(Modulo, &mut tokens);
                            self.next();
                            self.next();
                        }
                        _ => self.identifier(&mut tokens),
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    self.identifier_or_reserved(&mut tokens);
                }
                '0'..='9' => {
                    self.number_literal(&mut tokens);
                }
                '\\' => {
                    self.push_token(Lambda, &mut tokens);
                    self.next();
                }
                '#' => {
                    self.comment(&mut tokens);
                }
                '~' => {
                    self.push_token(Tilde, &mut tokens);
                    self.next();
                }
                '@' => {
                    self.push_token(Slot, &mut tokens);
                    self.next();
                }
                ':' => {
                    self.next();
                    match self.source[self.it..] {
                        [':', ':', ..] => {
                            self.push_token(NsGetInt, &mut tokens);
                            self.next();
                            self.next();
                        }
                        [':', ..] => {
                            self.push_token(NsGet, &mut tokens);
                            self.next()
                        }
                        _ => self.push_token(Colon, &mut tokens),
                    }
                }
                _ => unreachable!(),
            }
        }
        tokens.push(CommentedToken::new(EOF, self.line, self.offset));
        trace!("Tokenized: {:?}", tokens);
        tokens
    }

    fn push_token(&mut self, token: Token<'a>, tokens: &mut Vec<CommentedToken<'a>>) {
        tokens.push(CommentedToken::new(token, self.line, self.offset));
    }

    fn string_literal(&mut self, tokens: &mut Vec<CommentedToken<'a>>) {
        let delimiter = self.source[self.it];
        let (start_line, start_offset) = (self.line, self.offset);
        let start_it = self.it;
        self.next();
        while self.source[self.it] != delimiter || self.source[self.it - 1] == '\\' {
            if self.source[self.it] == '\n' {
                self.next_line();
            } else {
                self.next();
            }
        }
        tokens.push(CommentedToken::new(
            Literal(&self.raw_source[start_it..=self.it]),
            start_line,
            start_offset,
        ));
    }

    fn parse_decimal(&mut self) {
        while self.it < self.source.len() && self.source[self.it].is_ascii_digit() {
            self.next();
        }
    }

    fn parse_hexadecimal(&mut self) {
        while self.it < self.source.len() && self.source[self.it].is_ascii_hexdigit() {
            self.next();
        }
    }

    fn number_literal(&mut self, tokens: &mut Vec<CommentedToken<'a>>) {
        let start_it = self.it;
        match self.source[self.it..] {
            // Hexadecimal
            ['0', 'x', ..] | ['0', 'X', ..] => {
                self.next();
                self.next();
                self.parse_hexadecimal();
                if let ['.', ..] = self.source[self.it..] {
                    self.next();
                    self.parse_hexadecimal();
                    match self.source[self.it..] {
                        ['p', ..] | ['P', ..] => {
                            self.next();
                            self.parse_hexadecimal();
                        }
                        _ => {}
                    }
                }
            }
            // Decimal
            _ => {
                self.parse_decimal();
                match self.source[self.it..] {
                    ['.', ..] => {
                        self.next();
                        self.parse_decimal();
                        match self.source[self.it..] {
                            ['e', '+', ..] | ['E', '+', ..] | ['e', '-', ..] | ['E', '-', ..] => {
                                self.next();
                                self.next();
                                self.parse_decimal();
                            }
                            ['e', ..] | ['E', ..] => {
                                self.next();
                                self.parse_decimal();
                            }
                            _ => {}
                        }
                    }
                    ['e', '+', ..] | ['E', '+', ..] | ['e', '-', ..] | ['E', '-', ..] => {
                        self.next();
                        self.next();
                        self.parse_decimal();
                    }
                    ['e', ..] | ['E', ..] => {
                        self.next();
                        self.parse_decimal();
                    }
                    _ => {}
                }
            }
        }
        self.push_token(Literal(&self.raw_source[start_it..self.it]), tokens);
    }

    fn identifier(&mut self, tokens: &mut Vec<CommentedToken<'a>>) {
        let start_it = self.it;
        while self.it < self.source.len() && self.source[self.it].is_alphabetic()
            || self.source[self.it] == '.'
            || self.source[self.it] == '_'
        {
            self.next();
        }
        match &self.raw_source[start_it..self.it] {
            "TRUE" | "T" => self.push_token(Literal("TRUE"), tokens),
            "FALSE" | "F" => self.push_token(Literal("FALSE"), tokens),
            _ => self.push_token(Symbol(&self.raw_source[start_it..self.it]), tokens),
        }
    }

    fn identifier_or_reserved(&mut self, tokens: &mut Vec<CommentedToken<'a>>) {
        let start_it = self.it;
        while self.it < self.source.len() && !SYMBOL_ENDING.contains(&self.source[self.it]) {
            self.next();
        }

        match &self.raw_source[start_it..self.it] {
            "continue" => self.push_token(Continue, tokens),
            "break" => self.push_token(Break, tokens),
            "for" => self.push_token(For, tokens),
            "if" => self.push_token(If, tokens),
            "else" => self.push_token(Else, tokens),
            "in" => self.push_token(In, tokens),
            "while" => self.push_token(While, tokens),
            "repeat" => self.push_token(Repeat, tokens),
            "function" => self.push_token(Function, tokens),
            "TRUE" | "T" => self.push_token(Literal("TRUE"), tokens),
            "FALSE" | "F" => self.push_token(Literal("FALSE"), tokens),
            _ => self.push_token(Symbol(&self.raw_source[start_it..self.it]), tokens),
        }
    }

    fn comment(&mut self, tokens: &mut Vec<CommentedToken<'a>>) {
        let start_it = self.it;
        while self.it < self.source.len() && self.source[self.it] != '\n' {
            self.next();
        }

        match tokens.last() {
            Some(CommentedToken {
                token: Newline,
                line: _,
                offset: _,
                leading_comments: _,
                inline_comment: _,
            }) => self.push_token(Comment(&self.raw_source[start_it..self.it]), tokens),
            Some(_) => self.push_token(InlineComment(&self.raw_source[start_it..self.it]), tokens),
            None => self.push_token(Comment(&self.raw_source[start_it..self.it]), tokens),
        }
    }

    fn next(&mut self) {
        self.it += 1;
        self.offset += 1;
    }

    fn next_line(&mut self) {
        self.it += 1;
        self.line += 1;
        self.offset = 0;
    }

    fn lookahead(&self) -> Option<char> {
        if self.it + 1 < self.source.len() {
            Some(self.source[self.it + 1])
        } else {
            None
        }
    }
}
