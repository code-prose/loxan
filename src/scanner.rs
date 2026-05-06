use std::collections::HashMap;
use std::sync::LazyLock;

use crate::tokens::{Literal, Token, TokenType};
use crate::interpreter::Rlox;


static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(String::from("and"),    TokenType::And);
    map.insert(String::from("class"),  TokenType::Class);
    map.insert(String::from("else"),   TokenType::Else);
    map.insert(String::from("false"),  TokenType::False);
    map.insert(String::from("for"),    TokenType::For);
    map.insert(String::from("fun"),    TokenType::Fun);
    map.insert(String::from("if"),     TokenType::If);
    map.insert(String::from("nil"),    TokenType::Nil);
    map.insert(String::from("or"),     TokenType::Or);
    map.insert(String::from("print"),  TokenType::Print);
    map.insert(String::from("return"), TokenType::Return);
    map.insert(String::from("super"),  TokenType::Super);
    map.insert(String::from("this"),   TokenType::This);
    map.insert(String::from("true"),   TokenType::True);
    map.insert(String::from("var"),    TokenType::Var);
    map.insert(String::from("while"),  TokenType::While);

    map
});

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
            '!' => {
                let t = if self.match_token('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(t, None);
            },
            '=' => {
                let t = if self.match_token('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(t, None);
            },
            '>' => {
                let t = if self.match_token('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(t, None);
            },
            '<' => {
                let t = if self.match_token('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(t, None);
            },
            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            },
            ' ' => {},
            '\t' => {},
            '\r' => {},
            '\n' => {
                self.line += 1;
            },
            '"' => self.string(interp),
            'o' => {
                if self.match_token('r') {
                    self.add_token(TokenType::Or, None);
                }
            },
            _ => {
                if self.is_digit(character) {
                    self.number();
                } else if self.is_alpha(character) {
                    self.identifier();
                } else {
                    Rlox::error(interp, self.line, String::from("Unexpected character: {character}"));
                }
            },
        };

        todo!("")
    }

    fn is_alpha(&mut self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') ||
               (c >= 'A' && c <= 'Z') ||
                c == '_'
    }

    fn is_alphanumeric(&mut self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c)
    }

    fn identifier(&mut self) {
        let _c = self.peek();
        while self.is_alpha(_c) {
            self.advance();
            let _c = self.peek();
        }

        let slice = (&self.source[self.start..self.current]).to_vec();
        let text = String::from_utf8(slice).unwrap();

        let token: TokenType = match KEYWORDS.get(&text) {
            Some(t) => t.clone(),
            None => TokenType::Identifier
        };

        self.add_token(token, None);
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0'
        } else {
            self.source[self.current] as char
        }
    }

    fn is_digit(&mut self, c: char) -> bool {
        return c >= '0' && c <= '9'
    }


    fn number(&mut self) {
        let _c = self.peek();
        while self.is_digit(_c) {
            self.advance();
            let _c = self.peek();
        }
        let next = self.peek_next();

        if self.peek() == '.' && self.is_digit(next) {
            self.advance();

            let _c = self.peek();
            while self.is_digit(_c) {
                self.advance();
                let _c = self.peek();
            }
        }

        let slice = (&self.source[self.start..self.current]).to_vec();
        let temp_string = String::from_utf8(slice).unwrap();
        let value = Some(
            Literal::Number(str::parse::<f64>(&temp_string).unwrap())
        );

        self.add_token(TokenType::Number, value);

    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0'
        } else {
            return self.source[self.current + 1] as char
        }
    }

    fn string(&mut self, interp: &mut Rlox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Rlox::error(interp, self.line, String::from("Unterminated string."));
            return
        }

        self.advance();

        let slice = &self.source[self.start - 1..self.current + 1];
        let value = Some(
            Literal::Str(
                String::from_utf8(slice.to_vec()).unwrap()
            )
        );
        self.add_token(TokenType::String, value);
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] as char != expected {
            false
        } else {
            self.current += 1;
            true
        }
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
