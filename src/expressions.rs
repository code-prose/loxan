use crate::{environment::Environment, statements::RuntimeError, tokens::{Literal, Token, TokenType}};

#[allow(dead_code)]
pub struct EvaluationError {
    pub line_no: usize,
    pub message: String
}

impl From<RuntimeError> for EvaluationError  {
    fn from(value: RuntimeError) -> Self {
       EvaluationError { line_no: value.line_no, message: value.message } 
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Literal {
        literal: Literal,
    },
    Unary {
        operator: Token,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Ternary {
        left: Box<Expr>,
        operator: Token,
        middle: Box<Expr>,
        right: Box<Expr>
    },
    Grouping {
        expr: Box<Expr>,
    },
    Variable {
        name: Token
    },
    Assign {
        name: Token,
        expr: Box<Expr>
    },
}


#[allow(dead_code)]
impl Expr {
    pub fn pretty_print(expr: &Expr) -> String {
        match expr {
            Expr::Grouping { expr } => format!("(group {})", Self::pretty_print(expr)),
            Expr::Unary { operator, expr } => format!("({} {})", operator.lexeme, Self::pretty_print(expr)),
            Expr::Binary { left, operator, right } => {
                format!("({} {} {})", operator.lexeme, Self::pretty_print(left), Self::pretty_print(right))
            },
            Expr::Ternary { left, operator, middle, right } => {
                format!("({} ? {} {} {})", Self::pretty_print(left), Self::pretty_print(middle), operator.lexeme, Self::pretty_print(right))
            },
            Expr::Literal { literal } => Self::pretty_print_literal(literal),
            Expr::Variable { name } => {
                match &name.literal {
                    Some(v) => Self::pretty_print_literal(&v),
                    None => String::from("How did you get here?")
                }
            }
            Expr::Assign { name, expr } => {
                format!("{}: {}", name.lexeme, Self::pretty_print(expr))
            }
        }
    }

    pub fn pretty_print_literal(literal: &Literal) -> String {
        match literal {
            Literal::Number(n) => format!("{n}"),
            Literal::Str(s) => format!("{s}"),
            Literal::Bool(b) => format!("{b}"),
            Literal::Nil => String::from("nil")
        }
    }

    pub fn evaluate(expr: &Expr, env: &mut Environment) -> Result<Literal, EvaluationError> {
        match expr {
            Expr::Literal { literal } => Ok(literal.clone()),
            Expr::Unary { operator, expr } => {
                let right = Self::evaluate(expr, env)?;
                match operator.token_type {
                    TokenType::Minus => {
                        match right {
                            Literal::Number(n) => {
                                Ok(Literal::Number(n * -1.0))
                            },
                            _ => {
                                Err(EvaluationError { line_no: operator.line, message: String::from("Cannot negate non-number value") })
                            },
                        }
                    },
                    TokenType::Bang => Ok(Literal::Bool(!Self::is_truthy(&right))),
                    _ => {
                        Err(EvaluationError { line_no: operator.line, message: String::from("Failed to parse unary expression, operand must be a boolean") })
                    },
                }
            },
            Expr::Binary { left, operator, right } => {
                let right = Self::evaluate(right.as_ref(), env)?;
                let left = Self::evaluate(left.as_ref(), env)?;

                match operator.token_type {
                    TokenType::Star => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Number(a * b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operator {} on non-numbers", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Slash => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                if b == 0.0 {
                                    Err(EvaluationError { line_no: operator.line, message: String::from("Division by 0") })
                                } else {
                                    Ok(Literal::Number(a / b))
                                }
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operator {} on non-numbers", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Plus => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Number(a + b))
                            },
                            (Literal::Str(s), Literal::Str(t)) => {
                                Ok(Literal::Str(s + &t))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Operator {} must be used on numbers or strings", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Minus => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Number(a - b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operator {} on non-numbers", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::GreaterEqual => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a >= b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operator {} on non-numbers", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Greater => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a > b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operator {} on non-numbers", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::LessEqual => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a <= b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operator {} on non-numbers", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Less => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a < b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operator {} on non-numbers", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::EqualEqual => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a == b))
                            },
                            (Literal::Bool(a), Literal::Bool(b)) => {
                                Ok(Literal::Bool(a == b))
                            },
                            (Literal::Str(a), Literal::Str(b)) => {
                                Ok(Literal::Bool(a == b))
                            },
                            (Literal::Nil, Literal::Nil) => {
                                Ok(Literal::Bool(true))
                            },
                            (Literal::Nil, _) => {
                                Ok(Literal::Bool(false))
                            },
                            (_, _) => {
                                Ok(Literal::Bool(false))
                            }
                        }
                    },
                    TokenType::BangEqual => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a != b))
                            },
                            (Literal::Bool(a), Literal::Bool(b)) => {
                                Ok(Literal::Bool(a != b))
                            },
                            (Literal::Str(a), Literal::Str(b)) => {
                                Ok(Literal::Bool(a != b))
                            },
                            (Literal::Nil, Literal::Nil) => {
                                Ok(Literal::Bool(false))
                            },
                            (Literal::Nil, _) => {
                                Ok(Literal::Bool(true))
                            },
                            (_, _) => {
                                Ok(Literal::Bool(true))
                            }
                        }
                    },
                    TokenType::Comma => {
                        Ok(right)
                    },
                    _ => {
                        Err(
                            EvaluationError {
                                line_no: operator.line,
                                message: format!("Failed to parse binary expression with operator {}", operator.lexeme)
                            }
                        )
                    }
                }
            },
            Expr::Ternary { left, middle, right, .. } => {
                let left = Self::evaluate(left, env)?;

                if Self::is_truthy(&left) {
                    Self::evaluate(middle, env)
                } else {
                    Self::evaluate(right, env)
                }
            },
            Expr::Grouping { expr } => Self::evaluate(expr, env),
            Expr::Variable { name } => {
                let literal_ptr = env.get(&name.lexeme)?;
                let tok = literal_ptr.borrow().clone();
                Ok(tok)
            },
            Expr::Assign { name, expr } => {
                todo!()
            }
        }

    }

    fn is_truthy(literal: &Literal) -> bool {
        match literal {
            Literal::Nil => false,
            Literal::Bool(b) => *b,
            _ => true
        }
    }

}

#[allow(dead_code)]


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
        assert_eq!(String::from("(* (- 123) (group 45.67))"), Expr::pretty_print(&expression));

    }
}
