#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(usize),
    Bool(bool),
    None
}