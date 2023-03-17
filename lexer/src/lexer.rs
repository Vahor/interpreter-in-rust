use crate::token::{Token, TokenType};

struct Lexer {
    input: String,
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char, // current char under examination
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

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => Token::new(TokenType::ASSIGN),
            '+' => Token::new(TokenType::PLUS),
            ',' => Token::new(TokenType::COMMA),
            ';' => Token::new(TokenType::SEMICOLON),
            '(' => Token::new(TokenType::LPAREN),
            ')' => Token::new(TokenType::RPAREN),
            '{' => Token::new(TokenType::LBRACE),
            '}' => Token::new(TokenType::RBRACE),
            '\0' => Token::new(TokenType::EOF),
            _ => Token::new(TokenType::ILLEGAL),
        };

        self.next_char();

        return token;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn basic_tokens() {
        let input = "=+(){},;";
        let expected_tokens = vec![
            TokenType::ASSIGN,
            TokenType::PLUS,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::RBRACE,
            TokenType::COMMA,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.kind, expected_token);
        }

        assert_eq!(lexer.next_char(), '\0');
    }
}