#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Number(f64),
    Bool(bool),
    List(Vec<Value>),
    Null
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
            Value::Null => "nil".to_owned(),
        }
    }
}