use log::{debug, error, warn};
use thiserror::Error;

use ast::expression::Expression;
use ast::expression::Expression::IntegerLiteral;
use ast::program::Program;
use ast::statement::{ReturnStatementData, Statement};
use lexer::lexer::Lexer;
use lexer::precedence::Precedence;
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

    fn expected_error(&self, expected: String) -> ParserError {
        ParserError::Expected {
            expected: expected.to_string(),
            actual: self.peek_token.kind.clone(),
        }
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
                program.statements.push(stmt);
            } else {
                let err = stmt.err().unwrap_or(ParserError::Unknown);
                return Err(err);
            }
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.cur_token.kind {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression(&mut self, precedence: &Precedence) -> Option<Expression> {
        let left_expression = match &self.cur_token.kind {
            TokenType::INT(i) => Some(IntegerLiteral(*i)),
            TokenType::IDENT(ident) => Some(Expression::Identifier(ident.to_string())),
            TokenType::BANG | TokenType::PLUS | TokenType::MINUS => self.parse_prefix_expression(),
            _ => None
        };

        if left_expression.is_none() {
            return None;
        }

        let mut left_expression = left_expression.unwrap();
        while !matches!(&self.peek_token.kind, TokenType::SEMICOLON) && (precedence.get_precedence() < self.peek_token.to_precedence().get_precedence()) {
            // Infix match
            match &self.peek_token.kind {
                TokenType::PLUS | TokenType::MINUS | TokenType::SLASH | TokenType::ASTERISK | TokenType::EQ | TokenType::NOT_EQ | TokenType::LT | TokenType::GT => {
                    self.next_token();
                    let right_expression = self.parse_infix_expression(left_expression.clone());
                    if right_expression.is_none() {
                        break;
                    }

                    left_expression = right_expression.unwrap();
                }
                _ => break,
            };
        }

        return Some(left_expression);
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        self.next_token();
        let right = self.parse_expression(&Precedence::PREFIX);
        if right.is_none() {
            return None;
        }

        Some(Expression::PrefixExpression {
            operator: token.kind.to_string(),
            right: Box::new(right.unwrap()),
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.cur_token.clone();

        let precedence = self.cur_token.to_precedence();
        self.next_token();

        let right = self.parse_expression(&precedence);
        if right.is_none() {
            return Some(left);
        }

        Some(Expression::InfixExpression {
            operator: token.kind.to_string(),
            left: Box::new(left),
            right: Box::new(right.unwrap()),
        })
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

        let value = self.parse_expression(&Precedence::LOWEST);

        if value.is_none() {
            return Err(self.expected_error("Expression".to_string()));
        }
        self.next_token(); // (peek) Skip past the value

        if !matches!(self.cur_token.kind, TokenType::SEMICOLON) {
            return Err(self.expected_error(TokenType::SEMICOLON.to_string()));
        }

        Ok(Statement::LetStatement {
            identifier,
            value: value.expect("Should have been checked above"),
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token(); // (peek) Skip past the RETURN

        let value = self.parse_expression(&Precedence::LOWEST);

        if value.is_none() {
            return Err(self.expected_error("Expression".to_string()));
        }
        self.next_token(); // (peek) Skip past the value

        if !matches!(self.cur_token.kind, TokenType::SEMICOLON) {
            return Err(self.expected_error(TokenType::SEMICOLON.to_string()));
        }

        Ok(Statement::ReturnStatement(ReturnStatementData {
            value: value.expect("Should have been checked above"),
        }))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let exp = self.parse_expression(&Precedence::LOWEST);

        if exp.is_none() {
            return Err(self.expected_error("Expression".to_string()));
        }
        self.next_token(); // (peek) Skip past the value

        if matches!(self.peek_token.kind, TokenType::SEMICOLON) {
            self.next_token(); // (peek) Skip past the semicolon
        }

        Ok(Statement::ExpressionStatement(exp.expect("Should have been checked above")))
    }

}

#[cfg(test)]
mod tests {
    use ast::expression::Expression::{Identifier, IntegerLiteral, PrefixExpression};
    use ast::expression::{infix_expression, prefix_expression};
    use lexer::lexer::Lexer;

    use super::*;

    fn asset_let_statement(statement: &Statement, ident: &str, exp: &Expression) {
        match &statement {
            Statement::LetStatement { identifier, value } => {
                assert_eq!(identifier, ident);
                assert_eq!(value, exp);
            }
            _ => assert!(false, "Expected LetStatement, got {:?}", statement),
        }
    }

    fn asset_return_statement(statement: &Statement, exp: &Expression) {
        match &statement {
            Statement::ReturnStatement(data) => {
                assert_eq!(data.value, *exp);
            }
            _ => assert!(false, "Expected ReturnStatement, got {:?}", statement),
        }
    }

    fn asset_expression_statement(statement: &Statement, exp: &Expression) {
        match statement {
            Statement::ExpressionStatement(data) => {
                assert_eq!(data, exp);
            }
            _ => assert!(false, "Expected ExpressionStatement, got {:?}", statement),
        }
    }

    fn asset_prefix_expression(statement: &Statement, op: &str, exp: &Expression) {
        match statement {
            Statement::ExpressionStatement(data) => {
                match data {
                    PrefixExpression { operator, right } => {
                        assert_eq!(operator, op);
                        assert_eq!(right.as_ref(), exp);
                    }
                    _ => assert!(false, "Expected PrefixExpression, got {:?}", data),
                }
            }
            _ => assert!(false, "Expected ExpressionStatement, got {:?}", statement),
        }
    }

    fn assert_infix_expression(statement: &Statement, left: &Expression, op: &str, right: &Expression) {
        match statement {
            Statement::ExpressionStatement(data) => {
                match data {
                    Expression::InfixExpression { operator, left: l, right: r } => {
                        assert_eq!(operator, op);
                        assert_eq!(l.as_ref(), left);
                        assert_eq!(r.as_ref(), right);
                    }
                    _ => assert!(false, "Expected InfixExpression {:?}, got {:?}", statement.to_string(), data.to_string()),
                }
            }
            _ => assert!(false, "Expected ExpressionStatement, got {:?}", statement),
        }
    }


    #[ignore]
    #[test]
    fn test_let_statements_literal() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

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

    #[ignore]
    #[test]
    fn test_return_statements() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 3);

        asset_return_statement(&program.statements[0], &IntegerLiteral(5));
        asset_return_statement(&program.statements[1], &IntegerLiteral(10));
        asset_return_statement(&program.statements[2], &IntegerLiteral(993322));
    }

    #[ignore]
    #[test]
    fn test_identifier_expression() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let input = r#"
        foobar;
        5;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 2);

        asset_expression_statement(&program.statements[0], &Identifier("foobar".to_string()));
        asset_expression_statement(&program.statements[1], &IntegerLiteral(5));
    }


    #[ignore]
    #[test]
    fn test_prefix_expressions() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let input = r#"
        !5;
        -15;
        !-a;
        "#;
        // !true;
        // !false;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 3);

        asset_prefix_expression(&program.statements[0], "!", &IntegerLiteral(5));
        asset_prefix_expression(&program.statements[1], "-", &IntegerLiteral(15));
        asset_prefix_expression(&program.statements[2], "!", &prefix_expression("-".to_string(), Identifier("a".to_string())));
    }

    #[test]
    fn test_infix_expressions() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();


        let input = r#"
        5 + 5;
        5 - 5;
        5 * 5;
        5 / 5;
        5 > 5;
        5 < 5;
        5 == 5;
        5 != 5;

        -a * b;
        a + b + c;
        a + b - c;
        a * b * c;
        a * b / c;
        a + b / c;
        a + b * c + d / e - f;
        3 + 4; -5 * 5;
        5 > 4 == 3 < 4;
        5 < 4 != 3 > 4;
        3 + 4 * 5 == 3 * 1 + 4 * 5;
        "#;


        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        program.to_string().lines().collect::<Vec<&str>>().iter().enumerate().for_each(|(i, line)| {
            warn!("{}: {}", i, line);
        });

        assert_eq!(program.statements.len(), 20);

        assert_infix_expression(&program.statements[0], &IntegerLiteral(5), "+", &IntegerLiteral(5));
        assert_infix_expression(&program.statements[1], &IntegerLiteral(5), "-", &IntegerLiteral(5));
        assert_infix_expression(&program.statements[2], &IntegerLiteral(5), "*", &IntegerLiteral(5));
        assert_infix_expression(&program.statements[3], &IntegerLiteral(5), "/", &IntegerLiteral(5));
        assert_infix_expression(&program.statements[4], &IntegerLiteral(5), ">", &IntegerLiteral(5));
        assert_infix_expression(&program.statements[5], &IntegerLiteral(5), "<", &IntegerLiteral(5));
        assert_infix_expression(&program.statements[6], &IntegerLiteral(5), "==", &IntegerLiteral(5));
        assert_infix_expression(&program.statements[7], &IntegerLiteral(5), "!=", &IntegerLiteral(5));
        assert_infix_expression(&program.statements[8], &prefix_expression("-".to_string(), Identifier("a".to_string())), "*", &Identifier("b".to_string()));
        assert_infix_expression(&program.statements[9], &infix_expression(Identifier("a".to_string()), "+".to_string(), Identifier("b".to_string())), "+", &Identifier("c".to_string()));
        assert_infix_expression(&program.statements[10], &infix_expression(Identifier("a".to_string()), "+".to_string(), Identifier("b".to_string())), "-", &Identifier("c".to_string()));
        assert_infix_expression(&program.statements[11], &infix_expression(Identifier("a".to_string()), "*".to_string(), Identifier("b".to_string())), "*", &Identifier("c".to_string()));
        assert_infix_expression(&program.statements[12], &infix_expression(Identifier("a".to_string()), "*".to_string(), Identifier("b".to_string())), "/", &Identifier("c".to_string()));
        assert_eq!(&program.statements[13].to_string(), "(a + (b / c));");
        assert_eq!(&program.statements[14].to_string(), "(((a + (b * c)) + (d / e)) - f);");
        assert_eq!(&program.statements[15].to_string(), "(3 + 4);");
        assert_eq!(&program.statements[16].to_string(), "((-5) * 5);");
        assert_eq!(&program.statements[17].to_string(), "((5 > 4) == (3 < 4));");
        assert_eq!(&program.statements[18].to_string(), "((5 < 4) != (3 > 4));");
        assert_eq!(&program.statements[19].to_string(), "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));");

    }
}