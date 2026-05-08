use crate::tokens::{self, Literal, Token, TokenType};

pub struct EvaluationError {
    line_no: usize,
    message: String
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Literal {
        literal: Literal,
    },
    Ternary {
        left: Box<Expr>,
        operator: Token,
        middle: Box<Expr>,
        right: Box<Expr>
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

    pub fn evaluate(expr: &Expr) -> Result<Literal, EvaluationError> {
        match expr {
            Expr::Literal { literal } => Ok(literal.clone()),
            Expr::Unary { operator, expr } => {
                let right = Self::evaluate(expr)?;
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
                        Err(EvaluationError { line_no: operator.line, message: String::from("Failed to parse unary expression on line: {line_no}") })
                    },
                }
            },
            Expr::Binary { left, operator, right } => {
                let right = Self::evaluate(right.as_ref())?;
                let left = Self::evaluate(left.as_ref())?;

                match operator.token_type {
                    TokenType::Star => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Number(a * b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operand {}", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Slash => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Number(a / b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operand {}", operator.lexeme) })
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
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operand {}", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Minus => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Number(a - b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operand {}", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::GreaterEqual => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a >= b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operand {}", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Greater => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a > b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operand {}", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::LessEqual => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a <= b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operand {}", operator.lexeme) })
                            }
                        }
                    },
                    TokenType::Less => {
                        match (left, right) {
                            (Literal::Number(a), Literal::Number(b)) => {
                                Ok(Literal::Bool(a < b))
                            },
                            (_, _) => {
                                Err(EvaluationError { line_no: operator.line, message: format!("Cannot evaluate binary expression with operand {}", operator.lexeme) })
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
                    _ => {
                        Err(
                            EvaluationError {
                                line_no: operator.line,
                                message: format!("Failed to parse binary expression with operand {}", operator.lexeme)
                            }
                        )
                    }
                }
            },
            Expr::Ternary { left, middle, right, .. } => {
                let left = Self::evaluate(left)?;

                if Self::is_truthy(&left) {
                    Self::evaluate(middle)
                } else {
                    Self::evaluate(right)
                }
            },
            Expr::Grouping { expr } => Self::evaluate(expr),
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
