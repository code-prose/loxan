mod expressions;
mod interpreter;
mod parser;
mod scanner;
mod tokens;
mod statements;

use crate::interpreter::Rlox;

fn main() {
    let mut interp = Rlox::new();
    let _ = interp.main();
}
