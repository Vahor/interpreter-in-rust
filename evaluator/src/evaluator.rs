use log::{debug, warn};
use thiserror::Error;

use ast::expression::Expression;
use ast::program::Program;
use ast::statement::{BlockStatement, Statement};
use environment::environment::Environment;
use environment::object::ObjectType;

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

    #[error("Unknown identifier: {identifier}")]
    UnknownIdentifier {
        identifier: String,
    },
}

fn operator_not_supported(actual: String) -> EvaluatorError {
    EvaluatorError::OperatorNotSupported {
        actual,
    }
}

fn type_missmatch(expected: &str, operator: &str, actual: &str) -> EvaluatorError {
    EvaluatorError::TypeMismatch {
        expected: expected.to_string(),
        operator: operator.to_string(),
        actual: actual.to_string(),
    }
}

fn unknown_identifier(identifier: &str) -> EvaluatorError {
    EvaluatorError::UnknownIdentifier {
        identifier: identifier.to_string(),
    }
}

pub fn eval(program: &Program, environment: &mut Environment) -> Result<ObjectType, EvaluatorError> {
    let result = eval_block_statement(environment, &program.statements)?;

    if let ObjectType::Return(obj) = result {
        return Ok(*obj);
    }

    Ok(result)
}


fn eval_node(environment: &mut Environment, node: &Statement) -> Result<ObjectType, EvaluatorError> {
    return match node {
        Statement::ExpressionStatement(expr) => eval_expression(environment, expr),
        Statement::ReturnStatement { value } => {
            let evaluated = eval_expression(environment, value)?;
            return Ok(ObjectType::Return(Box::new(evaluated)));
        }
        Statement::LetStatement { value, identifier } => {
            let evaluated = eval_expression(environment, value)?;
            environment.set(identifier, evaluated);
            return Ok(ObjectType::Null);
        }
        _ => Err(operator_not_supported(node.to_string())),
    };
}

fn eval_expression(environment: &mut Environment, expr: &Expression) -> Result<ObjectType, EvaluatorError> {
    return match expr {
        Expression::IntegerLiteral(value) => Ok(ObjectType::Integer(*value)),
        Expression::BooleanLiteral(value) => {
            if *value {
                Ok(ObjectType::Boolean(true))
            } else {
                Ok(ObjectType::Boolean(false))
            }
        }
        Expression::PrefixExpression { operator, right } => eval_prefix_expression(operator, &eval_expression(environment, right)?),
        Expression::InfixExpression { left, operator, right } => eval_infix_expression(operator, &eval_expression(environment, left)?, &eval_expression(environment, right)?),
        Expression::IfExpression { condition, consequence, alternative } => eval_if_expression(environment, condition, consequence, alternative),
        Expression::Identifier(identifier) => {
            let value = environment.get(identifier);
            if let Some(value) = value {
                return Ok(value.clone()); // TODO: clone?
            }
            Err(unknown_identifier(identifier))
        },
        Expression::FunctionLiteral { parameters, body } => Ok(ObjectType::Function {
            parameters: parameters.clone(), // TODO: clone?
            body: body.clone(), // TODO: clone?
            environment: environment.clone(), // TODO: clone?
        }),
        Expression::CallExpression { function, arguments } => {
            let evaluated = eval_expression(environment, function)?;

            let mut evaluated_arguments = vec![];

            let arguments = arguments.iter();

            for argument in arguments {
                evaluated_arguments.push(eval_expression(environment, argument)?);
            }

            return apply_function(&evaluated, &evaluated_arguments);
        }
        _ => Err(operator_not_supported(expr.to_string())),
    };
}

fn eval_block_statement(environment: &mut Environment, statements: &BlockStatement) -> Result<ObjectType, EvaluatorError> {
    let iter = statements.iter();
    let mut result = ObjectType::Null;

    for statement in iter {
        let evaluated = eval_node(environment, statement);
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

fn eval_if_expression(environment: &mut Environment, condition: &Expression, consequence: &BlockStatement, alternative: &Option<BlockStatement>) -> Result<ObjectType, EvaluatorError> {
    if is_truthy(&eval_expression(environment, condition)?) {
        return eval_block_statement(environment, consequence);
    }

    if alternative.is_some() {
        return eval_block_statement(environment, alternative.as_ref().unwrap());
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

fn apply_function(function: &ObjectType, args: &Vec<ObjectType>) -> Result<ObjectType, EvaluatorError> {

    if let ObjectType::Function {parameters, body, environment} = function {
        let mut enclosing_environment = Environment::new_enclosed(environment);
        for (value, name) in parameters.iter().zip(args.iter()) {
            enclosing_environment.set(value.to_string().as_str(), name.clone()); // TODO: clone?
        }

        return eval_block_statement(&mut enclosing_environment, body);
    }

    Err(operator_not_supported("function".to_string()))
}

#[cfg(test)]
mod tests {
    use lexer::lexer::Lexer;
    use parser::parser::Parser;

    use super::*;

    fn test_eval(input: String) -> ObjectType {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let mut env = Environment::new();

        let program = parser.parse_program();

        if program.is_err() {
            panic!("Error: {:?}", program.err().unwrap());
        }

        let program = program.unwrap();

        let evaluated = eval(&program, &mut env);
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

    #[test]
    fn test_let_statements() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let tests = vec![
            ("let a = 5; a;", ObjectType::Integer(5)),
            ("let a = 5 * 5; a;", ObjectType::Integer(25)),
            ("let a = 5; let b = a; b;", ObjectType::Integer(5)),
            ("let a = 5; let b = a; let c = a + b + 5; c;", ObjectType::Integer(15)),
        ];

        tests.iter().for_each(|(input, result)| {
            let evaluated = test_eval(input.to_string());
            assert_eq!(evaluated, *result);
        })
    }

    #[test]
    fn test_function_definition() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let test = "fn(x) { x + 2;};";

        let evaluated = test_eval(test.to_string());
        if let ObjectType::Function { parameters, body, .. } = evaluated {
            assert_eq!(parameters.len(), 1);
            assert_eq!(parameters[0].to_string(), "x");
            assert_eq!(body[0].to_string(), "(x + 2);");
        } else {
            panic!("object is not a function");
        }
    }

    #[test]
    fn test_function_application() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", ObjectType::Integer(5)),
            ("let identity = fn(x) { return x; }; identity(5);", ObjectType::Integer(5)),
            ("let double = fn(x) { x * 2; }; double(5);", ObjectType::Integer(10)),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", ObjectType::Integer(10)),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", ObjectType::Integer(20)),
            ("fn(x) { x; }(5)", ObjectType::Integer(5)),
        ];

        tests.iter().for_each(|(input, result)| {
            let evaluated = test_eval(input.to_string());
            assert_eq!(evaluated, *result);
        })
    }

    #[test]
    fn test_closure() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let input = r#"
        let newAdder = fn(x) {
            fn(y) { x + y };
        };

        let addTwo = newAdder(2);
        addTwo(2);
        "#;

        let evaluated = test_eval(input.to_string());
        assert_eq!(evaluated, ObjectType::Integer(4));
    }
}