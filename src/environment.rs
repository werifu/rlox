use std::collections::HashMap;

use crate::{
    error::{ParseError, RuntimeError},
    expression::{self, LiteralValue},
    token::Token,
};

pub struct Environment {
    pub scopes: Vec<Scope>,
}

pub struct Scope {
    values: HashMap<String, LiteralValue>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        // with the global scope
        Self {
            scopes: vec![Scope::new()],
        }
    }
}

impl Environment {
    pub fn get(&self, name: &str) -> Result<&LiteralValue, RuntimeError> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.values.get(name) {
                return Ok(v);
            }
        }

        Err(RuntimeError::new(format!("Undefined variable `{}`.", name)))
    }

    pub fn define(&mut self, name: &str, value: LiteralValue) {
        let last_scope = self
            .scopes
            .last_mut()
            .expect("Interpretor must have a scope.");
        last_scope.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: Token, value: LiteralValue) -> Result<(), RuntimeError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.values.contains_key(&name.lexeme) {
                scope.values.insert(name.lexeme, value);
                return Ok(());
            }
        }
        Err(RuntimeError::new(format!(
            "Undefined variable `{}`.",
            name.lexeme
        )))
    }

    /// called when enter a new block
    pub fn create_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// called when finish a block
    pub fn drop_scope(&mut self) {
        self.scopes.pop();
    }
}
