use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{error, info, warn};

#[derive(Parser, Debug)]
#[command(name = "Interpreter")]
#[command(author = "Nathan D. <me@vahor.fr>")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Takes a string as input and executes it
    #[arg(short = 'e', value_name = "INPUT")]
    expression: Option<String>,

    /// Takes a file as input and executes it
    #[arg(short = 'f', long = "file", value_name = "FILE")]
    file: Option<String>,

    /// (Optional) Stops the program after the first error
    /// (default: false)
    #[arg(short = 's', long = "stop-on-error")]
    stop_on_error: bool,

    /// (Optional) Prints the evaluated result
    /// (default: false)
    #[arg(short = 'p', long = "print")]
    print: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {}

fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    let args = Args::parse();

    match args.command {
        Some(command) => match command {
            _ => {
                error!("Not implemented yet");
            }
        },
        None => {
            if args.stop_on_error {
                flags::STOP_AT_FIRST_ERROR.store(true, std::sync::atomic::Ordering::Relaxed);
            }

            if args.print {
                flags::PRINT_EVALUATED_RESULT.store(true, std::sync::atomic::Ordering::Relaxed);
            }

            if args.expression.is_some() {
                let input = args.expression.unwrap();
                info!("Executing inline input: {}", input);
                repl::interpreter::execute_program(input)?;
                return Ok(());
            }
            if args.file.is_some() {
                let file = args.file.unwrap();
                info!("Executing file: {}", file);

                let content = std::fs::read_to_string(&file);
                return if content.is_ok() {
                    repl::interpreter::execute_program(content.unwrap())?;
                    Ok(())
                } else {
                    error!("File {} not found", file);
                    Ok(())
                };
            }

            warn!("No input provided, starting REPL");
            repl::repl::start(">> ")?;
        }
    }

    Ok(())
}
