use log::error;
use environment::environment::Environment;
use evaluator::evaluator::eval;
use lexer::lexer::Lexer;
use parser::parser::{Parser, ParserError};

pub fn execute_program(input: String) -> Result<(), anyhow::Error> {
    println!("Executing program: {}", input);
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let mut environment = Environment::new();

    let program = parser.parse_program();

    if program.is_err() {
        let errors = program.err().unwrap();
        for error in errors {
            match error {
                ParserError::UnexpectedToken { expected, actual, line, column } => {
                    error!("Error: Unexpected token: expected {:?}, got {:?} at line {}, column {}", expected, actual, line, column);
                }
                _ => {
                    error!("Error: {:?}", error);
                }
            }
        }
        return Ok(());
    }

    let program = program.unwrap();
    let evaluated = eval(&program, &mut environment);
    if evaluated.is_err() {
        error!("Error: {:?}", evaluated.err().unwrap());
        return Ok(());
    }

    let evaluated = evaluated.unwrap();
    println!("{}", evaluated);

    Ok(())
}