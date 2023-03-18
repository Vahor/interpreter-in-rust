use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {

    StringLiteral(String),
    IntegerLiteral(i64),

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
}

pub fn prefix_expression(operator: String, right: Expression) -> Expression {
    return Expression::PrefixExpression {
        operator,
        right: Box::new(right),
    };
}

pub fn infix_expression(left: Expression, operator: String, right: Expression) -> Expression {
    return Expression::InfixExpression {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    };
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expression::StringLiteral(string) => write!(f, "{}", string),
            Expression::IntegerLiteral(int) => write!(f, "{}", int),
            Expression::Identifier(identifier) => write!(f, "{}", identifier),
            Expression::PrefixExpression { operator, right } => write!(f, "({}{})", operator, right),
            Expression::InfixExpression { left, operator, right } => write!(f, "({} {} {})", left, operator, right),
            Expression::GroupedExpression { expression } => write!(f, "({})", expression),
        };
    }
}