use std::env;
use std::io::{self, Write};
use std::fs;

mod scanner;
mod token;
mod token_type;
mod value;

struct Sapphire {
    pub had_error: bool,
}

impl Sapphire {
    fn read_file_contents(&self, file_path: String) -> Result<String, io::Error> {
        fs::read_to_string(file_path.as_str())
    }
    
    fn run(&mut self, file_contents: String) {
        let mut scan = scanner::Scanner::new(self, file_contents);
        let tokens = scan.scan_tokens();
        
        for token in tokens {
            println!("{}", token);
        }
    }
    
    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, String::from(""), message);
    }
    
    pub fn report(&mut self, line: usize, where_at: String, message: String) {
        println!("[line {line}] Error{where_at}: {message}");
        self.had_error = true;
    }
    
    fn run_file(&mut self, filename: String) {
        let mut contents = self.read_file_contents(filename);
    
        match contents {
            Ok(contents) => self.run(contents),
            Err(e) => println!("{}", e)
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
    let mut sapphire: Sapphire = Sapphire { had_error: false };

    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 2 {
        println!("Usage: sapphire [file]");
    } else if args_len == 2 {
        sapphire.run_file(args[1].clone());
    } else {
        sapphire.run_prompt();
    }

    Ok(())
}