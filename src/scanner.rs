use crate::tokens::{Literal, Token, TokenType};

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.into_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        // figure this out
        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            Some(Literal::Nil),
            self.line,
        ));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let character: char = self.advance();
        match character {
            '(' => self.add_token(TokenType::RightParen, None),
            ')' => self.add_token(TokenType::LeftParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            _ => std::process::exit(100),
        };

        todo!("")
    }

    fn advance(&mut self) -> char {
        let character = self.source[self.current];
        self.current += 1;
        character as char
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        todo!("")
    }
}
