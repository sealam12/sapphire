use crate::expr::{self, Expr};
use crate::token::Token;
use crate::token_type::{self, TokenType};
use crate::error::RuntimeError;
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

    pub fn interpret(&mut self, expression: &Expr) {
        let result: Result<Value, RuntimeError> = self.evaluate(expression);

        match result {
            Ok(val) => {
                println!("{:?}", val);
            },

            Err(error) => {
                self.main.runtime_error(error);
            }
        }
    }

    pub fn evaluate(&mut self, expression: &Expr) -> Result<Value, RuntimeError> {
        expression.accept(self)
    }

    pub fn is_truthy(&mut self, value: &Value) -> Result<bool, RuntimeError> {
        match *value {
            Value::Str(_) => Ok(true),
            Value::Number(_) => Ok(true),
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

                    _ => Err(RuntimeError::new("OperationError - Unsupported operand for binary operation on Number", operator.line))
                }
            } else {
                if let (Value::Str(sl), Value::Str(sr)) = (&result_left, &result_right) {
                    return Err(RuntimeError::new("TypeError - Unsupported operation for Str", operator.line));
                }

                Err(RuntimeError::new("TypeError - Type mismatch for operands of binary operation.", operator.line))
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
                Value::Str(val) => return Err(RuntimeError::new("TypeError - Invalid type for unary: 'Str', operand must be Number", operator.line)),
                Value::Null => return Err(RuntimeError::new("TypeError - Invalid type for unary: 'Null', operand must be Number", operator.line)),
                _ => ()
            }

            match operator.token_type {
                TokenType::Bang => {
                    match result_right {
                        Value::Bool(val) => Ok(Value::Bool(!self.is_truthy(&result_right)?)),
                        Value::Number(val ) => Ok(Value::Bool(!self.is_truthy(&result_right)?)),
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
}