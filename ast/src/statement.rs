use std::fmt::Display;

use crate::expression::Expression;

#[derive(Debug)]
pub struct ReturnStatementData {
    pub value: Expression,
}

#[derive(Debug)]
pub enum Statement {
    LetStatement {
        identifier: String,
        value: Expression,
    },

    ReturnStatement(ReturnStatementData),

    ExpressionStatement(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Statement::LetStatement { identifier, value } => write!(f, "let {} = {};", identifier, value),
            Statement::ReturnStatement(return_stmt) => write!(f, "return {};", return_stmt.value),
            Statement::ExpressionStatement(expr) => write!(f, "{};", expr),
        };
    }
}