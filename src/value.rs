#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Bool(bool),
    None
}