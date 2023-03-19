use anyhow::Result;
use log::{error, warn};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use evaluator::evaluator::eval;

use lexer::lexer::Lexer;
use parser::parser::{Parser, ParserError};

pub fn start(prompt: &str) -> Result<(), anyhow::Error> {
    let mut reader = DefaultEditor::new()?;
    let lexer = Lexer::default();
    let mut parser = Parser::new(lexer);

    if reader.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = reader.readline(prompt);
        let prompt_len = prompt.len() as u32;
        match readline {
            Ok(line) => {
                reader.add_history_entry(line.as_str())?;
                parser.reset(line);

                let program = parser.parse_program();

                if program.is_err() {
                    let errors = program.err().unwrap();
                    for error in errors {
                        match error {
                            ParserError::UnexpectedToken { expected, actual, line, column } => {
                                // print caret under the token
                                let mut caret = String::new();
                                for _ in 0..(column + prompt_len - 1) {
                                    caret.push(' ');
                                }
                                caret.push('^');

                                println!("{}", caret);
                                error!("Error: Unexpected token: expected {:?}, got {:?} at line {}, column {}", expected, actual, line, column);
                            }
                            _ => {
                                error!("Error: {:?}", error);
                            }
                        }
                    }
                    continue;
                }

                let program = program.unwrap();
                let evaluated = eval(&program);
                println!("{}", evaluated);
            }
            Err(ReadlineError::Interrupted) => {
                warn!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                warn!("CTRL-D");
                break;
            }
            Err(err) => {
                error!("IO Error: {:?}", err);
                break;
            }
        }
    }


    reader.save_history("history.txt")?;

    Ok(())
}

