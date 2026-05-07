mod interpreter;
mod tokens;
mod scanner;
mod expressions;
mod ast;
mod parser;

use crate::interpreter::Rlox;
use crate::expressions::{Expr, pretty_print};
use crate::tokens::{Token, TokenType, Literal};

fn main() {
    let mut interp = Rlox::new();
    let _ = interp.main();
}

