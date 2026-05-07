use crate::{expressions::Expr, tokens::{Literal, Token, TokenType}};

#[allow(dead_code)]
struct Parser {
    tokens: Vec<Token>,
    current: i32
}

#[allow(dead_code)]
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens, current: 0 }
    }

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {

        let mut expr = Box::new(self.comparison());
        
        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let opr = self.previous();
            let right = Box::new(self.comparison());
            
            expr = Box::new(Expr::Binary {left: expr, operator: opr, right: right })

        }
        
        expr
    }

    fn previous(&mut self) -> Token {
        todo!("")
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true
            }
        }

        false
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            return self.peek().token_type == *token_type
        }
    }

    fn is_at_end(&mut self) -> bool {
        todo!("")
    }

    fn peek(&mut self) -> Token {
        todo!("")
    }

    fn advance(&mut self) {
        todo!("")
    }

    fn comparison(&mut self) -> Expr {
        todo!("")
    }


}
