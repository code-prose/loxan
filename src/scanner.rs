use crate::tokens::{Token, TokenType, Literal};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source: source, tokens: Vec::new(), start: 0, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        // figure this out
        self.tokens.push(
            Token::new(TokenType::EOF,
                String::from(""),
                Some(Literal::Nil),
                self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current > self.source.len()
    }

    fn scan_token(&self) {
        todo!("")
    }
}
