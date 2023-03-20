use std::fmt::Display;

use crate::statement::BlockStatement;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    StringLiteral(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    ArrayLiteral(Vec<Expression>),

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
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },

    FunctionLiteral {
        /// Identifiers
        parameters: Vec<Expression>,
        body: BlockStatement,
    },

    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },

    IndexExpression {
        left: Box<Expression>,
        index: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expression::StringLiteral(string) => write!(f, "{}", string),
            Expression::IntegerLiteral(int) => write!(f, "{}", int),
            Expression::BooleanLiteral(boolean) => write!(f, "{}", boolean),
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::ArrayLiteral(elements) => {
                let mut result = String::new();
                result.push_str("[");
                result.push_str(elements.iter().map(|v| { v.to_string() }).collect::<Vec<_>>().join(", ").as_str());
                result.push_str("]");
                return write!(f, "{}", result);
            }
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
            Expression::FunctionLiteral { parameters, body } => {
                let mut result = String::new();
                result.push_str("fn(");
                result.push_str(parameters.iter().map(|v| { v.to_string() }).collect::<Vec<_>>().join(", ").as_str());
                result.push_str(") { ");
                body.iter().for_each(|statement| {
                    result.push_str(&statement.to_string());
                });
                result.push_str(" }");
                return write!(f, "{}", result);
            }
            Expression::CallExpression { function, arguments } => {
                let mut result = String::new();
                result.push_str(&function.to_string());
                result.push_str("(");
                result.push_str(arguments.iter().map(|v| { v.to_string() }).collect::<Vec<_>>().join(", ").as_str());
                result.push_str(")");
                return write!(f, "{}", result);
            }
            Expression::IndexExpression { left, index } => write!(f, "({}[{}])", left, index),
        };
    }
}