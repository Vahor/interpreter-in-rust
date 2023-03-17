use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum TokenType {

    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(i64),

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

pub struct Token {
    pub kind: TokenType,
}

impl Token {
    pub fn new(kind: TokenType) -> Self {
        return Self { kind };
    }

    pub fn to_string(&self) -> String {
        return match &self.kind {
            TokenType::ILLEGAL => "ILLEGAL".to_string(),
            TokenType::EOF => "EOF".to_string(),
            TokenType::IDENT(ident) => ident.to_string(),
            TokenType::INT(int) => int.to_string(),
            TokenType::ASSIGN => "=".to_string(),
            TokenType::PLUS => "+".to_string(),
            TokenType::COMMA => ",".to_string(),
            TokenType::SEMICOLON => ";".to_string(),
            TokenType::LPAREN => "(".to_string(),
            TokenType::RPAREN => ")".to_string(),
            TokenType::LBRACE => "{".to_string(),
            TokenType::RBRACE => "}".to_string(),
            TokenType::FUNCTION => "FUNCTION".to_string(),
            TokenType::LET => "LET".to_string(),
        };
    }
}