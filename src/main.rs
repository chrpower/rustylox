mod scanner;
mod token;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rustylox [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) {
    match std::fs::read_to_string(path) {
        Ok(contents) => {
            run(&contents);
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            std::process::exit(1);
        }
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                run(input.trim());
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run(source: &str) {
    match scanner::scan_tokens(source) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            std::process::exit(1);
        }
    }
}
