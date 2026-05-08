use crate::{
    expressions::Expr,
    tokens::{Literal, Token, TokenType},
};

#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// I should let peek_token be an Option<_> and then I can handle the cases where I don't have a
// token to raise with
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ParsingError {
    peek_token: Token,
    message: String
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Box<Expr>> {
        match self.expression() {
            Ok(v) => return Some(v),
            Err(_) => {
                println!("Couldn't parse expression");
                None
            }
        }
    }

    fn error(err: ParsingError) {
        match err.peek_token.token_type {
            TokenType::EOF => {
                Parser::report(err.peek_token.line, String::from(" at end"), err.message);
            },
            _ => {
                let lexeme = err.peek_token.lexeme;
                Parser::report(err.peek_token.line, format!(" at '{lexeme}'"), err.message)
            }
        }
    }

    fn report(line: usize, loc: String, message: String) {
        println!("[line {line}] Error {loc}: {message}");
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return
            }

            match self.peek().token_type {
                TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }

    }

    // Maybe: ternary operator ?: support

    // C style comma expressions support
    fn comma(&mut self) -> Result<Box<Expr>, ParsingError> {
        let mut expr = self.equality()?;

        while self.match_tokens(&[TokenType::Comma]) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right
            });
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParsingError> {
        self.comma()
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParsingError> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParsingError> {
        let mut expr: Box<Expr> = self.term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous();

            let right: Box<Expr> = self.term()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ParsingError> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Box::new(Expr::Binary { left: expr, operator, right });
        }


        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParsingError> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Box::new(Expr::Binary { left: expr, operator, right});
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParsingError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary { operator, expr: right }))
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParsingError> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Box::new(Expr::Literal { literal: Literal::Bool(false) }))
        }
        if self.match_tokens(&[TokenType::True]) {
            return Ok(Box::new(Expr::Literal { literal: Literal::Bool(true) }))
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal { literal: Literal::Nil }))
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            match self.previous().literal {
                Some(v) => return Ok(Box::new(Expr::Literal { literal: v })),
                None => {
                    println!("Expected to find a literal");
                    std::process::exit(101)
                }
            }
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, String::from("Expected ')' after expression."))?;
            return Ok(Box::new(Expr::Grouping { expr: expr }))
        }

        let err = ParsingError{peek_token: self.peek(), message: String::from("Expected an expression")};
        Parser::error(err.clone());
        Err(err)
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, ParsingError> {
        if self.check(&token_type) {
            return Ok(self.advance())
        }

        Err(ParsingError{ peek_token: self.peek(), message: message})
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

    fn check(&self, token_type: &TokenType) -> bool {
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

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
