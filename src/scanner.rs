use crate::value::Value;
use crate::token::Token;
use crate::token_type::TokenType;
use super::Sapphire;

use std::collections::HashMap;

pub struct Scanner<'a> {
    pub main: &'a mut Sapphire,
    pub source: String,
    tokens: Vec<Token>,
    
    start: usize,
    current: usize,
    line: usize,
}

pub fn get_keywords() -> HashMap<String, TokenType> {
    HashMap::from([
        (String::from("and"), TokenType::And),
        (String::from("class"), TokenType::Class),
        (String::from("else"), TokenType::Else),
        (String::from("false"), TokenType::False),
        (String::from("fn"), TokenType::Fun),
        (String::from("for"), TokenType::For),
        (String::from("if"), TokenType::If),
        (String::from("nil"), TokenType::Nil),
        (String::from("or"), TokenType::Or),
        (String::from("println"), TokenType::PrintLn),
        (String::from("return"), TokenType::Return),
        (String::from("super"), TokenType::Super),
        (String::from("this"), TokenType::This),
        (String::from("true"), TokenType::True),
        (String::from("var"), TokenType::Var),
        (String::from("while"), TokenType::While),  
    ])
}

impl<'a> Scanner<'a> {
    pub fn new(main: &'a mut Sapphire, source: String) -> Self {
        Self {
            main: main,
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn is_at_end(&self) -> bool {
        let source_length: usize = self.source.len();
        let result: bool = self.current >= source_length;

        result
    }

    fn next_at_end(&self) -> bool {
        let source_length: usize = self.source.len();
        let result: bool = (self.current + 1) >= source_length;

        result
    }

    fn advance(&mut self) -> char {
        let char: char = self.source.chars()
            .nth(self.current)
            .expect("No character at current index");
        self.current += 1;

        char
    }

    fn add_token_short(&mut self, token_type: TokenType) {
        self.add_token(token_type, Value::Null)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Value) {
        let lex: String = self.source.chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: lex,
            literal: literal,
            line: self.line
        });
    }

    fn match_char(&mut self, expected_char: char) -> bool {
        if self.is_at_end() { return false; }
        let next_char = self.source.chars()
            .nth(self.current)
            .unwrap();

        if next_char != expected_char { return false; }
        
        self.advance();
        true
    }

    fn match_to_type(&mut self, expected_char: char, type_unmatched: TokenType, type_matched: TokenType) {
        let matched: bool = self.match_char(expected_char);
        self.add_token_short(
            if matched { type_matched } else { type_unmatched }
        );
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() { return '\0'; }

        self.source
            .chars()
            .nth(self.current)
            .unwrap()
    }

    fn peek_next(&mut self) -> char {
        if self.next_at_end() {
            return '\0';
        }

        self.source
            .chars()
            .nth(self.current + 1)
            .unwrap()
    }

    fn is_alpha(&mut self, character: char) -> bool {
        (character >= 'a' && character <= 'z') ||
        (character >= 'A' && character <= 'Z') ||
            character == '_'
    }

    fn is_digit(&mut self, character: char) -> bool {
        character >= '0' && character <= '9'
    }
    
    fn is_alpha_numeric(&mut self, character: char) -> bool {
        self.is_alpha(character) || self.is_digit(character)
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token_short(TokenType::LeftParen),
            ')' => self.add_token_short(TokenType::RightParen),
            '{' => self.add_token_short(TokenType::LeftBrace),
            '}' => self.add_token_short(TokenType::RightBrace),
            ',' => self.add_token_short(TokenType::Comma),
            '.' => self.add_token_short(TokenType::Dot),
            '-' => self.add_token_short(TokenType::Minus),
            '+' => self.add_token_short(TokenType::Plus),
            ';' => self.add_token_short(TokenType::Semicolon),
            '*' => self.add_token_short(TokenType::Star),
            '!' => self.match_to_type('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.match_to_type('=', TokenType::Equal, TokenType::EqualEqual),
            '>' => self.match_to_type('=', TokenType::Greater, TokenType::GreaterEqual),
            '<' => self.match_to_type('=', TokenType::Less, TokenType::LessEqual),
            '/' => {
                if self.match_char('/') {
                    while (self.peek() != '\n') && (!self.is_at_end()) {
                        self.advance();
                    }
                } else {
                    self.add_token_short(TokenType::Slash);
                }
            }

            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),

            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                    return;
                } else if self.is_alpha(c) {
                    self.identifier();
                    return;
                }

                self.main.error(self.line, String::from(
                    format!("Unexpected character \"{c}\".")
                ));
            },
        }
    }

    fn identifier(&mut self) {
        let mut peek_next: char = self.peek();
        while self.is_alpha_numeric(peek_next) {
            self.advance();
            peek_next = self.peek();
        }

        let mut token_type: TokenType = TokenType::Identifier;

        let lexeme: String = self.source.chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        let keywords: HashMap<String, TokenType> = get_keywords();
        let keyword: Option<&TokenType> = keywords.get(&lexeme);
        if keyword.is_some() { token_type = keyword.unwrap().clone(); }

        self.add_token_short(token_type);
    }

    fn number(&mut self) {
        let mut next_char: char = self.peek();
        while self.is_digit(next_char) {
            self.advance();
            next_char = self.peek();
        }

        let mut peek_next: char = self.peek_next();
        if next_char == '.' && self.is_digit(peek_next) {
            self.advance(); // consume the .
            while self.is_digit(next_char) {
                self.advance();
                next_char = self.peek();
            }
        }

        let string_literal: String = self.source.chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        
        let int_literal: f64 = string_literal.parse::<f64>().expect("Failed to parse string to i32");

        self.add_token(TokenType::Number, Value::Number(int_literal));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1 }
            self.advance();
        }

        if self.is_at_end() {
            self.main.error(self.line, String::from("Unterminated string"));
        }

        self.advance();

        let string_literal: String = self.source.chars()
            .skip(self.start+1)
            .take((self.current - self.start) - 2)
            .collect();
        self.add_token(TokenType::String, Value::Str(string_literal));
    }
    
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        
        self.tokens.push(Token {token_type: TokenType::EOF, lexeme: String::from(""), literal: Value::Null, line: self.line});
        self.tokens.clone()
    }
}
