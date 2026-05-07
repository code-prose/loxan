use crate::tokens::{Token, TokenType, Literal};

enum Expr {
    Literal {
       literal: Literal 
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Unary {
        operator: Token,
        expr: Box<Expr>

    },
    Grouping {
        expr: Box<Expr>
    },
}
