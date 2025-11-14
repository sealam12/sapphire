use std::collections::HashMap;
use crate::error::RuntimeError;
use crate::value::Value;
use crate::token::Token;

pub struct Environment {
    pub values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    pub fn get(&mut self, name: &Token) -> Result<Value, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else {
            Err(RuntimeError::new(("Undefined variable: '".to_string() + name.lexeme.as_str() + "'.").as_str(), name.line))
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
}