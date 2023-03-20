use std::sync::atomic::Ordering;
use log::{debug, warn};
use ast::expression::Expression;
use ast::expression::Expression::{BooleanLiteral, IntegerLiteral, StringLiteral};
use ast::program::Program;
use ast::statement::{BlockStatement, Statement};
use error::EvaluatorError;
use lexer::lexer::Lexer;
use lexer::precedence::Precedence;
use lexer::token::{Token, TokenType};
use flags::STOP_AT_FIRST_ERROR;

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
    pub cur_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        // Read two tokens so cur_token and peek_token are defined
        parser.next_token().expect("Failed to read first token");
        parser.next_token().expect("Failed to read second token");

        parser
    }

    pub fn reset(&mut self, input: String) {
        self.lexer.reset(input);
        self.next_token().expect("Failed to read first token");
        self.next_token().expect("Failed to read second token");
    }

    fn expected_error_peek(&self, expected: String) -> EvaluatorError {
        EvaluatorError::expected_token(expected.to_string().as_str(),
                                       self.peek_token.kind.clone().to_string().as_str(),
                                       self.peek_token.line,
                                       self.peek_token.column)
    }
    fn expected_error_curr(&self, expected: String) -> EvaluatorError {
        EvaluatorError::expected_token(expected.to_string().as_str(),
                                       self.cur_token.kind.clone().to_string().as_str(),
                                       self.cur_token.line,
                                       self.cur_token.column)
    }

    fn peek_precedence(&self) -> Precedence {
        self.peek_token.to_precedence()
    }

    pub fn next_token(&mut self) -> Result<(), EvaluatorError> {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<EvaluatorError>> {
        let stop_first = STOP_AT_FIRST_ERROR.load(Ordering::Relaxed);

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
                let err = stmt.err().unwrap_or(EvaluatorError::unknown_error());
                errors.push(err);
                if stop_first {
                    return Err(errors);
                }
            }

            let next = self.next_token();
            if next.is_err() {
                let err = next.err().unwrap_or(EvaluatorError::unknown_error());
                errors.push(err);
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(program)
    }


    // Statements

    fn parse_statement(&mut self) -> Result<Statement, EvaluatorError> {
        match &self.cur_token.kind {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            TokenType::SEMICOLON => Ok(Statement::EmptyStatement),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, EvaluatorError> {
        if !matches!(&self.peek_token.kind, TokenType::IDENT(_)) {
            return Err(self.expected_error_peek("IDENT".to_string()));
        }
        self.next_token()?; // (peek) Skip past the LET

        let identifier = self.parse_indent().unwrap().to_string();


        if !matches!(self.peek_token.kind, TokenType::ASSIGN) {
            return Err(self.expected_error_peek(TokenType::ASSIGN.to_string()));
        }
        self.next_token()?; // (peek) Skip past the ASSIGN
        self.next_token()?; // (curr) Skip past the ASSIGN

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

    fn parse_return_statement(&mut self) -> Result<Statement, EvaluatorError> {
        self.next_token()?; // (peek) Skip past the RETURN

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

    fn parse_expression_statement(&mut self) -> Result<Statement, EvaluatorError> {
        let exp = self.parse_expression(&Precedence::LOWEST);

        if exp.is_err() {
            return Err(exp.err().expect("Should have been checked above"));
        }

        if matches!(self.cur_token.kind, TokenType::SEMICOLON) {
            self.next_token()?; // (cur_token) Skip past the SEMICOLON
        }

        Ok(Statement::ExpressionStatement(exp.unwrap()))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, EvaluatorError> {
        let mut statements: BlockStatement = vec![];

        self.next_token()?; // (cur_token) Skip past the LBRACE

        while !matches!(self.cur_token.kind, TokenType::RBRACE | TokenType::EOF) {
            let statement = self.parse_statement()?;

            if !matches!(statement, Statement::EmptyStatement) {
                statements.push(statement);
            }

            self.next_token()?;
        }

        Ok(statements)
    }

    // Expressions

    fn parse_expression(&mut self, precedence: &Precedence) -> Result<Expression, EvaluatorError> {
        let left_expression = match &self.cur_token.kind {
            TokenType::INT(_) => self.parse_int_literal(),
            TokenType::STRING(_) => self.parse_string_literal(),
            TokenType::IDENT(_) => self.parse_indent(),
            TokenType::BANG | TokenType::PLUS | TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::TRUE | TokenType::FALSE => self.parse_boolean_literal(),
            TokenType::LPAREN => self.parse_grouped_expression(),
            TokenType::IF => self.parse_if_expression(),
            TokenType::FUNCTION => self.parse_function_literal(),
            TokenType::LBRACKET => self.parse_array_literal(),
            _ => Err(self.expected_error_curr("Expression".to_string())),
        };

        if left_expression.is_err() {
            return Err(left_expression.err().unwrap());
        }

        let mut left_expression = left_expression.unwrap();

        while !matches!(&self.peek_token.kind, TokenType::SEMICOLON) && (precedence.value() < self.peek_precedence().value()) {
            // Infix match
            match &self.peek_token.kind {
                TokenType::PLUS | TokenType::MINUS | TokenType::SLASH | TokenType::ASTERISK | TokenType::EQ | TokenType::NOT_EQ | TokenType::LT | TokenType::GT | TokenType::LTE | TokenType::GTE => {
                    self.next_token()?;
                    let right_expression = self.parse_infix_expression(left_expression.clone());
                    if right_expression.is_err() {
                        return Err(right_expression.err().unwrap());
                    }

                    left_expression = right_expression.unwrap();
                }
                TokenType::LPAREN => {
                    self.next_token()?;
                    let right_expression = self.parse_call_expression(left_expression.clone())?;
                    left_expression = right_expression;
                },
                TokenType::LBRACKET => {
                    self.next_token()?;
                    let right_expression = self.parse_index_expression(left_expression.clone())?;
                    left_expression = right_expression;
                }
                _ => break,
            };
        }

        return Ok(left_expression);
    }

    fn parse_indent(&mut self) -> Result<Expression, EvaluatorError> {
        let token = self.cur_token.clone();
        if let TokenType::IDENT(value) = token.kind {
            return Ok(Expression::Identifier(value));
        }

        Err(self.expected_error_curr("IDENT".to_string()))
    }

    fn parse_int_literal(&mut self) -> Result<Expression, EvaluatorError> {
        let token = &self.cur_token;
        if let TokenType::INT(value) = token.kind {
            return Ok(IntegerLiteral(value));
        }

        Err(self.expected_error_curr("INT".to_string()))
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, EvaluatorError> {
        let operator = self.cur_token.kind.to_string();

        self.next_token()?; // Skip operator
        let right = self.parse_expression(&Precedence::PREFIX)?;

        Ok(Expression::PrefixExpression {
            operator,
            right: Box::new(right),
        })
    }

    fn parse_boolean_literal(&mut self) -> Result<Expression, EvaluatorError> {
        let token = &self.cur_token;
        return match token.kind {
            TokenType::TRUE | TokenType::FALSE => {
                Ok(BooleanLiteral(token.kind == TokenType::TRUE))
            }
            _ => Err(self.expected_error_curr("BOOLEAN".to_string())),
        };
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, EvaluatorError> {
        self.next_token()?; // (peek) Skip past the LPAREN
        let expression = self.parse_expression(&Precedence::LOWEST)?;

        if !matches!(&self.peek_token.kind, TokenType::RPAREN) {
            return Err(self.expected_error_peek(")".to_string()));
        }
        self.next_token()?; // (peek) Skip past the RPAREN

        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, EvaluatorError> {
        if !matches!(&self.peek_token.kind, TokenType::LPAREN) {
            return Err(self.expected_error_peek("(".to_string()));
        }
        self.next_token()?; // (peek) Skip past the LPAREN

        self.next_token()?; // (curr) Skip past the LPAREN
        let condition = self.parse_expression(&Precedence::LOWEST)?;


        if !matches!(&self.peek_token.kind, TokenType::RPAREN) {
            return Err(self.expected_error_peek(")".to_string()));
        }
        self.next_token()?; // (peek) Skip past the RPAREN

        if !matches!(&self.peek_token.kind, TokenType::LBRACE) {
            return Err(self.expected_error_peek("{".to_string()));
        }
        self.next_token()?; // (peek) Skip past the LBRACE

        let consequence = self.parse_block_statement()?;

        let mut alternative = None;
        if matches!(&self.peek_token.kind, TokenType::ELSE) {
            self.next_token()?; // (peek) Skip past the ELSE

            if !matches!(&self.peek_token.kind, TokenType::LBRACE) {
                return Err(self.expected_error_peek("{".to_string()));
            }
            self.next_token()?; // (peek) Skip past the LBRACE

            let block = self.parse_block_statement()?;
            alternative = Some(block);
        }

        Ok(Expression::IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, EvaluatorError> {
        let operator = self.cur_token.kind.to_string();

        let precedence = self.cur_token.to_precedence();
        self.next_token()?;

        let right = self.parse_expression(&precedence);
        if right.is_err() {
            debug!("Error parsing right expression: {:?}", right.err().unwrap());
            return Ok(left);
        }

        Ok(Expression::InfixExpression {
            operator,
            left: Box::new(left),
            right: Box::new(right.unwrap()),
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, EvaluatorError> {
        let arguments = self.parse_call_arguments()?;

        Ok(Expression::CallExpression {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, EvaluatorError> {
        // fn ()
        if matches!(&self.peek_token.kind, TokenType::RPAREN) {
            self.next_token()?; // (peek) Skip past the RPAREN
            return Ok(vec![]);
        }

        let arguments = self.parse_expression_list(&TokenType::RPAREN);

        return arguments;
    }

    fn parse_function_literal(&mut self) -> Result<Expression, EvaluatorError> {
        if !matches!(&self.peek_token.kind, TokenType::LPAREN) {
            return Err(self.expected_error_peek("(".to_string()));
        }
        self.next_token()?; // (peek) Skip past the LPAREN

        let parameters = self.parse_function_parameters()?;

        if !matches!(&self.peek_token.kind, TokenType::LBRACE) {
            return Err(self.expected_error_peek("{".to_string()));
        }
        self.next_token()?; // (peek) Skip past the LBRACE

        let body = self.parse_block_statement()?;

        Ok(Expression::FunctionLiteral {
            parameters,
            body,
        })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Expression>, EvaluatorError> {
        let mut identifiers = Vec::new();

        // fn ()
        if matches!(&self.peek_token.kind, TokenType::RPAREN) {
            self.next_token()?; // (peek) Skip past the RPAREN
            return Ok(identifiers);
        }

        self.next_token()?; // (peek) Skip past the first identifier
        if let TokenType::IDENT(ident) = &self.cur_token.kind {
            identifiers.push(Expression::Identifier(ident.clone()));
        } else {
            return Err(self.expected_error_curr("identifier".to_string()));
        }

        while matches!(&self.peek_token.kind, TokenType::COMMA) {
            self.next_token()?; // (peek) Skip past the COMMA
            self.next_token()?; // (peek) Skip past the next identifier

            if let TokenType::IDENT(ident) = &self.cur_token.kind {
                identifiers.push(Expression::Identifier(ident.clone()));
            } else {
                return Err(self.expected_error_curr("identifier".to_string()));
            }
        }

        if !matches!(&self.peek_token.kind, TokenType::RPAREN) {
            return Err(self.expected_error_peek(")".to_string()));
        }
        self.next_token()?; // (peek) Skip past the RPAREN

        Ok(identifiers)
    }

    fn parse_string_literal(&mut self) -> Result<Expression, EvaluatorError> {
        let token = self.cur_token.clone();
        if let TokenType::STRING(value) = token.kind {
            return Ok(StringLiteral(value));
        }

        Err(self.expected_error_curr("STRING".to_string()))
    }

    fn parse_array_literal(&mut self) -> Result<Expression, EvaluatorError> {
        let elements = self.parse_expression_list(&TokenType::RBRACKET)?;

        Ok(Expression::ArrayLiteral(elements))
    }

    fn parse_expression_list(&mut self, end: &TokenType) -> Result<Vec<Expression>, EvaluatorError> {
        let mut elements: Vec<Expression> = vec![];

        if &self.peek_token.kind == end {
            self.next_token()?; // (peek) Skip past the end token
            return Ok(elements);
        }

        self.next_token()?; // (cur_token) Skip past the end token
        elements.push(self.parse_expression(&Precedence::LOWEST)?);

        while matches!(self.peek_token.kind, TokenType::COMMA) {
            self.next_token()?; // (peek_token) Skip past the COMMA
            self.next_token()?; // (cur_token) Skip past the COMMA

            elements.push(self.parse_expression(&Precedence::LOWEST)?);
        }

        if &self.peek_token.kind != end {
            return Err(self.expected_error_peek(TokenType::RBRACKET.to_string()));
        }
        self.next_token()?; // (peek) Skip past the RBRACKET

        Ok(elements)
    }

    fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, EvaluatorError> {
        self.next_token()?; // (peek) Skip past the LBRACKET
        let index = self.parse_expression(&Precedence::LOWEST)?;

        if !matches!(&self.peek_token.kind, TokenType::RBRACKET) {
            return Err(self.expected_error_peek("]".to_string()));
        }
        self.next_token()?; // (peek) Skip past the LBRACKET

        Ok(Expression::IndexExpression {
            left: Box::new(left),
            index: Box::new(index),
        })
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

        let input = r#"
        let name = "John";
        let age = 30;
        let isMale = true;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 3);

        asset_let_statement(&program.statements[0], "name", &StringLiteral("John".to_string()));
        asset_let_statement(&program.statements[1], "age", &IntegerLiteral(30));
        asset_let_statement(&program.statements[2], "isMale", &BooleanLiteral(true));
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

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 2);

        assert_eq!(&program.statements[0].to_string(), "fn() { (x + y); }");
        assert_eq!(&program.statements[1].to_string(), "fn(x, y) { (x + y); }");
    }

    #[test]
    fn test_function_call() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        // add(1, 2 * 3, 4 + 5);
        let input = r#"
add(1, 2 * 3, 4 + 5);
a + add(b * c) + d;
add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8));
add(a + b + c * d / f + g);
"#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 4);

        assert_eq!(&program.statements[0].to_string(), "add(1, (2 * 3), (4 + 5));");
        assert_eq!(&program.statements[1].to_string(), "((a + add((b * c))) + d);");
        assert_eq!(&program.statements[2].to_string(), "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)));");
        assert_eq!(&program.statements[3].to_string(), "add((((a + b) + ((c * d) / f)) + g));");
    }

    #[test]
    fn test_array_literal() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        // [1];
        // [1, 2 * 2, 3 + 3];
        let input = r#"
        [1];
        [1, 2 * 2, 3 + 3];
"#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 2);

        assert_eq!(&program.statements[0].to_string(), "[1];");
        assert_eq!(&program.statements[1].to_string(), "[1, (2 * 2), (3 + 3)];");
    }

    #[test]
    fn test_array_index() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        // [1];
        // [1, 2 * 2, 3 + 3];
        let input = r#"
        myArray[1 + 1];
"#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 1);

        assert_eq!(&program.statements[0].to_string(), "(myArray[(1 + 1)]);");
    }

    #[test]
    fn test_precedence() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let input = r#"
        a * [1, 2, 3, 4][b * c] * d;
        add(a * b[2], b[1], 2 * [1, 2][1]);
"#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_ok());

        let program = program.unwrap();

        assert_eq!(program.statements.len(), 2);

        assert_eq!(&program.statements[0].to_string(), "((a * ([1, 2, 3, 4][(b * c)])) * d);");
        assert_eq!(&program.statements[1].to_string(), "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])));");
    }

}