use std::fmt::Display;
use crate::expression::Expression;

#[derive(Debug)]
pub struct LetStatementData {
    pub identifier: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct ReturnStatementData {
    pub value: Expression,
}

#[derive(Debug)]
pub enum Statement {

    LetStatement(LetStatementData),

    ReturnStatement(ReturnStatementData),

    ExpressionStatement(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Statement::LetStatement(let_stmt) => write!(f, "let {} = {};", let_stmt.identifier, let_stmt.value),
            Statement::ReturnStatement(return_stmt) => write!(f, "return {};", return_stmt.value),
            Statement::ExpressionStatement(expr) => write!(f, "{};", expr),
        };
    }
}