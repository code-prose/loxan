use std::env;
use std::io;
use std::fs;
use std::io::stdout;
use std::io::Write;

use crate::tokens::{TokenType, Token, Literal};
use crate::scanner::Scanner;

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

        if args.len() > 2 {
              println!("Usage: jlox [script]");
              std::process::exit(64) 
        } else if args.len() == 2 {
            self.run_file(&args[0]).unwrap()
        } else {
            self.run_prompt().unwrap()
        }
        
        Ok(())
    }

    fn run_file(&mut self, path: &String) -> io::Result<()> {
        let source = fs::read_to_string(path).unwrap();
        Rlox::run(self, source);

        if self.had_error { std::process::exit(65) }

        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let mut input = String::new();
        loop {
            let _ = stdout().flush();
            println!("> ");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {},
                Err(e) => {
                    println!("Could not interpret line: {e}");
                    break
                }
            };
            if input == "" { break }
            else { Rlox::run(self, input.clone()) };
            self.had_error = false
        }

        Ok(())

    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens(self);

        for token in tokens.iter() {
            println!("{token:?}")
        }
    }

    // Perhaps something like this in the future
    // Error: Unexpected "," in argument list.
    //
    //     15 | function(first, second,);
    //                                ^-- Here.
    pub fn report(&mut self, line: usize, loc: String, message: String) {
        println!("[line {line}] Error {loc}: {message}");
        self.had_error = true;
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, String::from(""), message);
    }


}
