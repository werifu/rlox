mod args;
mod error;
mod expression;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod token;

use args::Args;
use clap::Parser;
use error::LoxError;
use lox::Lox;

fn main() -> Result<(), LoxError> {
    let cli = Args::parse();
    let lox = Lox::new();
    match cli.file {
        Some(filename) => lox.run_file(filename),
        None => lox.run_prompt(),
    }
    Ok(())
}
