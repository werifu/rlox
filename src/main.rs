mod args;
mod environment;
mod error;
mod expression;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod statement;
mod token;
use args::Args;
use clap::Parser;
use error::LoxError;
use lox::Lox;

fn main() -> Result<(), LoxError> {
    let cli = Args::parse();
    let mut lox = Lox::new(std::io::stdout());
    match cli.file {
        Some(filename) => lox.run_file(filename),
        None => lox.run_prompt(),
    }
    Ok(())
}
