use crate::utils::token::{Token, TokenType};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
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
        let txt = self.source.get(self.start..self.current).unwrap();
        let new_token = Token {
            token_type: token_type,
            lexeme: txt.to_string(),
            literal: "null".to_string(),
            line: 1,
        };

        self.tokens.push(new_token);
    }

    fn advance(&mut self) -> char {
        let source_cloned = self.source.clone();
        let char_found = source_cloned.chars().nth(self.current).unwrap();
        self.current += 1;
        return char_found;
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance();
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

            _ => {
                println!("[line {}] Error: Unexpected character: {}", self.line, c);
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
