use std::env;
use std::io::{self, Write};
use std::fs;

fn read_file_contents(file_path: String) -> Result<String, io::Error> {
    fs::read_to_string(file_path.as_str())
}

fn run(file_contents: String) {
    println!("{}", file_contents);
}

fn run_file(filename: String) {
    let mut contents = read_file_contents(filename);

    match contents {
        Ok(contents) => run(contents),
        Err(e) => println!("{}", e)
    }
}

fn run_prompt() {
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

        run(trimmed);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 2 {
        println!("Usage: sapphire [file]");
    } else if args_len == 2 {
        run_file(args[1].clone());
    } else {
        run_prompt();
    }

    Ok(())
}