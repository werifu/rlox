use std::io::{self, Write};

use crate::error::{LoxError, ParseError};
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::fs::File;
use std::io::Read;

pub struct Lox<W: Write> {
    had_error: bool,
    interpretor: Interpreter<W>,
}

impl<W: Write> Lox<W> {
    pub fn new(output: W) -> Self {
        Self {
            had_error: false,
            interpretor: Interpreter::new(output),
        }
    }
}

impl<W: Write> Lox<W> {
    /// execute a .lox file
    /// TODO: error handler
    pub fn run_file(&mut self, filename: String) {
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
    pub fn run_prompt(&mut self) {
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

    pub fn run(&mut self, source: &str) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse().unwrap();
        if !parser.all_parsed() {
            return Err(LoxError::ParseError(ParseError::new(
                "not all token parsed".to_string(),
            )));
        }
        // execute all statements
        for stmt in stmts {
            self.interpretor
                .execute(&stmt)
                .map_err(|err| LoxError::RuntimeError(err))?;
        }

        // println!("{}", expr.to_string());
        Ok(())
    }
}

#[test]
fn parse_single_expr() {
    let kvs = vec![
        ("1 + 2", "(+ 1 2)"),
        ("-1 * (-3 + 4)", "(* (- 1) (grouping (+ (- 3) 4)))"),
    ];

    for (k, v) in kvs.iter() {
        let tokens = Scanner::new(k.to_string()).scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr.to_string(), v.to_string());
    }
}

#[test]
fn test_execute_var_print() {
    let in_out = vec![
        ("var a = 0; print a = 1;", "1\n"),
        ("var a = 0; a = 1; print a;", "1\n"),
        ("var a = 1; var b = 2; print a + b;", "3\n"),
        ("var a = 1; var b = 2; var c = a; print c + b;", "3\n"),
        (
            "var a = \"a string.\"; var b=\"b string\"; print a + b; ",
            "a string.b string\n",
        ),
        ("print true;", "true\n"),
        ("var a = 1; print !a;", "false\n"),
        ("var a = 1; print !!a;", "true\n"),
        ("var a = 0; print a;", "0\n"),
    ];

    for (src, expected) in in_out {
        let mut buf = vec![];
        let mut lox = Lox::new(&mut buf);
        lox.run(src).unwrap();
        assert_eq!(String::from_utf8_lossy(&buf), expected);
    }
}

#[test]
fn test_block_execute() {
    let in_out = vec![("var a = 0; {var a = 2; print a;} print a;", "2\n0\n")];

    for (src, expected) in in_out {
        let mut buf = vec![];
        let mut lox = Lox::new(&mut buf);
        lox.run(src).unwrap();
        assert_eq!(String::from_utf8_lossy(&buf), expected);
    }
}
