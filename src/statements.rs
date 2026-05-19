use std::cell::RefCell;
use std::io;

use crate::environment::Environment;
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
    Var { name: Token, initializer: Option<Box<Expr>> },
}

impl Stmt {
    pub fn execute(stmt: Stmt, output: &mut impl io::Write, env: &mut Environment) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression { expression } => {
                let value = Expr::evaluate(expression.as_ref(), env);
                match value {
                    Ok(_) => {}
                    Err(e) => return Err(e.into()),
                };

                Ok(())
            }
            Stmt::Print { expression } => {
                let value = Expr::evaluate(expression.as_ref(), env);
                match value {
                    Ok(tok) => {
                        // need to handle this lol
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
            Stmt::Var { name, initializer } => {
                match initializer {
                    Some(v) => {
                        let expr_ptr = RefCell::new(Expr::evaluate(v.as_ref(), env)?);
                        env.define(name.lexeme, expr_ptr); 
                    },
                    None => {
                        env.define(name.lexeme, RefCell::new(Literal::Nil));
                    }
                }
                Ok(())
            }
        }
    }

    // how do you write trait bounds???
    pub fn<T: impl io::Write> pretty_print(stmt: Stmt, output: &mut T, env: &mut Environment) -> String {

    }
}
