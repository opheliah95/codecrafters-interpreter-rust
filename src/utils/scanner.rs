use crate::utils::token::{Token, TokenType};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u16,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    pub fn is_at_end(&self) -> bool {
        // println!(
        //     "current {} vs source: {} len: {}",
        //     self.current,
        //     self.source,
        //     self.source.len()
        // );
        return self.current >= self.source.len();
    }

    fn add_token(&mut self, token_type: TokenType, value: Option<&str>) {
        let txt = self.source.get(self.start..self.current).unwrap();
        let literal = value.unwrap_or("null");
        let new_token = Token {
            token_type: token_type,
            lexeme: txt.to_string(),
            literal: literal.to_string(),
            line: self.line,
        };

        self.tokens.push(new_token);
    }

    fn advance(&mut self) -> char {
        let source_cloned = self.source.clone();
        let char_found = source_cloned.chars().nth(self.current).unwrap_or_default();
        self.current += 1;
        //eprintln!("char={char_found} advamce to {}", self.current);
        return char_found;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.source.chars().nth(self.current).unwrap_or_default();
        }
    }
    pub fn exit(self) {
        if self.has_error {
            std::process::exit(65);
        } else {
            std::process::exit(0);
        }
    }

    pub fn peek_next(&mut self) -> char {
        let source_cloned = self.source.clone();
        let current_clone = self.current;
        if current_clone + 1 >= self.source.len() {
            return '\0';
        }
        // eprintln!("current is {current_clone}");
        let char_found = source_cloned.chars().nth(current_clone + 1).unwrap();
        return char_found;
    }

     fn string_analyzer(&mut self) {
        while self.peek() != '"' && ! self.is_at_end() {
            // handle new line
            if self.peek() == '\n' {
                self.line += 1
            }

            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.has_error = true;
        }

        // handle the end ""
        self.advance();
        let output = self.source[self.start + 1..self.current-1].to_string();
        self.add_token(TokenType::STRING, Some(&output));
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance();
        //eprintln!("c is {c} c=whitespace => {}", c =='\n');
        match (c) {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            '*' => self.add_token(TokenType::STAR, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '"' => {
                self.string_analyzer();
                return;
            }
            '\n' => {
                self.line += 1;
                //eprintln!("whitespace line is {}", self.line);
            },
            x if vec![' ',  '\r'].contains(&x) => {
                return;
            },
            '\t' => return, 
            '/' => {
                let next = self.peek();
                // see if this is a comment
                match next {
                    '/' => {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance(); // advancing till at end of comment
                        }
                    }
                    _ => self.add_token(TokenType::SLASH, None),
                }
            }
            '=' => {
                let next = self.peek();
                //eprintln!("next is {next}");
                match next {
                    '=' => {
                        self.advance(); // remove next token
                        self.add_token(TokenType::EQUAL_EQUAL, None);
                    }

                    _ => self.add_token(TokenType::EQUAL, None),
                }
            }
            '!' => {
                let next = self.peek();
                //eprintln!("next is {next}");
                match next {
                    '=' => {
                        self.advance(); // remove next token
                        self.add_token(TokenType::BANG_EQUAL, None);
                    }

                    _ => self.add_token(TokenType::BANG, None),
                }
            }
            '>' => {
                let next = self.peek();
                //eprintln!("next is {next}");
                match next {
                    '=' => {
                        self.advance(); // remove next token
                        self.add_token(TokenType::GREATER_EQUAL, None);
                    }

                    _ => self.add_token(TokenType::GREATER, None),
                }
            }
            '<' => {
                let next = self.peek();
                //eprintln!("next is {next}");
                match next {
                    '=' => {
                        self.advance(); // remove next token
                        self.add_token(TokenType::LESS_EQUAL, None);
                    }

                    _ => self.add_token(TokenType::LESS, None),
                }
            }

            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                self.has_error = true;
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: "null".to_string(),
            line: 1,
        });

        return self.tokens.clone();
    }
}
