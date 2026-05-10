use crate::expressions::EvaluationError;
use crate::{expressions::Expr, tokens::Literal};
use std::io;


struct RuntimeError {
    line_no: usize,
    message: String
}

impl From<EvaluationError> for RuntimeError {
    fn from(err: EvaluationError) -> Self {
        RuntimeError {
            line_no: err.line_no,
            message: err.message
        }
    }
}

pub enum Stmt {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
}

impl Stmt {
    pub fn evaluate(stmt: Stmt, output: &mut impl io::Write) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression { expression } => {
                let value = Expr::evaluate(expression.as_ref());
                match value {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(e.into())
                    }
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
                    },
                    Err(e) => {
                        return Err(e.into())
                    }
                }
                Ok(())
            }
        }
    }
}
