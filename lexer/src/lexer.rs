use std::fmt::Debug;
use log::debug;

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    /// current position in input (points to current char)
    position: usize,
    /// current reading position in input (after current char)
    read_position: usize,
    /// current char under examination
    ch: char,

    /// current line
    line: u32,
    /// current column
    column: u32,
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lexer")
            .field("position", &self.position)
            .field("read_position", &self.read_position)
            .field("ch", &self.ch)
            .finish()
    }
}

impl Default for Lexer {
    fn default() -> Self {
        return Self::new("".to_string());
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 1,
        };

        lexer.next_char();
        lexer.column -= 1;

        return lexer;
    }

    pub fn reset(&mut self, input: String) {
        self.input = input;
        self.position = 0;
        self.read_position = 0;
        self.ch = '\0';
        self.next_char();
        self.line = 1;
        self.column = 1;
    }

    pub fn next_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
        self.column += 1;
        return self.ch;
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.ch, ' ' | '\t' | '\n' | '\r') {
            let is_newline = self.ch == '\n';
            self.next_char();
            if is_newline {
                self.line += 1;
                self.column = 1;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        // Skip whitespace
        self.skip_whitespace();

        let mut has_read = false;

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::EQ, self.line, self.column)
                } else {
                    Token::new(TokenType::ASSIGN, self.line, self.column)
                }
            }
            '+' => Token::new(TokenType::PLUS, self.line, self.column),
            '-' => Token::new(TokenType::MINUS, self.line, self.column),
            '!' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::NOT_EQ, self.line, self.column)
                } else {
                    Token::new(TokenType::BANG, self.line, self.column)
                }
            }
            '*' => Token::new(TokenType::ASTERISK, self.line, self.column),
            '/' => Token::new(TokenType::SLASH, self.line, self.column),
            '<' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::LTE, self.line, self.column)
                } else {
                    Token::new(TokenType::LT, self.line, self.column)
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::GTE, self.line, self.column)
                } else {
                    Token::new(TokenType::GT, self.line, self.column)
                }
            }
            ',' => Token::new(TokenType::COMMA, self.line, self.column),
            ';' => Token::new(TokenType::SEMICOLON, self.line, self.column),
            '(' => Token::new(TokenType::LPAREN, self.line, self.column),
            ')' => Token::new(TokenType::RPAREN, self.line, self.column),
            '{' => Token::new(TokenType::LBRACE, self.line, self.column),
            '}' => Token::new(TokenType::RBRACE, self.line, self.column),
            '\0' => Token::new(TokenType::EOF, self.line, self.column),
            'a'..='z' | 'A'..='Z' | '_' => {
                has_read = true;
                let start = self.position;
                while matches!(self.ch, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9') {
                    self.next_char();
                }
                let literal = self.input[start..self.position].to_string();

                // Handle special keywords
                let token_type = match literal.as_str() {
                    "fn" => TokenType::FUNCTION,
                    "let" => TokenType::LET,
                    "true" => TokenType::TRUE,
                    "false" => TokenType::FALSE,
                    "if" => TokenType::IF,
                    "else" => TokenType::ELSE,
                    "return" => TokenType::RETURN,
                    _ => TokenType::IDENT(literal),
                };

                Token::new(token_type, self.line, self.column)
            }
            '0'..='9' => {
                has_read = true;
                let start = self.position;
                while matches!(self.ch, '0'..='9') {
                    self.next_char();
                }
                let literal = self.input[start..self.position].to_string();

                Token::new(TokenType::INT(literal.parse::<i64>().unwrap()), self.line, self.column)
            }
            '"' => {
                let quote = self.ch;
                debug_assert!(quote == '"');
                has_read = true;
                let start = self.position + 1;
                let mut is_escaped = false;
                while is_escaped || (self.peek_char() != quote && self.peek_char() != '\0') {
                    is_escaped = false;
                    self.next_char();
                    if self.ch == '\\' { // \
                        is_escaped = true;
                    }
                }
                self.next_char();
                let literal = self.input[start..self.position].to_string()
                    // replace escaped characters
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\r", "\r")
                    .replace("\\\\", "\\")
                    .replace("\\\"", "\"")
                    .replace("\\'", "'");
                // if not closed, return illegal token
                if self.ch != '"' {
                    // TODO: return error
                    return Token::new(TokenType::ILLEGAL('"'), self.line, self.column);
                }

                self.next_char();
                Token::new(TokenType::STRING(literal), self.line, self.column)
            }
            v => Token::new(TokenType::ILLEGAL(v), self.line, self.column),
        };

        // Read next char if not literal or number
        if !has_read {
            self.next_char();
        }

        return token;
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn basic_tokens() {
        let input = "=+-*/!<>(){},;==!=<=>=";
        let expected_tokens = vec![
            TokenType::ASSIGN,
            TokenType::PLUS,
            TokenType::MINUS,
            TokenType::ASTERISK,
            TokenType::SLASH,
            TokenType::BANG,
            TokenType::LT,
            TokenType::GT,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::RBRACE,
            TokenType::COMMA,
            TokenType::SEMICOLON,
            TokenType::EQ,
            TokenType::NOT_EQ,
            TokenType::LTE,
            TokenType::GTE,
            TokenType::EOF,
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.kind, expected_token);
        }

        assert_eq!(lexer.next_char(), '\0');
    }

    #[test]
    fn full_list() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;

            "foobar"
            "hello \"world\""
            "hello \n world"
            "hello \t\t\t world"
        "#;

        let expected_tokens = vec![
            Token::with_type(TokenType::LET),
            Token::with_type(TokenType::IDENT("five".to_string())),
            Token::with_type(TokenType::ASSIGN),
            Token::with_type(TokenType::INT(5)),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::LET),
            Token::with_type(TokenType::IDENT("ten".to_string())),
            Token::with_type(TokenType::ASSIGN),
            Token::with_type(TokenType::INT(10)),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::LET),
            Token::with_type(TokenType::IDENT("add".to_string())),
            Token::with_type(TokenType::ASSIGN),
            Token::with_type(TokenType::FUNCTION),
            Token::with_type(TokenType::LPAREN),
            Token::with_type(TokenType::IDENT("x".to_string())),
            Token::with_type(TokenType::COMMA),
            Token::with_type(TokenType::IDENT("y".to_string())),
            Token::with_type(TokenType::RPAREN),
            Token::with_type(TokenType::LBRACE),
//
            Token::with_type(TokenType::IDENT("x".to_string())),
            Token::with_type(TokenType::PLUS),
            Token::with_type(TokenType::IDENT("y".to_string())),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::RBRACE),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::LET),
            Token::with_type(TokenType::IDENT("result".to_string())),
            Token::with_type(TokenType::ASSIGN),
            Token::with_type(TokenType::IDENT("add".to_string())),
            Token::with_type(TokenType::LPAREN),
            Token::with_type(TokenType::IDENT("five".to_string())),
            Token::with_type(TokenType::COMMA),
            Token::with_type(TokenType::IDENT("ten".to_string())),
            Token::with_type(TokenType::RPAREN),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::IF),
            Token::with_type(TokenType::LPAREN),
            Token::with_type(TokenType::INT(5)),
            Token::with_type(TokenType::LT),
            Token::with_type(TokenType::INT(10)),
            Token::with_type(TokenType::RPAREN),
            Token::with_type(TokenType::LBRACE),
//
            Token::with_type(TokenType::RETURN),
            Token::with_type(TokenType::TRUE),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::RBRACE),
            Token::with_type(TokenType::ELSE),
            Token::with_type(TokenType::LBRACE),
//
            Token::with_type(TokenType::RETURN),
            Token::with_type(TokenType::FALSE),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::RBRACE),
//
            Token::with_type(TokenType::INT(10)),
            Token::with_type(TokenType::EQ),
            Token::with_type(TokenType::INT(10)),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::INT(10)),
            Token::with_type(TokenType::NOT_EQ),
            Token::with_type(TokenType::INT(9)),
            Token::with_type(TokenType::SEMICOLON),
//
            Token::with_type(TokenType::STRING("foobar".to_string())),
            Token::with_type(TokenType::STRING("hello \"world\"".to_string())),
            Token::with_type(TokenType::STRING("hello \n world".to_string())),
            Token::with_type(TokenType::STRING("hello \t\t\t world".to_string())),
//
            Token::with_type(TokenType::EOF),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.kind, expected_token.kind);

            match token.kind {
                TokenType::IDENT(literal) => {
                    if let TokenType::IDENT(expected_literal) = expected_token.kind {
                        assert_eq!(literal, expected_literal);
                    } else {
                        panic!("Expected IDENT, got {:?}", expected_token.kind);
                    }
                }
                TokenType::INT(value) => {
                    if let TokenType::INT(expected_value) = expected_token.kind {
                        assert_eq!(value, expected_value);
                    } else {
                        panic!("Expected INT, got {:?}", expected_token.kind);
                    }
                }
                _ => {}
            }
        }

        assert_eq!(lexer.next_char(), '\0');
    }

    #[test]
    fn inline_addition() {
        let input = "5 + 6 * 7 - 8 / 9;";

        let expected_tokens = vec![
            Token::with_type(TokenType::INT(5)),
            Token::with_type(TokenType::PLUS),
            Token::with_type(TokenType::INT(6)),
            Token::with_type(TokenType::ASTERISK),
            Token::with_type(TokenType::INT(7)),
            Token::with_type(TokenType::MINUS),
            Token::with_type(TokenType::INT(8)),
            Token::with_type(TokenType::SLASH),
            Token::with_type(TokenType::INT(9)),
            Token::with_type(TokenType::SEMICOLON),
            Token::with_type(TokenType::EOF),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            println!("{:?} {:?}", token.kind, expected_token.kind);
            assert_eq!(token.kind, expected_token.kind);

            match token.kind {
                TokenType::INT(value) => {
                    if let TokenType::INT(expected_value) = expected_token.kind {
                        assert_eq!(value, expected_value);
                    } else {
                        panic!("Expected INT, got {:?}", expected_token.kind);
                    }
                }
                _ => {}
            }
        }
    }
}