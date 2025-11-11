use std::error::Error;

use crate::error::ParseError;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;
use crate::expr::Expr;
use super::Sapphire;

pub struct Parser<'a> {
    main: &'a mut Sapphire,
    tokens: Vec<Token>,
    current: usize
}

impl<'a> Parser<'a> {
    pub fn new(main: &'a mut Sapphire, tokens: Vec<Token>) -> Self {
        Self {
            main: main,
            tokens: tokens,
            current: 0 as usize
        }
    }

    pub fn error(&mut self, token: Token, message: String) -> ParseError {
        self.main.token_error(token, message.clone());
        ParseError::new(message.as_str())
    }

    pub fn synchronize(&mut self) {
        let _ = self.advance();

        while !self.is_at_end() {
            if self.previous().clone().token_type == TokenType::Semicolon {
                return;
            }
            
            let peeked_token: &Token = self.peek();
            match peeked_token.token_type {
                TokenType::Class | TokenType::Fn | TokenType::Var | 
                    TokenType::For | TokenType::If | TokenType::While | 
                    TokenType::PrintLn | TokenType::Return => return,
                _ => (),
            }

            let _ = self.advance();
        }
    }

    pub fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    pub fn advance(&mut self) -> Result<&Token, ParseError> {
        if !self.is_at_end() {
            self.current += 1;
            return Ok(self.previous());
        }

        self.main.error(0 as usize, "Expected expression".to_string());
        Err(ParseError::new("Expected expression"))
    }

    pub fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    pub fn match_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                let _ = self.advance();

                return true;
            }
        }

        false
    }

    pub fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, ParseError> {
        if self.check(token_type) { return Ok(self.advance()?); }

        let next_token: Token = self.peek().clone();
        Err(self.error(next_token, message))
    }

    pub fn parse(&mut self) -> Result<Expr, impl Error> {
        self.expression()
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;

        while self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison()?;
            
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.term()?;

        while self.match_types(vec![
                TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual
            ]) {
            
            let operator: Token = self.previous().clone();
            let right: Expr = self.term()?;

            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right) };
        }
    
        Ok(expr)
    }

    pub fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.factor()?;

        while self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor()?;

            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    pub fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.unary()?;

        while self.match_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;

            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;

            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }

        self.primary()
    }

    pub fn primary(&mut self) -> Result<Expr, ParseError> {
        let next_token: Token = self.advance()?.clone();

        match next_token.token_type {
            TokenType::True => return Ok(Expr::Literal { value: Value::Bool(true) }),
            TokenType::False => return Ok(Expr::Literal { value: Value::Bool(false) }),
            TokenType::Nil => return Ok(Expr::Literal { value: Value::Null }),
            TokenType::Number | TokenType::String => {
                return Ok(Expr::Literal { value: next_token.literal })
            },
            TokenType::LeftParen => {
                let expr: Expr = self.expression()?;
                self.consume(TokenType::RightParen, String::from("Expect ')' to close grouping expression."))?;
                return Ok(Expr::Grouping { expression: Box::new(expr) });
            },
            _ => {}
        }

        Err(self.error(next_token, "Expected expression".to_string()))
    }
} 