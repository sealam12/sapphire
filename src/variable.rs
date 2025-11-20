use crate::value::Value;

pub enum VariableType {
    Str,
    Number,
    Bool
}

pub struct Variable {
    value: Value,
    var_type: VariableType
}