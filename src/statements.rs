use std::io;

use crate::expressions::{EvaluationError, Expr};
use crate::tokens::{Literal, Token};

pub struct RuntimeError {
    pub line_no: usize,
    pub message: String,
}

impl From<EvaluationError> for RuntimeError {
    fn from(err: EvaluationError) -> Self {
        RuntimeError {
            line_no: err.line_no,
            message: err.message,
        }
    }
}

pub enum Stmt {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
    Var { name: Token, initializer: Box<Expr> },
}

impl Stmt {
    pub fn execute(stmt: Stmt, output: &mut impl io::Write) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression { expression } => {
                let value = Expr::evaluate(expression.as_ref());
                match value {
                    Ok(_) => {}
                    Err(e) => return Err(e.into()),
                };

                Ok(())
            }
            Stmt::Print { expression } => {
                let value = Expr::evaluate(expression.as_ref());
                match value {
                    Ok(tok) => {
                        match tok {
                            Literal::Number(n) => writeln!(output, "{}", n),
                            Literal::Bool(b) => writeln!(output, "{}", b),
                            Literal::Str(s) => writeln!(output, "{}", s),
                            Literal::Nil => writeln!(output, "{}", "nil"),
                        };
                    }
                    Err(e) => return Err(e.into()),
                }
                Ok(())
            },
            Stmt::Var { name, initializer } =>  {
                todo!("")
            }

        }
    }
}
