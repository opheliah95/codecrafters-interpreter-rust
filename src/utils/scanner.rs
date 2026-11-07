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

    fn add_token(&mut self, token_type: TokenType) {
        let txt = self.source.get(self.start..self.current).unwrap_or("null");
        let digit = txt.parse::<f64>();
        let mut literal = txt.to_string();

        if token_type == TokenType::STRING || token_type == TokenType::NUMBER{

        }
        match digit {
            Ok(d) => {
                let decimals = txt.rfind(".").unwrap_or(0);
                if decimals == 0 {
                    literal = format!("{:.1}", d);
                } else {
                    let after_decimal: Vec<char> = txt[decimals + 1..].chars().collect();
                    //eprintln!("after decimals {:?}", after_decimal);
                    if after_decimal.iter().all(|x| *x == '0') {
                        literal = format!("{:.1}", d);
                    } else if after_decimal.len() > 1 && *after_decimal.last().unwrap() == '0' {
                        let final_digit = txt.len() - 1;
                        literal = format!("{}.{}", &txt[..decimals], &txt[decimals + 1..final_digit]);
                    } else {
                        literal = format!("{}.{}", &txt[..decimals], &txt[decimals + 1..]);
                    }
                }
            }

            Err(_) => {}
        }
        let new_token = Token {
            token_type: token_type,
            lexeme: txt.to_string(),
            literal: literal,
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
        while self.peek() != '"' && !self.is_at_end() {
            // handle new line
            if self.peek() == '\n' {
                self.line += 1
            }

            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.has_error = true;
            return;
        }

        // handle the end ""
        self.advance();
        let output = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::STRING);
    }

    fn number(&mut self) {
        let mut peek = self.peek();
        while self.is_digit(peek) {
            self.advance();
            peek = self.peek();
            let peek_next = self.peek_next();

            if peek == '.' || self.is_digit(peek_next) {
                self.advance();
                peek = self.peek();
            }
        }
        self.add_token(TokenType::NUMBER);
    }

    fn is_digit(&mut self, c: char) -> bool {
        return c.is_ascii_digit();
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance();
        //eprintln!("c is {c} c=whitespace => {}", c =='\n');
        match (c) {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            '*' => self.add_token(TokenType::STAR),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            d if self.is_digit(d) => {
                self.number();
            }
            '"' => {
                self.string_analyzer();
                return;
            }
            '\n' => {
                self.line += 1;
                //eprintln!("whitespace line is {}", self.line);
            }
            x if vec![' ', '\r'].contains(&x) => {
                return;
            }
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
                    _ => self.add_token(TokenType::SLASH),
                }
            }
            '=' => {
                let next = self.peek();
                //eprintln!("next is {next}");
                match next {
                    '=' => {
                        self.advance(); // remove next token
                        self.add_token(TokenType::EQUAL_EQUAL);
                    }

                    _ => self.add_token(TokenType::EQUAL),
                }
            }
            '!' => {
                let next = self.peek();
                //eprintln!("next is {next}");
                match next {
                    '=' => {
                        self.advance(); // remove next token
                        self.add_token(TokenType::BANG_EQUAL);
                    }

                    _ => self.add_token(TokenType::BANG),
                }
            }
            '>' => {
                let next = self.peek();
                //eprintln!("next is {next}");
                match next {
                    '=' => {
                        self.advance(); // remove next token
                        self.add_token(TokenType::GREATER_EQUAL);
                    }

                    _ => self.add_token(TokenType::GREATER),
                }
            }
            '<' => {
                let next = self.peek();
                //eprintln!("next is {next}");
                match next {
                    '=' => {
                        self.advance(); // remove next token
                        self.add_token(TokenType::LESS_EQUAL);
                    }

                    _ => self.add_token(TokenType::LESS),
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
