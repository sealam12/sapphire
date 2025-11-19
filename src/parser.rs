use crate::error::ParseError;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;
use crate::expr::Expr;
use crate::stmt::Stmt;
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
                    TokenType::Print | TokenType::Return => return,
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    pub fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_types(vec![TokenType::Var]) {
            match self.var_declaration() {
                Ok(stmt) => return Ok(stmt),
                Err(err) => {
                    self.synchronize();
                    return Err(err)
                }
            }
        }


        match self.statement() {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }
    }

    pub fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name: Token = self.consume(TokenType::Identifier, "Expected variable name.".to_string())?.clone();

        let mut initializer: Expr = Expr::Literal { value: Value::Null };

        let matched: bool = self.match_types(vec![TokenType::Equal]);
        if matched {
            initializer = self.expression()?.clone();
        }

        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration.".to_string())?;

        Ok(Stmt::Var { name: name, initializer: initializer })
    }

    pub fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_types(vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    pub fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Exepcted ; to follow value".to_string())?;

        Ok(Stmt::Print { expression: value })
    }

    pub fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Exepcted ; to follow value".to_string())?;

        Ok(Stmt::Expression { expression: value })
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    pub fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr: Expr = self.equality()?;

        if self.match_types(vec![TokenType::Equal]) {
            let equals: Token = self.previous().clone();
            let value: Expr = self.assignment()?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assign { name, value: Box::new(value) });
                }
                _ => return Err(self.error(equals, "Invalid assignment target.".to_string()))
            }
        }

        Ok(expr)
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

        while self.match_types(vec![TokenType::Minus, TokenType::Plus, TokenType::DoubleDot]) {
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
                return Ok(Expr::Literal { value: next_token.literal });
            },
            TokenType::Identifier => {
                return Ok(Expr::Variable { name: self.previous().clone() });
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