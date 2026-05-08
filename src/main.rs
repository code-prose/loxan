mod interpreter;
mod tokens;
mod scanner;
mod expressions;
mod ast;
mod parser;

use crate::interpreter::Rlox;
use crate::expressions::{Expr, pretty_print};
use crate::tokens::{Token, TokenType, Literal};
use crate::scanner::Scanner;
use crate::parser::Parser;

// fn main() {
//     let mut interp = Rlox::new();
//     let _ = interp.main();
// }


fn main () {
    let mut interp = Rlox::new();
    let source = String::from("var test = \"123\";");
    let mut scanner = Scanner::new(source); 

    let tokens = scanner.scan_tokens(&mut interp);

    let mut parser = Parser::new(tokens);

    let expr = match parser.parse() {
        Some(v) => v,
        None => std::process::exit(127)
    };

    println!("{expr:?}")
    
}

