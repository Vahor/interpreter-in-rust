#[derive(Debug, PartialEq)]
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

pub struct Token {
    pub kind: TokenType,
}

impl Token {
    pub fn new(kind: TokenType) -> Self {
        return Self { kind };
    }

    pub fn to_string(&self) -> String {
        return match &self.kind {
            TokenType::ILLEGAL(illegal) => "ILLEGAL: (".to_string() + illegal.to_string().as_str() + ")",
            TokenType::EOF => "EOF".to_string(),
            TokenType::IDENT(ident) => ident.to_string(),
            TokenType::INT(int) => int.to_string(),
            TokenType::ASSIGN => "=".to_string(),
            TokenType::PLUS => "+".to_string(),
            TokenType::MINUS => "-".to_string(),
            TokenType::BANG => "!".to_string(),
            TokenType::ASTERISK => "*".to_string(),
            TokenType::SLASH => "/".to_string(),
            TokenType::LT => "<".to_string(),
            TokenType::GT => ">".to_string(),
            TokenType::LTE => "<=".to_string(),
            TokenType::GTE => ">=".to_string(),
            TokenType::EQ => "==".to_string(),
            TokenType::NOT_EQ => "!=".to_string(),
            TokenType::COMMA => ",".to_string(),
            TokenType::SEMICOLON => ";".to_string(),
            TokenType::LPAREN => "(".to_string(),
            TokenType::RPAREN => ")".to_string(),
            TokenType::LBRACE => "{".to_string(),
            TokenType::RBRACE => "}".to_string(),
            TokenType::FUNCTION => "FUNCTION".to_string(),
            TokenType::LET => "LET".to_string(),
            TokenType::TRUE => "TRUE".to_string(),
            TokenType::FALSE => "FALSE".to_string(),
            TokenType::IF => "IF".to_string(),
            TokenType::ELSE => "ELSE".to_string(),
            TokenType::RETURN => "RETURN".to_string(),
        };
    }
}