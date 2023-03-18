use std::fmt::Display;

use crate::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    EmptyStatement,

    LetStatement {
        identifier: String,
        value: Expression,
    },

    ReturnStatement {
        value: Expression,
    },

    ExpressionStatement(Expression),
}

pub type BlockStatement = Vec<Statement>;

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Statement::EmptyStatement => write!(f, ""),
            Statement::LetStatement { identifier, value } => write!(f, "let {} = {};", identifier, value),
            Statement::ReturnStatement { value } => write!(f, "return {};", value),
            Statement::ExpressionStatement(expr) => {
                if let Expression::IfExpression{..} = expr {
                    // If the expression is an if expression, we don't want to add a semicolon
                    write!(f, "{}", expr)
                } else {
                    write!(f, "{};", expr)
                }
            }
        };
    }
}

impl Statement {
    pub fn to_string(&self) -> String {
        return format!("{}", self);
    }
}