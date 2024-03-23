use crate::tokens::{
    LocatedToken,
    Token::{self, *},
};

/// Tokenizer for an R program.
///
/// Transforms an R program into an array of language tokens.
pub struct Tokenizer<'a> {
    pub tokens: Vec<LocatedToken<'a>>,
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
            tokens: vec![],
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
    pub fn tokenize(&mut self) -> Vec<LocatedToken> {
        while self.it < self.source.len() {
            match self.source[self.it] {
                ' ' | '\t' => {
                    self.next();
                }
                '\r' => {
                    self.next();
                    self.push_token(Newline);
                    self.next();
                }
                '\n' => {
                    self.push_token(Newline);
                    self.next_line();
                }
                ';' => {
                    self.push_token(Semicolon);
                    self.next();
                }
                ',' => {
                    self.push_token(Comma);
                    self.next();
                }
                '(' => {
                    self.push_token(LParen);
                    self.next();
                }
                ')' => {
                    self.push_token(RParen);
                    self.next();
                }
                '{' => {
                    self.push_token(LBrace);
                    self.next();
                }
                '}' => {
                    self.push_token(RBrace);
                    self.next();
                }
                '[' => {
                    self.push_token(LSubscript);
                    self.next();
                }
                ']' => {
                    self.push_token(RSubscript);
                    self.next();
                }
                '\'' | '\"' => {
                    self.string_literal();
                    self.next();
                }
                '*' => {
                    let next_char = self.lookahead().expect("Script does not end on '*'");
                    match next_char {
                        // That's undocumented, but it actually works...
                        '*' => {
                            self.push_token(Power);
                            self.next();
                        }
                        _ => self.push_token(Multiply),
                    }
                    self.next();
                }
                '/' => {
                    self.push_token(Divide);
                    self.next();
                }
                '^' => {
                    self.push_token(Power);
                    self.next();
                }
                '+' => {
                    self.push_token(Plus);
                    self.next();
                }
                '?' => {
                    self.push_token(Help);
                    self.next();
                }
                '<' => {
                    let next_char = self.lookahead().expect("Script does not end on '<'");
                    match next_char {
                        '-' => {
                            self.push_token(LAssign);
                            self.next();
                        }
                        '=' => {
                            self.push_token(LowerEqual);
                            self.next();
                        }
                        _ => self.push_token(LowerThan),
                    }
                    self.next();
                }
                '>' => {
                    let next_char = self.lookahead().expect("Script does not end on '>'");
                    match next_char {
                        '=' => {
                            self.push_token(GreaterEqual);
                            self.next();
                        }
                        _ => {
                            self.push_token(GreaterThan);
                        }
                    }
                    self.next();
                }
                '|' => {
                    let next_char = self.lookahead().expect("Script does not end on '|'");
                    match next_char {
                        '|' => {
                            self.push_token(VectorizedOr);
                            self.next();
                        }
                        '>' => {
                            self.push_token(Pipe);
                            self.next();
                        }
                        _ => self.push_token(Or),
                    }
                    self.next();
                }
                '&' => {
                    let next_char = self.lookahead().expect("Script does not end on '&'");
                    match next_char {
                        '&' => {
                            self.push_token(VectorizedAnd);
                            self.next();
                        }
                        _ => self.push_token(And),
                    }
                    self.next();
                }
                '=' => {
                    let next_char = self.lookahead().expect("Script does not end on '='");
                    match next_char {
                        '=' => {
                            self.push_token(Equal);
                            self.next();
                        }
                        _ => self.push_token(OldAssign),
                    }
                    self.next();
                }
                '$' => {
                    self.push_token(Dollar);
                    self.next();
                }
                '-' => {
                    let next_char = self.lookahead().expect("Script does not end on '-'");
                    match next_char {
                        '>' => {
                            self.push_token(RAssign);
                            self.next();
                        }
                        _ => self.push_token(Minus),
                    }
                    self.next();
                }
                '!' => {
                    self.next();
                    match self.source[self.it..] {
                        ['=', ..] => {
                            self.push_token(NotEqual);
                            self.next();
                        }
                        _ => self.push_token(UnaryNot),
                    }
                }
                '.' => {
                    let next_char = self.lookahead().expect("Script does not end on '.'");
                    match next_char {
                        'a'..='z' | 'A'..='Z' => {
                            self.identifier();
                        }
                        '0'..='9' => {
                            self.number_literal();
                        }
                        _ => {
                            eprintln!(
                                "Found not alphabetic and non-numeric character after a dot. Treating it as an identifier."
                            );
                            self.identifier();
                        }
                    }
                }
                '`' | '_' => {
                    self.identifier();
                }
                '%' => {
                    let next_char = self.lookahead().expect("Script does not end on '%'");
                    match next_char {
                        '%' => {
                            self.push_token(Modulo);
                            self.next();
                            self.next();
                        }
                        _ => self.identifier(),
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    self.identifier_or_reserved();
                }
                '0'..='9' => {
                    self.number_literal();
                }
                '\\' => {
                    self.push_token(Lambda);
                    self.next();
                }
                '#' => {
                    self.comment();
                }
                '~' => {
                    self.push_token(Tilde);
                    self.next();
                }
                '@' => {
                    self.push_token(Slot);
                    self.next();
                }
                ':' => {
                    self.next();
                    match self.source[self.it..] {
                        [':', ':', ..] => {
                            self.push_token(NsGetInt);
                            self.next();
                            self.next();
                        }
                        [':', ..] => {
                            self.push_token(NsGet);
                            self.next()
                        }
                        _ => self.push_token(Colon),
                    }
                }
                _ => unreachable!(),
            }
        }
        self.tokens
            .push(LocatedToken::new(EOF, self.line, self.offset));
        self.tokens.clone()
    }

    fn push_token(&mut self, token: Token<'a>) {
        self.tokens
            .push(LocatedToken::new(token, self.line, self.offset));
    }

    fn string_literal(&mut self) {
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
        self.tokens.push(LocatedToken::new(
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

    fn number_literal(&mut self) {
        let start_it = self.offset;
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
        self.push_token(Literal(&self.raw_source[start_it..self.it]));
    }

    fn identifier(&mut self) {
        let start_it = self.it;
        while self.it < self.source.len() && self.source[self.it].is_alphabetic()
            || self.source[self.it] == '.'
            || self.source[self.it] == '_'
        {
            self.next();
        }
        self.push_token(Symbol(&self.raw_source[start_it..self.it]));
    }

    fn identifier_or_reserved(&mut self) {
        let start_it = self.it;
        while self.it < self.source.len() && !SYMBOL_ENDING.contains(&self.source[self.it]) {
            self.next();
        }

        match &self.raw_source[start_it..self.it] {
            "continue" => self.push_token(Continue),
            "break" => self.push_token(Break),
            "for" => self.push_token(For),
            "if" => self.push_token(If),
            "else" => self.push_token(Else),
            "in" => self.push_token(In),
            "while" => self.push_token(While),
            "repeat" => self.push_token(Repeat),
            "function" => self.push_token(Function),
            _ => self.push_token(Symbol(&self.raw_source[start_it..self.it])),
        }
    }

    fn comment(&mut self) {
        let start_it = self.it;
        while self.it < self.source.len() && self.source[self.it] != '\n' {
            self.next();
        }

        match self.tokens.last() {
            Some(LocatedToken {
                token: Newline,
                line: _,
                offset: _,
            }) => self.push_token(Comment(&self.raw_source[start_it..self.it])),
            Some(_) => self.push_token(InlineComment(&self.raw_source[start_it..self.it])),
            None => self.push_token(Comment(&self.raw_source[start_it..self.it])),
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
