use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Expression {

    StringLiteral(String),
    IntegerLiteral(i64),

    GroupedExpression {
        expression: Box<Expression>,
    },

    OperatorExpression {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expression::StringLiteral(string) => write!(f, "{}", string),
            Expression::IntegerLiteral(int) => write!(f, "{}", int),
            Expression::GroupedExpression { expression } => write!(f, "({})", expression),
            Expression::OperatorExpression { left, operator, right } => write!(f, "{} {} {}", left, operator, right),
        };
    }
}