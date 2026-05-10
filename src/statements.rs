use crate::expressions::Expr;

pub enum Stmt {
    Expression {
        expression: Expr
    },
    Print {
        expression: Expr
    }

}

impl Stmt {
    fn evaluate(&mut self) {

    }
}


