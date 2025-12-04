use crate::callable::SapCallable;

#[derive(Debug)]
pub enum Value {
    Str(String),
    Number(f64),
    Bool(bool),
    List(Vec<Value>),
    Callable(Box<dyn SapCallable>),
    Null
}

// Implement Clone manually for Value
impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Str(s) => Value::Str(s.clone()),
            Value::Number(n) => Value::Number(*n),
            Value::Bool(b) => Value::Bool(*b),
            Value::List(l) => Value::List(l.clone()),
            // The Callable variant cannot be cloned generically without a custom trait method
            Value::Callable(c) => Value::Callable(c.clone_callable()),
            Value::Null => Value::Null,
        }
    }
}

// Implement PartialEq manually for Value
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(s1), Value::Str(s2)) => s1 == s2,
            (Value::Number(n1), Value::Number(n2)) => (n1 - n2).abs() < f64::EPSILON, // Simple float comparison
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::List(l1), Value::List(l2)) => l1 == l2,
            // Trait objects don't support generic PartialEq comparisons easily.
            (Value::Callable(_), Value::Callable(_)) => false, // Or define what equality means for callables
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Str(str) => str.clone(),
            Value::Number(num) => num.to_string(),
            Value::Bool(bool) => bool.to_string(),
            Value::List(list) => {
                let mut string: String = "[".to_owned();

                let mut string_list: Vec<String> = vec![];
                for val in list {
                    string_list.push(val.to_string());
                }

                string += &string_list.join(", ");
                string += "]";

                string
            },
            Value::Callable(cal) => {
                "<fn>".to_owned()
            },
            Value::Null => "nil".to_owned(),
        }
    }
}