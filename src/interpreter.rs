use std::mem;
use std::rc::Rc;
use std::cell::RefCell;

use crate::environment::Environment;
use crate::token_type::TokenType;
use crate::error::RuntimeError;
use crate::expr::{self, Expr};
use crate::stmt::{self, Stmt};
use crate::value::Value;
use super::Sapphire;

pub struct Interpreter<'a> {
    pub main: &'a mut Sapphire
}

impl<'a> Interpreter<'a> {
    pub fn new(main: &'a mut Sapphire) -> Self {
        Self {
            main: main,
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            let result: Result<(), RuntimeError> = self.execute(&statement);
    
            match result {
                Err(error) => {
                    self.main.runtime_error(error);
                },
                Ok(_) => ()
            }
        }
    }

    pub fn execute(&mut self, statement: &Stmt) -> Result<(), RuntimeError> {
        statement.accept(self)
    }

    pub fn execute_block(&mut self, block: &Stmt, environment: Environment) -> Result<(), RuntimeError> {
        if let Stmt::Block { statements } = block {
            // 1. Swap the current environment (self.main.environment) with the new block environment.
            //    `previous_env_ptr` now holds the pointer to the parent scope that we must restore later.
            //    Note: We wrap the new 'environment: Environment' object in the necessary Rc<RefCell<...>> container.
            let previous_env_ptr = mem::replace(
                &mut self.main.environment, 
                Rc::new(RefCell::new(environment))
            );

            // 2. Execute the statements within the new scope.
            //    We use a block expression (`{ ... }`) to ensure robust scope restoration via `match`.
            let execution_result = {
                // The main loop for executing statements
                for stmt in statements {
                    // If any statement fails (returns Err), the `?` operator immediately exits this block, 
                    // jumping down to the `match` expression below.
                    self.execute(stmt)?; 
                }
                Ok(()) // If loop finishes successfully, return Ok(())
            };

            // 3. CRITICAL: Restore the previous (parent) environment before `execute_block` returns.
            //    This happens whether `execution_result` was Ok or Err.
            self.main.environment = previous_env_ptr;

            // 4. Return the outcome of the execution.
            execution_result

        } else {
            // A helper function for execute_block should only be called with a Block Stmt
            unreachable!("execute_block called with a non-block statement variant")
        }
    }

    pub fn evaluate(&mut self, expression: &Expr) -> Result<Value, RuntimeError> {
        expression.accept(self)
    }

    pub fn is_truthy(&mut self, value: &Value) -> Result<bool, RuntimeError> {
        match *value {
            Value::Str(_) => Ok(true),
            Value::Number(_) => Ok(true),
            Value::List(_) => Ok(true),
            Value::Bool(bool_value) => Ok(bool_value),
            Value::Null => Ok(false)
        }
    }

    pub fn is_equal(&mut self, val1: &Value, val2: &Value) -> bool {
        match (val1, val2) {
            (Value::Null, Value::Null) => true,
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::Str(s1), Value::Str(s2)) => s1 == s2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,

            _ => false, // Catches all other combinations
        }
    }
}

impl<'a> expr::Visitor for Interpreter<'a> {
    type Result = Result<Value, RuntimeError>;

    fn visit_literal(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Literal { value } = expr {
            Ok(value.clone())
        } else {
            unreachable!()
        }
    }
    
    fn visit_binary(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Binary { left, operator, right } = expr {
            let result_left: Value = self.evaluate(left)?;
            let result_right: Value = self.evaluate(right)?;

            if operator.token_type == TokenType::DoubleDot {
                return Ok(Value::Str(result_left.to_string() + result_right.to_string().as_str()));
            }

            if let (Value::Number(nl), Value::Number(nr)) = (&result_left, &result_right) {
                match operator.token_type {
                    TokenType::Plus => Ok(Value::Number(nl + nr)),
                    TokenType::Minus => Ok(Value::Number(nl - nr)),
                    TokenType::Star => Ok(Value::Number(nl * nr)),
                    TokenType::Slash => Ok(Value::Number(nl / nr)),

                    TokenType::Greater => Ok(Value::Bool(nl > nr)),
                    TokenType::GreaterEqual => Ok(Value::Bool(nl >= nr)),
                    TokenType::Less => Ok(Value::Bool(nl < nr)),
                    TokenType::LessEqual => Ok(Value::Bool(nl <= nr)),

                    TokenType::BangEqual => Ok(Value::Bool(nl != nr)),
                    TokenType::EqualEqual => Ok(Value::Bool(nl == nr)),

                    _ => Err(RuntimeError::new("OperationError - Invalid operand for binary expression", operator.line))
                }
            } else {
                if let (Value::Str(_), Value::Str(_)) = (&result_left, &result_right) {
                    return Err(RuntimeError::new("TypeError - Unsupported operation for Str. Maybe you meant to use '..' instead?", operator.line));
                }

                Err(RuntimeError::new("TypeError - Type mismatch for operands of binary operation.", operator.line))
            }
        } else {
            unreachable!()
        }
    }

    fn visit_logical(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Logical { left, operator, right } = expr {
            match operator.token_type {
                TokenType::DoubleAmp => {
                    let left: Value = self.evaluate(left)?; 

                    if !self.is_truthy(&left)? {
                        return Ok(Value::Bool(false));
                    }

                    let right: Value = self.evaluate(right)?;

                    return Ok(Value::Bool(self.is_truthy(&left)? && self.is_truthy(&right)?));
                }

                TokenType::DoublePipe => {
                    let left: Value = self.evaluate(left)?; 

                    if self.is_truthy(&left)? {
                        return Ok(Value::Bool(true));
                    }

                    let right: Value = self.evaluate(right)?;

                    return Ok(Value::Bool(self.is_truthy(&left)? || self.is_truthy(&right)?)); 
                }

                _ => Err(RuntimeError::new("Invalid logical expression operand.", operator.line))
            }
        } else {
            unreachable!()
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Grouping { expression } = expr {
            self.evaluate(expression)
        } else {
            unreachable!()
        }
    }

    fn visit_unary(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Unary { operator, right } = expr {
            let result_right: Value = self.evaluate(right)?;

            match result_right {
                Value::Str(_) => return Err(RuntimeError::new("TypeError - Invalid type for unary: 'Str', operand must be Number", operator.line)),
                Value::Null => return Err(RuntimeError::new("TypeError - Invalid type for unary: 'Null', operand must be Number", operator.line)),
                _ => ()
            }

            match operator.token_type {
                TokenType::Bang => {
                    match result_right {
                        Value::Bool(_) => Ok(Value::Bool(!self.is_truthy(&result_right)?)),
                        Value::Number(_ ) => Ok(Value::Bool(!self.is_truthy(&result_right)?)),
                        _ => unreachable!()
                    }
                },
                TokenType::Minus => {
                    match result_right {
                        Value::Bool(val) => Ok(Value::Bool(!val)),
                        Value::Number(val ) => Ok(Value::Number(-val)),
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_variable(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Variable { name } = expr {
            self.main.environment.borrow().get(name)
        } else {
            unreachable!()
        }
    }

    fn visit_assign(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Assign { name, value } = expr {
            let new_value: Value = self.evaluate(value)?;

            self.main.environment.borrow_mut().assign(name, new_value.clone())?;

            Ok(new_value)
        } else {
            unreachable!()
        }
    }

    fn visit_ternary(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Ternary { condition, if_true, if_false } = expr {
            let condition_value = self.evaluate(&condition)?;
            if self.is_truthy(&condition_value)? {
                self.evaluate(&if_true)
            } else {
                self.evaluate(&if_false)
            }
        } else {
            unreachable!()
        }
    }

    fn visit_list(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::List { expressions } = expr {
            let mut vals: Vec<Value> = vec![];

            for expr in expressions {
                vals.push(self.evaluate(expr)?);
            }

            Ok(Value::List(vals))
        } else {
            unreachable!()
        }
    }

    fn visit_call(&mut self, expr: &Expr) -> Self::Result {
        Ok(Value::Number(32 as f64))
    }

    fn visit_index(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Index { indexee, bracket, index } = expr {
            let indexee_result: Value = self.evaluate(indexee)?;
            let index_result: Value = self.evaluate(index)?;

            let idx: usize;

            match index_result {
                Value::Number(v) => idx = v as usize,
                _ => return Err(RuntimeError::new("TypeError: Type for index must be Number", bracket.line))
            }

            match indexee_result {
                Value::Str(string) => {
                    let char: Option<char> = string.chars().nth(idx);

                    match char {
                        Some(c) => Ok(Value::Str(c.to_string())),
                        None => Err(RuntimeError::new("IndexOutOfBoundsError: Tried to index out of bounds of Str", bracket.line))
                    }
                },

                Value::List(vec) => {
                    let val: Option<&Value> = vec.get(idx);

                    match val {
                        Some(v) => Ok(v.clone()),
                        None => Err(RuntimeError::new("IndexOutOfBoundsError: Tried to index out of bounds of List", bracket.line))
                    }
                },

                _ => Err(RuntimeError::new("TypeError: Invalid type for indexing, must be Str or List", bracket.line))
            }
        } else {
            unreachable!()
        }
    }
}

impl<'a> stmt::Visitor for Interpreter<'a> {
    type Result = Result<(), RuntimeError>;

    fn visit_var(&mut self, statement: &Stmt) -> Self::Result {
        if let Stmt::Var { name, initializer } = statement {
            let val: Value = self.evaluate(initializer)?;

            self.main.environment.borrow_mut().define(name, val)?;

            Ok(())
        } else {
            unreachable!()
        }
    }

    fn visit_expression(&mut self, statement: &Stmt) -> Self::Result {
        if let Stmt::Expression { expression } = statement {
            let _ = self.evaluate(&expression)?;
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn visit_print(&mut self, statement: &Stmt) -> Self::Result {
        if let Stmt::Print { expression } = statement {
            let result: Value = self.evaluate(&expression)?;
            println!("{}", result.to_string());
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn visit_block(&mut self, stmt: &Stmt) -> Self::Result {
        let parent_env_ptr = Rc::clone(&self.main.environment);
        let new_scope_env = Environment::new(Some(parent_env_ptr));

        self.execute_block(stmt, new_scope_env)
    }

    fn visit_if(&mut self, stmt: &Stmt) -> Self::Result {
        if let Stmt::If { condition, then_branch, else_branch } = stmt {
            let condition_result: Value = self.evaluate(condition)?;

            if self.is_truthy(&condition_result)? {
                self.execute(&then_branch)?;
            } else {
                match else_branch {
                    Some(stmt) => {
                        self.execute(stmt)?;
                    },
                    None => (),
                }
            }

            Ok(())
        } else {
            unreachable!()
        }
    }

    fn visit_while(&mut self, stmt: &Stmt) -> Self::Result {
        if let Stmt::While { condition, body } = stmt {
            let mut condition_result: Value = self.evaluate(condition)?;

            while self.is_truthy(&condition_result)? {
                self.execute(&body)?;

                condition_result = self.evaluate(condition)?;
            }

            Ok(())
        } else {
            unreachable!()
        }
    }
}