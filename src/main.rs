mod interpreter;
mod tokens;
mod scanner;
mod expressions;
mod ast;

use crate::interpreter::Rlox;

fn main() {
    let mut interp = Rlox::new();
    let _ = interp.main();

}
