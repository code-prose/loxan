use std::env;
use std::io;
use std::io::Read;
use std::fs;

#[allow(dead_code)]
pub struct Rlox;

impl Rlox {
    pub fn main() ->  io::Result<()> {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
              println!("Usage: jlox [script]");
              std::process::exit(64) 
        } else if args.len() == 1 {
            runFile(args[0]);
        } else {
            runPrompt();
        }
        
        Ok(())
    }

    fn runFile(path: String) -> io::Result<()> {
        let source = fs::read_to_string(path).unwrap();
        println!("{source:?}");
        Rlox::run(source);

        Ok(())
    }

    fn runPrompt() -> io::Result<()> {
        let mut input = String::new();

        // fix this fuckery
        loop {
            println!("> ");
            io::stdin().read_line(&mut input);
            let input = match input {
                Ok(line) => line,
                Err(e) => {
                    println!("Could not interpret line: {e}");
                    break
                } 
            }
        }

        Ok(())

    }

    fn run(source: String) {

    }
}
