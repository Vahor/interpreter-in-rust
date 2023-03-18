use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {

    ILLEGAL(char),
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(i64),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,
    LTE,
    GTE,

    EQ,
    NOT_EQ,

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
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            TokenType::ILLEGAL(illegal) => write!(f, "ILLEGAL: {}", illegal),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::IDENT(ident) => write!(f, "IDENT: {}", ident),
            TokenType::INT(int) => write!(f, "INT: {}", int),
            TokenType::ASSIGN => write!(f, "="),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::BANG => write!(f, "!"),
            TokenType::ASTERISK => write!(f, "*"),
            TokenType::SLASH => write!(f, "/"),
            TokenType::LT => write!(f, "<"),
            TokenType::GT => write!(f, ">"),
            TokenType::LTE => write!(f, "<="),
            TokenType::GTE => write!(f, ">="),
            TokenType::EQ => write!(f, "=="),
            TokenType::NOT_EQ => write!(f, "!="),
            TokenType::COMMA => write!(f, ","),
            TokenType::SEMICOLON => write!(f, ";"),
            TokenType::LPAREN => write!(f, "("),
            TokenType::RPAREN => write!(f, ")"),
            TokenType::LBRACE => write!(f, "{{"),
            TokenType::RBRACE => write!(f, "}}"),
            TokenType::FUNCTION => write!(f, "FUNCTION"),
            TokenType::LET => write!(f, "LET"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::FALSE => write!(f, "FALSE"),
            TokenType::IF => write!(f, "IF"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::RETURN => write!(f, "RETURN"),
        };
    }

}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenType,
}

impl Default for Token {
    fn default() -> Self {
        return Self { kind: TokenType::EOF };
    }
}

impl Token {
    pub fn new(kind: TokenType) -> Self {
        return Self { kind };
    }

    pub fn to_string(&self) -> String {
        return format!("{}", self.kind);
    }
}