#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Number(f64),
    Bool(bool),
    Null
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Str(str) => str.clone(),
            Value::Number(num) => num.to_string(),
            Value::Bool(bool) => bool.to_string(),
            Value::Null => "nil".to_string(),
        }
    }
}