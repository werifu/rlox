use std::io::{self, Write};

use crate::error::ParseError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::fs::File;
use std::io::Read;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }
}

impl Lox {
    /// execute a .lox file
    /// TODO: error handler
    pub fn run_file(&self, filename: String) {
        let mut file = File::open(filename).unwrap();
        let mut src_code = String::new();

        file.read_to_string(&mut src_code).unwrap();
        self.run(&src_code).unwrap();
        if self.had_error {
            return;
        };
    }

    /// create an interactive shell environment
    /// TODO: error handler
    pub fn run_prompt(&self) {
        loop {
            print!(">>>");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Err(err) = self.run(&input) {
                        err.report();
                    }
                }
                Err(error) => println!("error: {}", error),
            }
        }
    }

    pub fn run(&self, source: &str) -> Result<String, ParseError> {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        if !parser.all_parsed() {
            return Err(ParseError::new("not all token parsed".to_string()));
        }
        let interpreter = Interpreter::new();
        let res = interpreter.evaluate(&expr);
        if let Ok(Some(value)) = res {
            println!("{:?}", value);
        } else {
            println!("error!");
        }
        // println!("{}", expr.to_string());
        Ok(expr.to_string())
    }
}

#[test]
fn parse_single_expr() {
    let lox = Lox::new();
    let kvs = vec![
        ("1 + 2", "(+ 1 2)"),
        ("-1 * (-3 + 4)", "(* (- 1) (grouping (+ (- 3) 4)))"),
    ];
    for (k, v) in kvs.iter() {
        let res = lox.run(k).unwrap();
        assert_eq!(res, v.to_string());
    }
}
