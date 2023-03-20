use log::debug;
use ast::expression::Expression;
use environment::object::ObjectType;
use error::EvaluatorError;

pub fn convert_object_to_expression(object: ObjectType) -> Result<Expression, EvaluatorError> {
    match object {
        ObjectType::Null => Ok(Expression::NullLiteral),
        ObjectType::Integer(i) => Ok(Expression::IntegerLiteral(i)),
        ObjectType::Boolean(b) => Ok(Expression::BooleanLiteral(b)),
        ObjectType::String(s) => Ok(Expression::StringLiteral(s)),
        ObjectType::Quote(expr) => {
            debug!("Converting quote to expression: {}", expr);
            Ok(*expr)
        }
        _ => Err(EvaluatorError::conversion_error(object.to_string())),
    }
}