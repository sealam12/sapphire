use std::fmt::Debug;
use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::value::Value;

pub trait SapCallable: Debug {
    // arity returns the number of arguments the function expects
    fn arity(&self) -> usize;
    // call executes the function, taking a reference to the interpreter and arguments
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, RuntimeError>;

    fn to_string(&self) -> String {
        "<fn callable>".to_owned()
    }

    fn clone_callable(&self) -> Box<dyn SapCallable>;
}