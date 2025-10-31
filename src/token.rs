use std::fmt;
use crate::token_type::TokenType;
use crate::value::Value;

#[derive(Clone)]
pub struct Token {
   pub token_type: TokenType,
   pub lexeme: String,
   pub literal: Value,
   pub line: usize,
}

impl Token {}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
