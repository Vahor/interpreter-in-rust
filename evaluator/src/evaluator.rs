use log::{debug, warn};
use thiserror::Error;

use ast::expression::Expression;
use ast::program::Program;
use ast::statement::{BlockStatement, Statement};
use object::object::ObjectType;

#[derive(Error, Debug)]
pub enum EvaluatorError {
    #[error("Operator not supported: {actual}")]
    OperatorNotSupported {
        actual: String,
    },

    #[error("Type mismatch: {expected} {operator} {actual}")]
    TypeMismatch {
        expected: String,
        operator: String,
        actual: String,
    },
}

fn operator_not_supported(actual: String) -> EvaluatorError {
    EvaluatorError::OperatorNotSupported {
        actual: actual.to_string(),
    }
}

fn type_missmatch(expected: &str, operator: &str, actual: &str) -> EvaluatorError {
    EvaluatorError::TypeMismatch {
        expected: expected.to_string(),
        operator: operator.to_string(),
        actual: actual.to_string(),
    }
}

pub fn eval(program: &Program) -> Result<ObjectType, EvaluatorError> {
    let result = eval_block_statement(&program.statements)?;

    if let ObjectType::Return(obj) = result {
        return Ok(*obj);
    }

    Ok(result)
}

fn eval_block_statement(statements: &BlockStatement) -> Result<ObjectType, EvaluatorError> {
    let iter = statements.iter();
    let mut result = ObjectType::Null;

    for statement in iter {
        let evaluated = eval_node(statement);
        if let Err(error) = evaluated {
            return Err(error);
        }
        let evaluated = evaluated.unwrap();
        result = evaluated;

        if let ObjectType::Return(_) = result {
            break;
        }
    }

    return Ok(result);
}

fn eval_node(node: &Statement) -> Result<ObjectType, EvaluatorError> {
    return match node {
        Statement::ExpressionStatement(expr) => eval_expression(expr),
        Statement::ReturnStatement { value } => {
            let evaluated = eval_expression(value)?;
            return Ok(ObjectType::Return(Box::new(evaluated)));
        }
        _ => Err(operator_not_supported(node.to_string())),
    };
}

fn eval_expression(expr: &Expression) -> Result<ObjectType, EvaluatorError> {
    return match expr {
        Expression::IntegerLiteral(value) => Ok(ObjectType::Integer(*value)),
        Expression::BooleanLiteral(value) => {
            if *value {
                Ok(ObjectType::Boolean(true))
            } else {
                Ok(ObjectType::Boolean(false))
            }
        }
        Expression::PrefixExpression { operator, right } => eval_prefix_expression(operator, &eval_expression(right)?),
        Expression::InfixExpression { left, operator, right } => eval_infix_expression(operator, &eval_expression(left)?, &eval_expression(right)?),
        Expression::IfExpression { condition, consequence, alternative } => eval_if_expression(condition, consequence, alternative),
        _ => Err(operator_not_supported(expr.to_string())),
    };
}

fn eval_prefix_expression(operator: &str, right: &ObjectType) -> Result<ObjectType, EvaluatorError> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Err(operator_not_supported(operator.to_string())),
    }
}

fn eval_infix_expression(operator: &str, left: &ObjectType, right: &ObjectType) -> Result<ObjectType, EvaluatorError> {
    match (left, right) {
        (ObjectType::Integer(left_value), ObjectType::Integer(right_value)) => {
            eval_integer_infix_expression(operator, left_value, right_value)
        }
        (ObjectType::Boolean(left_value), ObjectType::Boolean(right_value)) => {
            eval_boolean_infix_expression(operator, left_value, right_value)
        }
        _ => Err(type_missmatch(left.to_string().as_str(), operator, right.to_string().as_str())),
    }
}

fn eval_integer_infix_expression(operator: &str, left: &i64, right: &i64) -> Result<ObjectType, EvaluatorError> {
    match operator {
        "+" => Ok(ObjectType::Integer(left + right)),
        "-" => Ok(ObjectType::Integer(left - right)),
        "*" => Ok(ObjectType::Integer(left * right)),
        "/" => Ok(ObjectType::Integer(left / right)),
        "<" => Ok(ObjectType::Boolean(left < right)),
        "<=" => Ok(ObjectType::Boolean(left <= right)),
        ">" => Ok(ObjectType::Boolean(left > right)),
        ">=" => Ok(ObjectType::Boolean(left >= right)),
        "==" => Ok(ObjectType::Boolean(left == right)),
        "!=" => Ok(ObjectType::Boolean(left != right)),
        _ => Err(operator_not_supported(operator.to_string())),
    }
}

fn eval_boolean_infix_expression(operator: &str, left: &bool, right: &bool) -> Result<ObjectType, EvaluatorError> {
    match operator {
        "==" => Ok(ObjectType::Boolean(left == right)),
        "!=" => Ok(ObjectType::Boolean(left != right)),
        _ => Err(type_missmatch(left.to_string().as_str(), operator, right.to_string().as_str())),
    }
}

fn eval_bang_operator_expression(right: &ObjectType) -> Result<ObjectType, EvaluatorError> {
    match right {
        ObjectType::Boolean(value) => {
            if value == &true {
                Ok(ObjectType::Boolean(false))
            } else {
                Ok(ObjectType::Boolean(true))
            }
        }
        ObjectType::Integer(value) => {
            if *value == 0 {
                Ok(ObjectType::Boolean(true))
            } else {
                Ok(ObjectType::Boolean(false))
            }
        }
        ObjectType::Null => Ok(ObjectType::Boolean(true)),
        _ => Err(operator_not_supported(right.to_string())),
    }
}

fn eval_minus_prefix_operator_expression(right: &ObjectType) -> Result<ObjectType, EvaluatorError> {
    match right {
        ObjectType::Integer(value) => Ok(ObjectType::Integer(-*value)),
        _ => Err(operator_not_supported(right.to_string())),
    }
}

fn eval_if_expression(condition: &Expression, consequence: &BlockStatement, alternative: &Option<BlockStatement>) -> Result<ObjectType, EvaluatorError> {
    if is_truthy(&eval_expression(condition)?) {
        return eval_block_statement(consequence);
    }

    if alternative.is_some() {
        return eval_block_statement(alternative.as_ref().unwrap());
    }

    Ok(ObjectType::Null)
}

fn is_truthy(obj: &ObjectType) -> bool {
    match obj {
        ObjectType::Boolean(value) => *value,
        ObjectType::Null => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use lexer::lexer::Lexer;
    use parser::parser::Parser;

    use super::*;

    fn test_eval(input: String) -> ObjectType {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        if program.is_err() {
            panic!("Error: {:?}", program.err().unwrap());
        }

        let program = program.unwrap();

        let evaluated = eval(&program);
        if evaluated.is_err() {
            panic!("Error: {:?}", evaluated.err().unwrap());
        }

        return evaluated.unwrap();
    }

    #[test]
    fn test_integer_literal() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        tests.iter().for_each(|(input, result)| {
            let evaluated = test_eval(input.to_string());
            assert_eq!(evaluated, ObjectType::Integer(*result));
        })
    }

    #[test]
    fn test_boolean_literal() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
            ("1 <= 2", true),
            ("1 >= 2", false),
            ("1 <= 1", true),
            ("1 >= 1", true),
            ("1 <= 0", false),
            ("1 >= 0", true),
        ];

        tests.iter().for_each(|(input, result)| {
            let evaluated = test_eval(input.to_string());
            assert_eq!(evaluated, ObjectType::Boolean(*result));
        })
    }

    #[test]
    fn test_bang_operator() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!!true", true),
            ("!!false", false),
            ("!5", false),
            ("!!5", true),
            ("!0", true),
            ("!!0", false),
        ];

        tests.iter().for_each(|(input, result)| {
            let evaluated = test_eval(input.to_string());
            assert_eq!(evaluated, ObjectType::Boolean(*result));
        })
    }

    #[test]
    fn test_if_else_expressions() {
        let tests = vec![
            ("if (true) {10}", ObjectType::Integer(10)),
            ("if (false) {10}", ObjectType::Null),
            ("if (1) {10}", ObjectType::Integer(10)),
            ("if (1 < 2) {10}", ObjectType::Integer(10)),
            ("if (1 > 2) {10}", ObjectType::Null),
            ("if (1 > 2) {10} else {20}", ObjectType::Integer(20)),
            ("if (1 < 2) {10} else {20}", ObjectType::Integer(10)),
        ];

        tests.iter().for_each(|(input, result)| {
            let evaluated = test_eval(input.to_string());
            assert_eq!(evaluated, *result);
        })
    }

    #[test]
    fn test_return_statements() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let tests = vec![
            ("return 10;", ObjectType::Integer(10)),
            ("return 10; 9;", ObjectType::Integer(10)),
            ("return 2 * 5; 9;", ObjectType::Integer(10)),
            ("9; return 2 * 5; 9;", ObjectType::Integer(10)),
            ("if (10 > 1) { if (10 > 1) { return 10; } return 1; }", ObjectType::Integer(10)),
        ];

        tests.iter().for_each(|(input, result)| {
            let evaluated = test_eval(input.to_string());
            assert_eq!(evaluated, *result);
        })
    }
}