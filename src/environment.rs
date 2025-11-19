use std::collections::HashMap;
use crate::error::RuntimeError;
use crate::value::Value;
use crate::token::Token;

#[derive(Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Value>
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new()
        }
    }

    pub fn get(&mut self, name: &Token) -> Result<Value, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        }

        match &mut self.enclosing {
            Some(enclosing_env) => Ok(enclosing_env.get(name)?),
            None => Err(RuntimeError::new(
                ("Undefined variable: '".to_owned() + name.lexeme.as_str() + "'.").as_str(), 
                name.line
            ))
        }
    }

    pub fn define(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            Err(RuntimeError::new(
                ("Attempted to initialize already initialized variable: '".to_owned() + name.lexeme.as_str() + "'.").as_str(),
                name.line
            ))
        } else {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        }
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }
        
        match &mut self.enclosing {
            Some(enclosing_env) => Ok(enclosing_env.assign(name, value)?),
            None => Err(RuntimeError::new(
                ("Attempted to assign to undefined variable: '".to_owned() + name.lexeme.as_str() + "'.").as_str(), 
                name.line
            ))
        }
    }
}