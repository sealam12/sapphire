use std::collections::HashMap;
use std::rc::Rc; // Used for shared ownership (Reference Counting)
use std::cell::RefCell; // Used for interior mutability (borrow_mut() at runtime)

// Assume these types are defined in other modules
use crate::error::RuntimeError; 
use crate::value::Value; 
use crate::token::Token; 

// Remove #[derive(Clone)] -- we use Rc::clone() instead, which is cheap
#[derive(Debug)] // Derive Debug for easier debugging if needed
pub struct Environment {
    // The enclosing environment is now a shared, mutable reference pointer
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Value>
}

impl Environment {
    // Constructor takes the new shared pointer type
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self { enclosing, values: HashMap::new() }
    }

    // `define` takes `&mut self` because it only affects the *current* scope, 
    // where we have unique mutable access during declaration.
    pub fn define(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            Err(RuntimeError::new(
                &format!("Attempted to initialize already initialized variable: '{}'.", name.lexeme),
                name.line
            ))
        } else {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        }
    }

    // `get` takes `&self` but uses `.borrow()` internally to read from parent scopes safely
    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            // Return a clone of the value found
            return Ok(value.clone());
        }

        // Recursively try to get the variable from the enclosing scope
        if let Some(enclosing_env_ptr) = &self.enclosing {
            // Use .borrow() to safely get a read reference to the parent Environment
            return enclosing_env_ptr.borrow().get(name);
        }
        
        Err(RuntimeError::new(
            &format!("Undefined variable: '{}'.", name.lexeme),
            name.line
        ))
    }

    // `assign` takes `&self` but uses `.borrow_mut()` internally to write to parent scopes
    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            // Assign in the current scope
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        // Recursively try to assign in the enclosing scope
        if let Some(enclosing_env_ptr) = &self.enclosing {
            // Use .borrow_mut() to safely get a mutable reference to the parent Environment
            return enclosing_env_ptr.borrow_mut().assign(name, value);
        }

        Err(RuntimeError::new(
            &format!("Attempted to assign to undefined variable: '{}'.", name.lexeme),
            name.line
        ))
    }
}