use anyhow::Result;
use log::{error, info, warn};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use environment::environment::Environment;
use error::EvaluatorError;
use evaluator::evaluator::eval;
use lexer::lexer::Lexer;
use parser::parser::{Parser};

fn build_caret(column: &u32, prompt_len: &u32) -> String {
    let mut caret = String::new();
    for _ in 0..(column + prompt_len - 1) {
        caret.push(' ');
    }
    caret.push('^');

    caret
}

pub fn start(prompt: &str) -> Result<(), anyhow::Error> {
    let mut reader = DefaultEditor::new()?;
    let lexer = Lexer::default();
    let mut parser = Parser::new(lexer)?;
    let mut environment = Environment::new();
    let print_evaluated_result = flags::PRINT_EVALUATED_RESULT.load(std::sync::atomic::Ordering::Relaxed);

    if reader.load_history("history.txt").is_err() {
        info!("No previous history.");
    }

    loop {
        let readline = reader.readline(prompt);
        let prompt_len = &(prompt.len() as u32);
        match readline {
            Ok(line) => {
                reader.add_history_entry(line.as_str())?;
                if let Err(err) = parser.reset(line) {
                    error!("Error: {:}", err);
                    continue;
                }

                let program = parser.parse_program();

                let mut is_first = true;
                if program.is_err() {
                    let errors = program.err().unwrap();
                    for error in errors {
                        match &error {
                            EvaluatorError::UnexpectedToken { column, .. }  | EvaluatorError::UnfinishedString { column, .. }=> {
                                if is_first {
                                    println!("{}", build_caret(column, prompt_len));
                                }
                                error!("Error: {:}", error);
                            }
                            _ => {
                                error!("Error: {:}", error);
                            }
                        }
                        is_first = false;
                    }
                    continue;
                }

                let program = program.unwrap();
                let evaluated = eval(&program, &mut environment);
                if evaluated.is_err() {
                    error!("Error: {:}", evaluated.err().unwrap());
                    continue;
                }

                let evaluated = evaluated.unwrap();
                if print_evaluated_result {
                    println!("{}", evaluated);
                }
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

