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
        todo!("")
    }

    fn comparison(&mut self) -> Expr {
        todo!("")
    }


}
