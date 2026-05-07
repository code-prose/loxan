mod interpreter;
mod tokens;
mod scanner;
mod expressions;
mod ast;

use crate::interpreter::Rlox;
use crate::expressions::{Expr, pretty_print};
use crate::tokens::{Token, TokenType, Literal};

// fn main() {
//     let mut interp = Rlox::new();
//     let _ = interp.main();
// }


fn main() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, String::from("-"), None, 1),
            expr: Box::new(Expr::Literal { literal: Literal::Number(123.0) }),
        }),
        operator: Token::new(TokenType::Star, String::from("*"), None, 1),
        right: Box::new(Expr::Grouping {
            expr: Box::new(Expr::Literal { literal: Literal::Number(45.67) })
        }

        ),
    };
    println!("{}", pretty_print(&expression));
}
