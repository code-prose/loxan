use crate::expressions::Expr;

pub enum Stmt {
    Expression {
        expression: Box<Expr>
    },
    Print {
        expression: Box<Expr>
    }

}

impl Stmt {
    fn evaluate(&mut self) {

    }
}


