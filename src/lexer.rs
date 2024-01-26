use crate::token::KEYWORD_TABLE;
use crate::{Loc, Token, TokenValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    start_idx: u32,
    start_loc: Loc,
    curr_idx: u32,
    curr_loc: Loc,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut l = Self {
            source,
            tokens: Vec::new(),
            start_idx: 0,
            start_loc: Loc { line: 1, col: 1 },
            curr_idx: 0,
            curr_loc: Loc { line: 1, col: 1 },
        };
        l.scan_tokens();
        l
    }

    pub fn emit(&self) {
        for token in &self.tokens {
            println!("{:?}", token);
        }
    }

    fn error(&self, message: String, line: u32, col: u32) {
        panic!("{} at {}:{}", message, line, col);
    }

    fn is_at_end(&self) -> bool {
        self.curr_idx >= self.source.len() as u32
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.curr_idx as usize).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.curr_idx + 1 >= self.source.len() as u32 {
            '\0'
        } else {
            self.source
                .chars()
                .nth((self.curr_idx + 1) as usize)
                .unwrap()
        }
    }

    fn increment_position(&mut self) {
        if self.peek() == '\n' {
            self.curr_idx += 1;
            self.curr_loc.line += 1;
            self.curr_loc.col = 1;
        } else {
            self.curr_idx += 1;
            self.curr_loc.col += 1;
        }
    }

    fn next(&mut self) -> char {
        let c = self.peek();
        self.increment_position();
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek() != expected {
            false
        } else {
            self.increment_position();
            true
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start_idx = self.curr_idx;
            self.start_loc = self.curr_loc;
            self.scan_token();
        }
        self.start_idx = self.curr_idx;
        self.start_loc = self.curr_loc;
        // self.tokens
        // .push(Token::new(self.start_loc, self.start_loc, TokenValue::EOF));
    }

    fn is_digit(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }
    fn is_alpha(c: char) -> bool {
        match c {
            'A'..='Z' | 'a'..='z' | '_' => true,
            _ => false,
        }
    }
    fn is_alphanumeric(c: char) -> bool {
        Self::is_digit(c) || Self::is_alpha(c)
    }

    fn scan_token(&mut self) {
        let c: char = self.next();
        match c {
            '(' => self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::LeftParen,
            )),
            ')' => self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::RightParen,
            )),
            '{' => self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::LeftBrace,
            )),
            '}' => self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::RightBrace,
            )),
            ',' => self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Comma)),
            '.' => self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Dot)),
            ';' => self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::Semicolon,
            )),
            ':' => {
                if self.match_char(':') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::ColonColon,
                    ));
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Colon));
                }
            }
            '+' => {
                if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::PlusEqual,
                    ))
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Plus))
                }
            }
            '-' => {
                if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::MinusEqual,
                    ))
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Minus))
                }
            }
            '*' => {
                if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::StarEqual,
                    ))
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Star))
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.increment_position();
                    }
                } else if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::SlashEqual,
                    ))
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Slash))
                }
            }
            '%' => {
                if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::ModEqual,
                    ))
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Mod))
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::EqualEqual,
                    ))
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Equal))
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::NotEqual,
                    ))
                } else {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::LogNot,
                    ))
                }
            }
            '<' => {
                if self.match_char('<') {
                    if self.match_char('=') {
                        self.add_token(Token::new(
                            self.start_loc,
                            self.curr_loc,
                            TokenValue::LeftShiftEqual,
                        ))
                    } else {
                        self.add_token(Token::new(
                            self.start_loc,
                            self.curr_loc,
                            TokenValue::LeftShift,
                        ))
                    }
                } else if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::LessEqual,
                    ))
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::Less))
                }
            }
            '>' => {
                if self.match_char('>') {
                    if self.match_char('=') {
                        self.add_token(Token::new(
                            self.start_loc,
                            self.curr_loc,
                            TokenValue::RightShiftEqual,
                        ))
                    } else {
                        self.add_token(Token::new(
                            self.start_loc,
                            self.curr_loc,
                            TokenValue::RightShift,
                        ))
                    }
                } else if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::GreaterEqual,
                    ))
                } else {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::Greater,
                    ))
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::LogAnd,
                    ))
                } else if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::BitAndEqual,
                    ))
                } else {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::BitAnd,
                    ))
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::LogOr))
                } else if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::BitOrEqual,
                    ))
                } else {
                    self.add_token(Token::new(self.start_loc, self.curr_loc, TokenValue::BitOr))
                }
            }
            '~' => self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::BitNot,
            )),
            '^' => {
                if self.match_char('=') {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::BitXorEqual,
                    ))
                } else {
                    self.add_token(Token::new(
                        self.start_loc,
                        self.curr_loc,
                        TokenValue::BitXor,
                    ))
                }
            }
            '"' => self.scan_string(),
            ' ' | '\r' | '\t' | '\n' => (),
            '0'..='9' => self.scan_number(),
            'A'..='Z' | 'a'..='z' | '_' => self.scan_identifier(),

            _ => self.error(
                "Unexpected character".to_string(),
                self.start_loc.line,
                self.start_loc.col,
            ),
        }
    }

    fn scan_string(&mut self) {
        let mut str = String::new();
        while self.peek() != '"' && !self.is_at_end() {
            let c = self.next();
            if c == '\\' {
                if self.match_char('0') {
                    str.push('\0');
                } else if self.match_char('n') {
                    str.push('\n');
                } else if self.match_char('r') {
                    str.push('\r');
                } else if self.match_char('t') {
                    str.push('\t');
                } else if self.match_char('\\') {
                    str.push('\\');
                } else if self.match_char('"') {
                    str.push('"');
                } else {
                    self.error(
                        "Invalid escape sequence".to_string(),
                        self.curr_loc.line,
                        self.curr_loc.col,
                    );
                }
            } else {
                str.push(c);
            }
        }

        if self.is_at_end() {
            self.error(
                "Unterminated string".to_string(),
                self.curr_loc.line,
                self.curr_loc.col,
            );
        }

        self.next();
        self.add_token(Token::new(
            self.start_loc,
            self.curr_loc,
            TokenValue::String(str),
        ));
    }

    fn scan_number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.increment_position();
        }
        if !(self.peek() == '.' && Self::is_digit(self.peek_next())) {
            self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::Integer(
                    self.source
                        .get((self.start_idx as usize)..(self.curr_idx as usize))
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                ),
            ));
        } else {
            self.increment_position();
            while self.peek().is_ascii_digit() {
                self.increment_position();
            }
            self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::Float(
                    self.source
                        .get((self.start_idx as usize)..(self.curr_idx as usize))
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                ),
            ));
        }
    }

    fn scan_identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.increment_position();
        }

        let word = self
            .source
            .get((self.start_idx as usize)..(self.curr_idx as usize))
            .unwrap()
            .to_string();
        if let Some(token_value) = KEYWORD_TABLE.get(&word) {
            self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                token_value.clone(),
            ));
        } else {
            self.add_token(Token::new(
                self.start_loc,
                self.curr_loc,
                TokenValue::Identifier(word),
            ));
        }
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}
