use log::{error, info};
use linefeed::{Interface, ReadResult};
use anyhow::Result;
use lexer::lexer::{Lexer};
use lexer::token::TokenType;

pub fn start(prompt: &str) -> Result<(), anyhow::Error> {
    let reader = Interface::new("interpreter")?;

    reader.set_prompt(prompt)?;

    reader.set_report_signal(linefeed::terminal::Signal::Interrupt, true);

    let mut lexer = Lexer::default();

    while let ReadResult::Input(input) = reader.read_line()? {
        lexer.reset(input);

        loop {
            let token = lexer.next_token();

            match token.kind {
                TokenType::EOF => {
                    info!("EOF");
                    break;
                }
                TokenType::ILLEGAL(_) => {
                    error!("Illegal token: {}", token.to_string());
                    break;
                }
                _ => {
                    info!("{}", token.to_string());
                }
            }
        }
    }

    Ok(())
}

