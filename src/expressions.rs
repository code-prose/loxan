use crate::tokens::{Literal, Token, TokenType};

pub enum Expr {
    Literal {
        literal: Literal,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        expr: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

pub fn pretty_print(expr: &Expr) -> String {
    match expr {
        Expr::Grouping { expr } => format!("(group {})", pretty_print(expr)),
        Expr::Unary { operator, expr } => format!("({} {})", operator.lexeme, pretty_print(expr)),
        Expr::Binary { left, operator, right } => {
            format!("({} {} {})", operator.lexeme, pretty_print(left), pretty_print(right))
        },
        Expr::Literal { literal } => {
            match literal {
                Literal::Number(n) => format!("{n}"),
                Literal::Str(s) => format!("{s}"),
                Literal::Bool(b) => format!("{b}"),
                Literal::Nil => String::from("nil")
            }
        },
    }
}
