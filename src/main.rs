use std::env;
use std::io::{self, Write};
use std::fs;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::token_type::TokenType;
use crate::error::RuntimeError;
use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::token::Token;
use crate::stmt::Stmt;

mod environment;
mod interpreter;
mod token_type;
mod astprinter;
mod variable;
mod scanner;
mod parser;
mod token;
mod value;
mod error;
mod expr; 
mod stmt;

struct Sapphire {
    pub had_error: bool,
    pub had_runtime_error: bool,
    pub environment: Environment,
}

impl Sapphire {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
            environment: Environment::new(Option::None)
        }
    }

    fn read_file_contents(&self, file_path: String) -> Result<String, io::Error> {
        fs::read_to_string(file_path.as_str())
    }
    
    fn run(&mut self, contents: String) {
        let mut scanner: Scanner<'_> = Scanner::new(self, contents);
        let tokens: Vec<Token> = scanner.scan_tokens().clone();

        if self.had_error {
            return;
        }

        let mut parser: Parser = Parser::new(self, tokens);

        let statements: Vec<Stmt>;
        match parser.parse() {
            Ok(stmts) => statements = stmts.clone(),
            Err(_) => return
        }

        if self.had_error {
            return;
        }
        
        let mut interpreter: Interpreter = Interpreter::new(self);
        interpreter.interpret(&statements);
    }

    pub fn runtime_error(&mut self, error: RuntimeError) {
        println!("[line {}] [RuntimeError] {}", error.line, error.message);
        self.had_runtime_error = true;
    }
    
    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    pub fn token_error(&mut self, token: Token, message: String) {
        match token.token_type {
            TokenType::EOF => self.report(token.line, " at end".to_string(), message),
            _ => self.report(token.line, " at '".to_string() + token.lexeme.as_str() + "'", message)
        }
    }
    
    pub fn report(&mut self, line: usize, where_at: String, message: String) {
        println!("[line {line}] [Error{where_at}]: {message}");
        self.had_error = true;
    }
    
    fn run_file(&mut self, filename: String) {
        let contents = self.read_file_contents(filename);
    
        match contents {
            Ok(contents) => self.run(contents),
            Err(_) => println!("There was an error reading the file.")
        }

        if self.had_error {
            println!("Exiting with error.");
        } else if self.had_runtime_error {
            println!("Exiting with runtime error.");
        }
    }
    
    fn run_prompt(&mut self) {
        loop {
            let mut contents: String = String::new();
    
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");
    
            io::stdin()
                .read_line(&mut contents)
                .expect("Failed to read line");
            
            let trimmed = contents.trim().to_string();
            if trimmed == "q" {
                break;
            }
    
            self.run(trimmed);
            self.had_error = false;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut sapphire: Sapphire = Sapphire::new();

    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();

    if args_len > 2 {
        println!("Usage: sapphire [file]");
    } else if args_len == 2 {
        sapphire.run_file(args[1].clone());
    } else {
        sapphire.run_prompt();
    }

    Ok(())
}