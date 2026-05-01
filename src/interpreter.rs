use std::env;
use std::io;
use std::fs;

#[allow(dead_code)]
pub struct Rlox;

#[allow(dead_code)]
impl Rlox {
    pub fn main() ->  io::Result<()> {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
              println!("Usage: jlox [script]");
              std::process::exit(64) 
        } else if args.len() == 1 {
            Rlox::run_file(&args[0]).unwrap()
        } else {
            Rlox::run_prompt().unwrap()
        }
        
        Ok(())
    }

    fn run_file(path: &String) -> io::Result<()> {
        let source = fs::read_to_string(path).unwrap();
        println!("{source:?}");
        Rlox::run(source);

        Ok(())
    }

    fn run_prompt() -> io::Result<()> {
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
