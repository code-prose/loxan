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
        operator: Option<Token>,
        expr: Box<Expr>

    },
    Grouping {
        // left paren
        left: TokenType,
        // right paren
        right: TokenType,
        expr: Box<Expr>
    },
}
