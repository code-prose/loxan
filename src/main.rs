mod ast;
mod expressions;
mod interpreter;
mod parser;
mod scanner;
mod tokens;

use crate::interpreter::Rlox;

fn main() {
    let mut interp = Rlox::new();
    let _ = interp.main();
}
