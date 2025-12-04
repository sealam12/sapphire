use crate::interpreter::Interpreter;
use crate::environment::Environment;
use crate::callable::SapCallable;
use crate::error::RuntimeError;
use crate::value::Value;
use crate::stmt::Stmt;

#[derive(Debug)]
pub struct SapFunction {
    declaration: Stmt
}

impl SapFunction {
    fn new(declaration: Stmt) -> Self {
        Self {
            declaration
        }
    }
}

impl SapCallable for SapFunction {
    fn arity(&self) -> usize {
        if let Stmt::Function { name, params, body } = &self.declaration {
            params.len()
        } else {
            unreachable!()
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        let mut new_environment: Environment = Environment::new(
            Option::Some(interpreter.main.globals.clone())
        );

        if let Stmt::Function { name, params, body } = &self.declaration {
            for i in 0..params.len() {
                let param = params.get(i).unwrap();
                let arg: &Value = match arguments.get(i) {
                    Some(v) => v,
                    None => return Err(RuntimeError::new("Invalid # of arguments to function.", name.line))
                };

                new_environment.define(param, arg.clone())?;
            }

            interpreter.execute_block(body, new_environment)?;
            Ok(Value::Null)
        } else {
            unreachable!()
        }
    }

    fn clone_callable(&self) -> Box<dyn SapCallable> {
        Box::new(
            SapFunction::new(self.declaration.clone())
        )
    }
}