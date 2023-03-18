use clap::{Parser, Subcommand};
use log::{error, info};
use anyhow::Result;

use repl::repl;

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
}

#[derive(Subcommand, Debug)]
enum Commands {
}

fn main() -> Result<(), anyhow::Error> {
    // set RUST_LOG=debug to see debug logs
    std::env::set_var("RUST_LOG", "trace");

    env_logger::init();

    let args = Args::parse();

    match args.command {
        Some(command) => match command {
            _ => {
                error!("Not implemented yet");
            }
        },
        None => {
            if args.inline.is_some() {
                let input = args.inline.unwrap();
                info!("Executing inline input: {}", input);
            } else {
                return repl::start(">> ");
            }
        }
    }

    Ok(())
}
