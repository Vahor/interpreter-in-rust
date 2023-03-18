use log::{debug, error, warn};
use thiserror::Error;

use ast::expression::Expression;
use ast::expression::Expression::{IntegerLiteral};
use ast::program::Program;
use ast::statement::{LetStatementData, Statement};
use lexer::lexer::Lexer;
use lexer::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
    pub cur_token: Token,
    pub peek_token: Token,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expected {expected:?}, got {actual:?}")]
    Expected { expected: String, actual: TokenType },

    #[error("Unexpected token {token:?}")]
    UnexpectedToken { token: TokenType },

    #[error("Unknown error")]
    Unknown,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        // Read two tokens so cur_token and peek_token are defined
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program::default();
        while !matches!(&self.cur_token.kind, TokenType::EOF) {
            let stmt = self.parse_statement();
            if stmt.is_ok() {
                let stmt = stmt.unwrap();
                debug!("got statement: {:?}", stmt);
                program.statements.push(stmt);
            } else {
                let err = stmt.err().unwrap_or(ParserError::Unknown);
                error!("got error: {:?}", err);
                return Err(err);
            }
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.cur_token.kind {
            TokenType::LET => self.parse_let_statement(),
            // TokenType::RETURN => self.parse_return_statement(),
            // _ => self.parse_expression_statement(),
            _ => Err(ParserError::UnexpectedToken { token: self.cur_token.kind.clone() }),
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match &self.cur_token.kind {
            TokenType::INT(i) => Some(IntegerLiteral(*i)),
            _ => None
        }
    }

    fn expected_error(&self, expected: String) -> ParserError {
        ParserError::Expected {
            expected: expected.to_string(),
            actual: self.peek_token.kind.clone(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        if !matches!(&self.peek_token.kind, TokenType::IDENT(_)) {
            return Err(self.expected_error("IDENT".to_string()));
        }
        self.next_token(); // (peek) Skip past the LET

        let identifier = match &self.cur_token.kind {
            TokenType::IDENT(ident) => ident,
            _ => unreachable!("Should have been checked above"),
        }.to_string();

        self.next_token(); // (peek) Skip past the IDENT

        if !matches!(self.cur_token.kind, TokenType::ASSIGN) {
            return Err(self.expected_error(TokenType::ASSIGN.to_string()));
        }
        self.next_token(); // (peek) Skip past the ASSIGN

        let value = self.parse_expression();

        if value.is_none() {
            return Err(self.expected_error("Expression".to_string()));
        }
        self.next_token(); // (peek) Skip past the value

        if !matches!(self.cur_token.kind, TokenType::SEMICOLON) {
            return Err(self.expected_error(TokenType::SEMICOLON.to_string()));
        }

        Ok(Statement::LetStatement(LetStatementData {
            identifier,
            value: value.expect("Should have been checked above"),
        }))
    }
}

#[cfg(test)]
mod tests {
    use ast::expression::Expression::IntegerLiteral;
    use lexer::lexer::Lexer;

    use super::*;

    fn asset_let_statement(statement: &Statement, ident: &str, exp: &Expression) {
        match &statement {
            Statement::LetStatement(data) => {
                assert_eq!(data.identifier, ident);
                assert_eq!(data.value, *exp);
            }
            _ => assert!(false, "Expected LetStatement, got {:?}", statement),
        }
    }


    #[test]
    fn test_let_statements_literal() {
        std::env::set_var("RUST_LOG", "trace");

        env_logger::init();

        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();


        assert_eq!(program.statements.len(), 3);

        asset_let_statement(&program.statements[0], "x", &IntegerLiteral(5));
        asset_let_statement(&program.statements[1], "y", &IntegerLiteral(10));
        asset_let_statement(&program.statements[2], "foobar", &IntegerLiteral(838383));
    }
}