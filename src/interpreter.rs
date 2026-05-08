use std::env;
use std::io;
use std::fs;

use crate::expressions::pretty_print;
use crate::parser::Parser;
use crate::tokens::{TokenType, Token, Literal};
use crate::scanner::Scanner;

#[allow(dead_code)]
pub struct Rlox {
    pub had_error: bool
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
        self.run_prompt_on(io::stdin(), io::stdout())
    }

    fn run_prompt_on(&mut self, input: impl io::Read, mut output: impl io::Write) -> io::Result<()> {
        let reader = io::BufReader::new(input);
        for line in io::BufRead::lines(reader) {
            let line: String = line?;
            if line.is_empty() { break }
            let _ = write!(output, "> ");
            output.flush()?;
            self.run(line);
            self.had_error = false;
        }

        Ok(())
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens(self);

        let mut parser = Parser::new(tokens);
        if let Some(expr) = parser.parse() {
            println!("{}", pretty_print(&expr))
        }

        // for token in tokens.iter() {
        //     println!("{token:?}")
        // }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_prompt_on_input() {
        let mut rlox = Rlox::new();
        let input = b"var test = 1;\n";
        let mut output = Vec::new();

        let res = rlox.run_prompt_on(input.as_ref(), &mut output);
        println!("{res:?}");

        assert!(!rlox.had_error);
    }
}
