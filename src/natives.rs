use std::time::{SystemTime, UNIX_EPOCH};

use crate::callable::SapCallable;
use crate::value::Value;

#[derive(Debug)]
pub struct SapNativeClock {}

impl SapCallable for SapNativeClock {
    fn arity(&self) -> usize { 0 as usize }
    fn call(&self, interpreter: &mut crate::interpreter::Interpreter, arguments: Vec<crate::value::Value>) -> Result<crate::value::Value, crate::error::RuntimeError> {
        let now = SystemTime::now();
        let duration_since_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Ok(Value::Number((duration_since_epoch.as_millis() as f64) * (0.001 as f64)))
    }

    fn clone_callable(&self) -> Box<dyn SapCallable> {
        Box::new(
            SapNativeClock {}
        )
    }
}