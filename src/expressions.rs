use crate::tokens::{Literal, Token};

#[allow(dead_code)]
#[derive(Debug, Clone)]
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

#[allow(dead_code)]
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenType;

    #[test]
    fn test_pretty_print() {
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
        assert_eq!(String::from("(* (- 123) (group 45.67))"), pretty_print(&expression));

    }
}
