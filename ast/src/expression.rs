
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