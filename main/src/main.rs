use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{error, info, warn};
use flags::STOP_AT_FIRST_ERROR;

#[derive(Parser, Debug)]
#[command(name = "Interpreter")]
#[command(author = "Nathan D. <me@vahor.fr>")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Takes a string as input and executes it
    #[arg(short = 'c', value_name = "INPUT")]
    inline: Option<String>,

    /// Takes a file as input and executes it
    #[arg(short = 'f', long = "file", value_name = "FILE")]
    file: Option<String>,

    /// (Optional) Stops the program after the first error
    /// (default: false)
    #[arg(short = 's', long = "stop-on-error")]
    stop_on_error: bool,
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
                STOP_AT_FIRST_ERROR.store(true, std::sync::atomic::Ordering::Relaxed);
            }

            if args.inline.is_some() {
                let input = args.inline.unwrap();
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
