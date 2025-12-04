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

        let previous: Token = self.previous().clone();
        Err(ParseError::new("Expected token, got EOF"))
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
        let mut stmt: Result<Stmt, ParseError>;

        if self.match_types(vec![TokenType::Var]) {
            stmt = self.var_declaration();
        } else {
            stmt = self.statement();
        }

        match stmt {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }
    }

    pub fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name: Token = self.consume(TokenType::Identifier, "Expected variable name.".to_owned())?.clone();

        let mut initializer: Expr = Expr::Literal { value: Value::Null };

        let matched: bool = self.match_types(vec![TokenType::Equal]);
        if matched {
            initializer = self.expression()?.clone();
        }

        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration.".to_owned())?;

        Ok(Stmt::Var { name: name, initializer: initializer })
    }

    pub fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_types(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.match_types(vec![TokenType::LeftBrace]) {
            self.block_statement()
        } else if self.match_types(vec![TokenType::If]) {
            self.if_statement()
        } else if self.match_types(vec![TokenType::While]) {
            self.while_statement()
        } else if self.match_types(vec![TokenType::Fn]) {
            self.function_statement()
        } else {
            self.expression_statement()
        }
    }

    pub fn function_statement(&mut self) -> Result<Stmt, ParseError> {
        let fn_name: Token = self.advance()?.clone();
        self.consume(TokenType::LeftParen, "Expected LEFT_PAREN after function name".to_owned())?;

        let mut params: Vec<Token> = vec![];
        if !self.match_types(vec![TokenType::RightParen]) {
            let mut matched: bool = true;

            while matched {
                params.push(self.advance()?.clone());
                matched = self.match_types(vec![TokenType::Comma]);
            }
        }

        Ok(Stmt::Function { name: fn_name, params, body: Box::new(self.statement()?) })
    }

    pub fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        let condition: Expr = self.expression()?;
        let then_branch: Stmt = self.statement()?;
        let mut else_branch: Option<Box<Stmt>> = Option::None;

        if self.match_types(vec![TokenType::Else]) {
            else_branch = Some(Box::new(
                self.statement()?
            ));
        }

        Ok(Stmt::If { condition, then_branch: Box::new(then_branch), else_branch })
    }

    pub fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        let condition: Expr = self.expression()?;
        let body: Box<Stmt> = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    pub fn block_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' to close block.".to_owned())?;
        
        Ok(Stmt::Block { statements })
    }

    pub fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Exepcted ; to follow value".to_owned())?;

        Ok(Stmt::Print { expression: value })
    }

    pub fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Exepcted ; to follow value".to_owned())?;

        Ok(Stmt::Expression { expression: value })
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.shorthand_if()
    }

    pub fn shorthand_if(&mut self) -> Result<Expr, ParseError> {
        let expr: Expr = self.assignment()?;

        if self.match_types(vec![TokenType::QuestionMark]) {
            let if_true: Expr = self.shorthand_if()?;
            self.consume(TokenType::Colon, "Expected ':' to seperate true from false clause in shorthand-if".to_owned())?;
            let if_false: Expr = self.shorthand_if()?;

            return Ok(Expr::Ternary { condition: Box::new(expr), if_true: Box::new(if_true), if_false: Box::new(if_false) });
        }

        Ok(expr)
    }

    pub fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr: Expr = self.logical()?;

        if self.match_types(vec![TokenType::Equal]) {
            let equals: Token = self.previous().clone();
            let value: Expr = self.assignment()?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assign { name, value: Box::new(value) });
                }
                _ => return Err(self.error(equals, "Invalid assignment target.".to_owned()))
            }
        }

        Ok(expr)
    }
    
    pub fn logical(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.equality()?;

        while self.match_types(vec![TokenType::DoubleAmp, TokenType::DoublePipe]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.equality()?;

            expr = Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) };
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

            Ok(Expr::Unary { operator, right: Box::new(right) })
        } else {
            self.index()
        }
    }

    pub fn index(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.list()?;

        if self.match_types(vec![TokenType::LeftBracket]) {
            let bracket: Token = self.previous().clone();
            let index: Expr = self.expression()?;

            self.consume(TokenType::RightBracket, "Expected ] to close Index statement".to_owned())?;

            expr = Expr::Index { indexee: Box::new(expr), bracket, index: Box::new(index) };
        }

        Ok(expr)
    }

    pub fn list(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(vec![TokenType::LeftBracket]) {
            let mut list: Vec<Expr> = vec![];
            while !self.match_types(vec![TokenType::RightBracket]) {
                list.push(self.expression()?);
    
                self.match_types(vec![TokenType::Comma]);
            }
    
            Ok(Expr::List { expressions: list })
        } else {
            Ok(self.call()?)
        }
    }

    pub fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.primary()?;

        loop {
            if self.match_types(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    pub fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut args: Vec<Expr> = vec![];

        if !self.check(TokenType::RightParen) {
            let mut matched = true;

            while matched {
                args.push(self.expression()?);
                matched = self.match_types(vec![TokenType::Comma]);
            }
        }

        let paren: Token = self.consume(TokenType::RightParen, "Expected ')' after arguments.".to_owned())?.clone();
        Ok(Expr::Call { callee: Box::new(callee), paren, arguments: args })
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

        Err(self.error(next_token, "Expected expression".to_owned()))
    }
}