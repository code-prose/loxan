mod interpreter;
mod tokens;
mod scanner;

use crate::interpreter::Rlox;

fn main() {
    println!("Hello, world!");
    let mut interp = Rlox::new();
    let _ = interp.main();

}
