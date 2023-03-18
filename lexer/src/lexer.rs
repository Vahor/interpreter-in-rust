use std::fmt::Debug;

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    /// current position in input (points to current char)
    position: usize,
    /// current reading position in input (after current char)
    read_position: usize,
    /// current char under examination
    ch: char,
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
        };

        lexer.next_char();
        return lexer;
    }

    pub fn reset(&mut self, input: String) {
        self.input = input;
        self.position = 0;
        self.read_position = 0;
        self.ch = '\0';
        self.next_char();
    }

    pub fn next_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;

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
            self.next_char();
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
                    Token::new(TokenType::EQ)
                } else {
                    Token::new(TokenType::ASSIGN)
                }
            },
            '+' => Token::new(TokenType::PLUS),
            '-' => Token::new(TokenType::MINUS),
            '!' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::NOT_EQ)
                } else {
                    Token::new(TokenType::BANG)
                }
            },
            '*' => Token::new(TokenType::ASTERISK),
            '/' => Token::new(TokenType::SLASH),
            '<' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::LTE)
                } else {
                    Token::new(TokenType::LT)
                }
            },
            '>' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Token::new(TokenType::GTE)
                } else {
                    Token::new(TokenType::GT)
                }
            },
            ',' => Token::new(TokenType::COMMA),
            ';' => Token::new(TokenType::SEMICOLON),
            '(' => Token::new(TokenType::LPAREN),
            ')' => Token::new(TokenType::RPAREN),
            '{' => Token::new(TokenType::LBRACE),
            '}' => Token::new(TokenType::RBRACE),
            '\0' => Token::new(TokenType::EOF),
            'a'..='z' | 'A'..='Z' | '_' => {
                has_read = true;
                let start = self.position;
                while matches!(self.ch, 'a'..='z' | 'A'..='Z' | '_') {
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

                Token::new(token_type)
            }
            '0'..='9' => {
                has_read = true;
                let start = self.position;
                while matches!(self.ch, '0'..='9') {
                    self.next_char();
                }
                let literal = self.input[start..self.position].to_string();

                Token::new(TokenType::INT(literal.parse::<i64>().unwrap()))
            }
            v => Token::new(TokenType::ILLEGAL(v)),
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
    fn addition() {
        let input = "
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
        ";
        let expected_tokens = vec![
            Token::new(TokenType::LET),
            Token::new(TokenType::IDENT("five".to_string())),
            Token::new(TokenType::ASSIGN),
            Token::new(TokenType::INT(5)),
            Token::new(TokenType::SEMICOLON),

            Token::new(TokenType::LET),
            Token::new(TokenType::IDENT("ten".to_string())),
            Token::new(TokenType::ASSIGN),
            Token::new(TokenType::INT(10)),
            Token::new(TokenType::SEMICOLON),

            Token::new(TokenType::LET),
            Token::new(TokenType::IDENT("add".to_string())),
            Token::new(TokenType::ASSIGN),
            Token::new(TokenType::FUNCTION),
            Token::new(TokenType::LPAREN),
            Token::new(TokenType::IDENT("x".to_string())),
            Token::new(TokenType::COMMA),
            Token::new(TokenType::IDENT("y".to_string())),
            Token::new(TokenType::RPAREN),
            Token::new(TokenType::LBRACE),

            Token::new(TokenType::IDENT("x".to_string())),
            Token::new(TokenType::PLUS),
            Token::new(TokenType::IDENT("y".to_string())),
            Token::new(TokenType::SEMICOLON),

            Token::new(TokenType::RBRACE),
            Token::new(TokenType::SEMICOLON),

            Token::new(TokenType::LET),
            Token::new(TokenType::IDENT("result".to_string())),
            Token::new(TokenType::ASSIGN),
            Token::new(TokenType::IDENT("add".to_string())),
            Token::new(TokenType::LPAREN),
            Token::new(TokenType::IDENT("five".to_string())),
            Token::new(TokenType::COMMA),
            Token::new(TokenType::IDENT("ten".to_string())),
            Token::new(TokenType::RPAREN),
            Token::new(TokenType::SEMICOLON),

            Token::new(TokenType::IF),
            Token::new(TokenType::LPAREN),
            Token::new(TokenType::INT(5)),
            Token::new(TokenType::LT),
            Token::new(TokenType::INT(10)),
            Token::new(TokenType::RPAREN),
            Token::new(TokenType::LBRACE),

            Token::new(TokenType::RETURN),
            Token::new(TokenType::TRUE),
            Token::new(TokenType::SEMICOLON),

            Token::new(TokenType::RBRACE),
            Token::new(TokenType::ELSE),
            Token::new(TokenType::LBRACE),

            Token::new(TokenType::RETURN),
            Token::new(TokenType::FALSE),
            Token::new(TokenType::SEMICOLON),

            Token::new(TokenType::RBRACE),

            Token::new(TokenType::EOF),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            println!("{:?} {:?}", token.kind, expected_token.kind);
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
}