use crate::tokens::{Literal, Token, TokenType};
use crate::interpreter::Rlox;

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

    pub fn scan_tokens(&mut self, interp: &mut Rlox) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(interp);
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

    fn scan_token(&mut self, interp: &mut Rlox) {
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
            _ => {
                Rlox::error(interp, self.line, String::from("Unexpected character: {character}"));
            },
        };

        todo!("")
    }

    fn advance(&mut self) -> char {
        let character = self.source[self.current];
        self.current += 1;
        character as char
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = String::from_utf8_lossy(&self.source[self.start..self.current]).to_string();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }
}
