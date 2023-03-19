use log::{debug, error, warn};
use thiserror::Error;

use ast::expression::Expression;
use ast::expression::Expression::{BooleanLiteral, IntegerLiteral};
use ast::program::Program;
use ast::statement::{BlockStatement, Statement};
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
    UnexpectedToken { token: Token },

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

    fn expected_error_peek(&self, expected: String) -> ParserError {
        ParserError::Expected {
            expected: expected.to_string(),
            actual: self.peek_token.kind.clone(),
        }
    }
    fn expected_error_curr(&self, expected: String) -> ParserError {
        ParserError::Expected {
            expected: expected.to_string(),
            actual: self.cur_token.kind.clone(),
        }
    }

    fn peek_precedence(&self) -> Precedence {
        self.peek_token.to_precedence()
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut program = Program::default();
        let mut errors = Vec::new();
        while !matches!(&self.cur_token.kind, TokenType::EOF) {
            let stmt = self.parse_statement();
            if stmt.is_ok() {
                let stmt = stmt.unwrap();
                if !matches!(stmt, Statement::EmptyStatement) {
                    program.statements.push(stmt);
                }
            } else {
                let err = stmt.err().unwrap_or(ParserError::Unknown);
                errors.push(err);
            }
            self.next_token();
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(program)
    }



    // Statements

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.cur_token.kind {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            TokenType::SEMICOLON => Ok(Statement::EmptyStatement),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        if !matches!(&self.peek_token.kind, TokenType::IDENT(_)) {
            return Err(self.expected_error_peek("IDENT".to_string()));
        }
        self.next_token(); // (peek) Skip past the LET

        let identifier = self.parse_indent().unwrap().to_string();


        if !matches!(self.peek_token.kind, TokenType::ASSIGN) {
            return Err(self.expected_error_peek(TokenType::ASSIGN.to_string()));
        }
        self.next_token(); // (peek) Skip past the ASSIGN
        self.next_token(); // (curr) Skip past the ASSIGN

        let value = self.parse_expression(&Precedence::LOWEST);
        if value.is_err() {
            return Err(value.err().unwrap());
        }

        if !matches!(self.peek_token.kind, TokenType::SEMICOLON) {
            return Err(self.expected_error_peek(TokenType::SEMICOLON.to_string()));
        }

        Ok(Statement::LetStatement {
            identifier,
            value: value.unwrap(),
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token(); // (peek) Skip past the RETURN

        let value = self.parse_expression(&Precedence::LOWEST);

        if value.is_err() {
            return Err(value.err().unwrap());
        }

        if !matches!(self.peek_token.kind, TokenType::SEMICOLON) {
            return Err(self.expected_error_peek(TokenType::SEMICOLON.to_string()));
        }

        Ok(Statement::ReturnStatement {
            value: value.unwrap(),
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let exp = self.parse_expression(&Precedence::LOWEST);

        if exp.is_err() {
            return Err(exp.err().expect("Should have been checked above"));
        }

        if matches!(self.cur_token.kind, TokenType::SEMICOLON) {
            self.next_token(); // (cur_token) Skip past the SEMICOLON
        }

        Ok(Statement::ExpressionStatement(exp.unwrap()))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        let mut statements: BlockStatement = vec![];

        if matches!(self.cur_token.kind, TokenType::LBRACE) {
            self.next_token(); // (cur_token) Skip past the LBRACE
        }

        while !matches!(self.cur_token.kind, TokenType::RBRACE | TokenType::EOF) {
            let statement = self.parse_statement();

            if statement.is_err() {
                return Err(statement.err().unwrap());
            }

            statements.push(statement.unwrap());
            self.next_token();
        }
        Ok(statements)
    }

    // Expressions

    fn parse_expression(&mut self, precedence: &Precedence) -> Result<Expression, ParserError> {
        let left_expression = match &self.cur_token.kind {
            TokenType::INT(_) => self.parse_int_literal(),
            TokenType::IDENT(_) => self.parse_indent(),
            TokenType::BANG | TokenType::PLUS | TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::TRUE | TokenType::FALSE => self.parse_boolean_literal(),
            TokenType::LPAREN => self.parse_grouped_expression(),
            TokenType::IF => self.parse_if_expression(),
            TokenType::FUNCTION => self.parse_function_literal(),
            _ => None,
        };

        if left_expression.is_none() {
            return Err(ParserError::UnexpectedToken {
                token: self.cur_token.clone(),
            });
        }

        let mut left_expression = left_expression.unwrap();

        while !matches!(&self.peek_token.kind, TokenType::SEMICOLON) && (precedence.value() < self.peek_precedence().value()) {
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

        return Ok(left_expression);
    }

    fn parse_indent(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        if let TokenType::IDENT(value) = token.kind {
            return Some(Expression::Identifier(value));
        }

        None
    }

    fn parse_int_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        if let TokenType::INT(value) = token.kind {
            return Some(IntegerLiteral(value));
        }

        None
    }

    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        return match token.kind {
            TokenType::TRUE | TokenType::FALSE => {
                Some(BooleanLiteral(token.kind == TokenType::TRUE))
            }
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        self.next_token(); // Skip operator

        let right = self.parse_expression(&Precedence::PREFIX);
        if right.is_err() {
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
        if right.is_err() {
            return Some(left);
        }

        Some(Expression::InfixExpression {
            operator: token.kind.to_string(),
            left: Box::new(left),
            right: Box::new(right.unwrap()),
        })
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !matches!(&self.peek_token.kind, TokenType::LPAREN) {
            return None;
        }
        self.next_token(); // (peek) Skip past the LPAREN

        self.next_token(); // (curr) Skip past the LPAREN
        let condition = self.parse_expression(&Precedence::LOWEST);
        if condition.is_err() {
            return None;
        }


        if !matches!(&self.peek_token.kind, TokenType::RPAREN) {
            return None;
        }
        self.next_token(); // (peek) Skip past the RPAREN

        if !matches!(&self.peek_token.kind, TokenType::LBRACE) {
            return None;
        }
        self.next_token(); // (peek) Skip past the LBRACE

        let consequence = self.parse_block_statement();

        if consequence.is_err() {
            return None;
        }

        let mut alternative = None;
        if matches!(&self.peek_token.kind, TokenType::ELSE) {
            self.next_token(); // (peek) Skip past the ELSE

            if !matches!(&self.peek_token.kind, TokenType::LBRACE) {
                return None;
            }
            self.next_token(); // (peek) Skip past the LBRACE

            if let Ok(stmt) = self.parse_block_statement() {
                alternative = Some(stmt);
            } else {
                return None;
            }

        }

        Some(Expression::IfExpression {
            condition: Box::new(condition.unwrap()),
            consequence: consequence.unwrap(),
            alternative,
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token(); // (peek) Skip past the LPAREN
        let expression = self.parse_expression(&Precedence::LOWEST);
        if !expression.is_ok() {
            return None;
        }

        if !matches!(&self.peek_token.kind, TokenType::RPAREN) {
            return None;
        }
        self.next_token(); // (peek) Skip past the RPAREN

        expression.ok()
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        if !matches!(&self.peek_token.kind, TokenType::LPAREN) {
            return None;
        }
        self.next_token(); // (peek) Skip past the LPAREN

        let parameters = self.parse_function_parameters();
        if parameters.is_err() {
            return None;
        }
        let parameters = parameters.unwrap();

        if !matches!(&self.peek_token.kind, TokenType::LBRACE) {
            return None;
        }
        self.next_token(); // (peek) Skip past the LBRACE

        let body = self.parse_block_statement();
        if body.is_err() {
            return None;
        }

        Some(Expression::FunctionLiteral {
            parameters,
            body: body.unwrap(),
        })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut identifiers = Vec::new();

        // fn ()
        if matches!(&self.peek_token.kind, TokenType::RPAREN) {
            self.next_token(); // (peek) Skip past the RPAREN
            return Ok(identifiers);
        }

        self.next_token(); // (peek) Skip past the first identifier
        if let TokenType::IDENT(ident) = &self.cur_token.kind {
            identifiers.push(Expression::Identifier(ident.clone()));
        } else {
            return Err(self.expected_error_curr("identifier".to_string()));
        }

        while matches!(&self.peek_token.kind, TokenType::COMMA) {
            self.next_token(); // (peek) Skip past the COMMA
            self.next_token(); // (peek) Skip past the next identifier

            if let TokenType::IDENT(ident) = &self.cur_token.kind {
                identifiers.push(Expression::Identifier(ident.clone()));
            } else {
                return Err(self.expected_error_curr("identifier".to_string()));
            }
        }

        if !matches!(&self.peek_token.kind, TokenType::RPAREN) {
            return Err(self.expected_error_peek(")".to_string()));
        }
        self.next_token(); // (peek) Skip past the RPAREN

        Ok(identifiers)
    }
}

#[cfg(test)]
mod tests {
    use ast::expression::Expression::{Identifier, InfixExpression, IntegerLiteral, PrefixExpression};
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
            Statement::ReturnStatement { value } => {
                assert_eq!(value, exp);
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
                    InfixExpression { operator, left: l, right: r } => {
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

    fn prefix_expression(operator: String, right: Expression) -> Expression {
        return PrefixExpression {
            operator,
            right: Box::new(right),
        };
    }

    fn infix_expression(left: Expression, operator: String, right: Expression) -> Expression {
        return InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }


    #[test]
    fn test_let_statements_literal() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        // let name = "John";
        let input = r#"
        let age = 30;
        let isMale = true;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 2);

        asset_let_statement(&program.statements[0], "age", &IntegerLiteral(30));
        asset_let_statement(&program.statements[1], "isMale", &BooleanLiteral(true));
        // asset_let_statement(&program.statements[2], "name", &Expression::StringLiteral("John".to_string()));
    }

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

    #[test]
    fn test_prefix_expressions() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let input = r#"
        !5;
        -15;
        !-a;
        !true;
        !false;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 5);

        asset_prefix_expression(&program.statements[0], "!", &IntegerLiteral(5));
        asset_prefix_expression(&program.statements[1], "-", &IntegerLiteral(15));
        asset_prefix_expression(&program.statements[2], "!", &prefix_expression("-".to_string(), Identifier("a".to_string())));
        asset_prefix_expression(&program.statements[3], "!", &BooleanLiteral(true));
        asset_prefix_expression(&program.statements[4], "!", &BooleanLiteral(false));
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

        3 > 5 == false;
        3 < 5 == true;
        true != false;

        1 + (2 + 3) + 4;
        (5 + 5) * 2;
        2 / (5 + 5);
        -(5 + 5);
        !(true == true);
        "#;


        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        // program.to_string().lines().collect::<Vec<&str>>().iter().enumerate().for_each(|(i, line)| {
        //     warn!("{}: {}", i, line);
        // });

        assert_eq!(program.statements.len(), 28);

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

        assert_eq!(&program.statements[20].to_string(), "((3 > 5) == false);");
        assert_eq!(&program.statements[21].to_string(), "((3 < 5) == true);");
        assert_eq!(&program.statements[22].to_string(), "(true != false);");

        assert_eq!(&program.statements[23].to_string(), "((1 + (2 + 3)) + 4);");
        assert_eq!(&program.statements[24].to_string(), "((5 + 5) * 2);");
        assert_eq!(&program.statements[25].to_string(), "(2 / (5 + 5));");
        assert_eq!(&program.statements[26].to_string(), "(-(5 + 5));");
        assert_eq!(&program.statements[27].to_string(), "(!(true == true));");
    }

    #[test]
    fn test_if_expression() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        // if (x < y) { x }
        // if (x < 3 * y) { x + 1; } else { y }
        // if (x < 2 * y) { x + 1; }
        let input = r#"
if (x < y) { x }
if (x < 2 * y) { x + 1; }
if (x < 3 * y) { x + 1; } else { y }
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();


        assert_eq!(program.statements.len(), 3);

        assert_eq!(&program.statements[0].to_string(), "if (x < y) { x; }");
        assert_eq!(&program.statements[1].to_string(), "if (x < (2 * y)) { (x + 1); }");
        assert_eq!(&program.statements[2].to_string(), "if (x < (3 * y)) { (x + 1); } else { y; }");
    }

    #[test]
    fn test_function_literal() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        // fn(x, y) { x + y; }
        let input = r#"
fn() { x + y; }
fn(x, y) { x + y; }
"#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        debug!("{:?}", program);

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 2);

        assert_eq!(&program.statements[0].to_string(), "fn() { (x + y); }");
        assert_eq!(&program.statements[1].to_string(), "fn(x, y) { (x + y); }");
    }
}