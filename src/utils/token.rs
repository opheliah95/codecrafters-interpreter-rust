use std::fmt;
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    //one or two character token
    EQUAL,

    // keywords
    VAR,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    //single char token
    SEMICOLON,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    STAR,
    COMMA,
    DOT,
    PLUS,
    MINUS,

    //eof
    EOF,
}

// impl fmt::Display for TokenType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             TokenType::EQUAL => write!(f, "="),
//             TokenType::VAR => write!(f, "var"),
//             TokenType::IDENTIFIER => write!(f, "identifier"),
//             TokenType::STRING => write!(f, "string"),
//             TokenType::NUMBER => write!(f, "number"),
//             TokenType::SEMICOLON => write!(f, ";"),
//             TokenType::EOF => write!(f, "EOF"),
//             TokenType::LEFT_PAREN => write!(f, "("),
//             TokenType::RIGHT_PAREN => write!(f, ")")
//         }
//     }
// }

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: u16,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u16) -> Self {
        return Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line,
        };
    }
    pub fn to_string(&self) -> String {
        let output = format!("{:?} {} {}", self.token_type, self.lexeme, self.literal);
        return output;
    }
}
