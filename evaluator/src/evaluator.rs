use ast::program::Program;
use lexer::lexer::Lexer;
use object::object::ObjectType;
use log::debug;
use ast::expression::Expression;
use ast::statement::Statement;

pub fn eval(program: &Program) -> ObjectType {
    let mut result = ObjectType::Null;
    program.statements.iter().for_each(|statement| {
        debug!("Statement: {}", statement);

        result = eval_node(statement);
    });

    result
}

fn eval_node(node: &Statement) -> ObjectType {
    return match node {
        Statement::ExpressionStatement(expr) => eval_expression(expr),
        _ => ObjectType::Null,
    }
}

fn eval_expression(expr: &Expression) -> ObjectType {
    return match expr {
        Expression::IntegerLiteral(value) => ObjectType::Integer(*value),
        Expression::BooleanLiteral(value) => ObjectType::Boolean(*value),
        _ => ObjectType::Null,
    }
}

#[cfg(test)]
mod tests {
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

        eval(&program)
    }

    #[test]
    fn test_integer_literal() {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::try_init();

        let tests = vec![
            ("5", 5),
            ("10", 10),
            // ("-5", -5),
            // ("-10", -10),
            ("5;", 5),
            ("10;", 10),
            // ("-5;", -5),
            // ("-10;", -10),
        ];

        tests.iter().for_each(|(input, result)| {
            let evaluated = test_eval(input.to_string());
            assert_eq!(evaluated, ObjectType::Integer(*result));
        })
    }
}