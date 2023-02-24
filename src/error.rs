#[derive(Debug)]
pub enum LoxError {
    TokenError(),
    ParseError(ParseError),
    RuntimeError(RuntimeError),
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn report(&self) {
        println!("Error: {}", self.message);
    }

    pub fn new(msg: String) -> Self {
        Self { message: msg }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

impl RuntimeError {
    pub fn report(&self) {
        println!("RuntimeError: {}", self.message);
    }

    pub fn new(msg: String) -> Self {
        Self { message: msg }
    }
}
