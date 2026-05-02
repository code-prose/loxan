use std::env;
use std::io;
use std::fs;

use crate::tokens::{TokenType, Token, Literal};

#[allow(dead_code)]
pub struct Rlox {
    had_error: bool
}

#[allow(dead_code)]
impl Rlox {

    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn main(&mut self) ->  io::Result<()> {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
              println!("Usage: jlox [script]");
              std::process::exit(64) 
        } else if args.len() == 1 {
            self.run_file(&args[0]).unwrap()
        } else {
            self.run_prompt().unwrap()
        }
        
        Ok(())
    }

    fn run_file(&self, path: &String) -> io::Result<()> {
        let source = fs::read_to_string(path).unwrap();
        println!("{source:?}");
        Rlox::run(source);

        if self.had_error { std::process::exit(65) }

        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let mut input = String::new();
        loop {
            println!("> ");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {},
                Err(e) => {
                    println!("Could not interpret line: {e}");
                    break
                }
            };
            if input == "" { break }
            else { Rlox::run(input.clone()) };
            self.had_error = false
        }

        Ok(())

    }

    fn run(source: String) {
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scanTokens();

        for token in tokens.iter() {
            println!("{token}")
        }
    }
}
