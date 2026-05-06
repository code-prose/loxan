use crate::tokens::Token;

enum Expr {
    Literal {
        
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Unary {

    },
    Grouping {

    },
}
