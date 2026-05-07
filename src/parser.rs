use crate::{
    expressions::Expr,
    tokens::{Literal, Token, TokenType},
};

#[allow(dead_code)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let opr = self.previous();
            let right = self.comparison();

            expr = Box::new(Expr::Binary {
                left: expr,
                operator: opr,
                right: right,
            })
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr: Box<Expr> = self.term();

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let opr: Token = self.previous();

            let right: Box<Expr> = self.term();
            expr = Box::new(Expr::Binary {
                left: expr,
                operator: opr,
                right: right,
            });
        }

        expr
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let opr = self.previous();
            let right = self.factor();

            expr = Box::new(Expr::Binary { left: expr, operator: opr, right: right });
        }


        expr
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut expr = self.unary();
        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let opr = self.previous();
            let right = self.unary();

            expr = Box::new(Expr::Binary { left: expr, operator: opr, right: right});
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        todo!("")
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            return self.peek().token_type == *token_type;
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
