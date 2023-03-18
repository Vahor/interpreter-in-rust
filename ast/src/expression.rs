use std::fmt::Display;

use crate::statement::BlockStatement;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    StringLiteral(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),

    Identifier(String),

    PrefixExpression {
        operator: String,
        right: Box<Expression>,
    },

    InfixExpression {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },

    GroupedExpression {
        expression: Box<Expression>,
    },

    IfExpression {
        condition: Box<Expression>,
        consequence: Box<BlockStatement>,
        alternative: Option<Box<BlockStatement>>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expression::StringLiteral(string) => write!(f, "{}", string),
            Expression::IntegerLiteral(int) => write!(f, "{}", int),
            Expression::BooleanLiteral(boolean) => write!(f, "{}", boolean),
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::PrefixExpression { operator, right } => write!(f, "({}{})", operator, right),
            Expression::InfixExpression { left, operator, right } => write!(f, "({} {} {})", left, operator, right),
            Expression::GroupedExpression { expression } => write!(f, "({})", expression),
            Expression::IfExpression { condition, consequence, alternative } => {
                let mut result = String::new();
                result.push_str("if ");
                result.push_str(&condition.to_string());
                result.push_str(" { ");
                consequence.iter().for_each(|statement| {
                    result.push_str(&statement.to_string());
                });
                result.push_str(" }");
                if let Some(alternative) = alternative {
                    result.push_str(" else ");
                    result.push_str("{ ");
                    alternative.iter().for_each(|statement| {
                        result.push_str(&statement.to_string());
                    });
                    result.push_str(" }");
                }
                return write!(f, "{}", result);
            }
        };
    }
}