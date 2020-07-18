// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::parsers::{KeyWord, Token, TokenType, Tokens};

pub struct Lexer {
    pub text: String,
    pos: usize,
    end: usize,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        let size = text.len();
        Lexer {
            text,
            pos: 0,
            end: size,
        }
    }

    pub fn parse(&mut self) -> Tokens {
        let mut tokens = Tokens::default();

        while let Some(token) = self.next_token() {
            if token.is_significant() {
                tokens.tokens.push(token);
            }
        }
        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if !self.next() {
            return None;
        }

        let c = self.text.as_bytes()[self.pos];
        match c as char {
            ' ' | '\r' | '\n' => {
                let mut token = Token::default(TokenType::WhiteSpace);
                token.begin = self.pos;
                while self.next() && Lexer::is_whitespace_ascii(self.text.as_bytes()[self.pos]) {
                    token.token.push(char::from(self.text.as_bytes()[self.pos]));
                    self.pos += 1;
                }
                token.length = self.pos - token.begin;
                Some(token)
            }
            '1'..='9' => {
                let mut token = Token::default(TokenType::Number);
                token.begin = self.pos;
                while self.next() && Lexer::is_numeric_ascii(self.text.as_bytes()[self.pos]) {
                    token.token.push(char::from(self.text.as_bytes()[self.pos]));
                    self.pos += 1;
                }
                token.length = self.pos - token.begin;
                Some(token)
            }
            '\'' => {
                let mut token = Token::default(TokenType::StringLiteral);

                token.token.push('\'');
                token.begin = self.pos;
                token.length = 1;
                self.pos += 1;
                Some(token)
            }
            ',' => {
                let mut token = Token::default(TokenType::Comma);
                token.token.push(',');
                token.begin = self.pos;
                token.length = 1;
                self.pos += 1;
                Some(token)
            }
            _ => {
                if Lexer::is_alpha_ascii(c) {
                    let mut token = Token::default(TokenType::BareWord);
                    token.begin = self.pos;
                    while self.next() && Lexer::is_wordchars_ascii(self.text.as_bytes()[self.pos]) {
                        token.token.push(char::from(self.text.as_bytes()[self.pos]));
                        self.pos += 1;
                    }
                    token.length = self.pos - token.begin;
                    token.keyword = KeyWord::get_keyword(token.token.as_ref());
                    Some(token)
                } else {
                    let mut token = Token::default(TokenType::Unknown);
                    token.begin = self.pos;
                    token.token.push(char::from(self.text.as_bytes()[self.pos]));
                    self.pos += 1;
                    Some(token)
                }
            }
        }
    }

    pub fn next(&self) -> bool {
        self.pos < self.end
    }

    pub fn is_whitespace_ascii(c: u8) -> bool {
        match c as char {
            ' ' => true,
            _ => false,
        }
    }

    pub fn is_alpha_ascii(c: u8) -> bool {
        let c = c as char;
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    }

    pub fn is_numeric_ascii(c: u8) -> bool {
        let c = c as char;
        c >= '0' && c <= '9'
    }

    pub fn is_wordchars_ascii(c: u8) -> bool {
        Lexer::is_numeric_ascii(c) || Lexer::is_alpha_ascii(c)
    }
}
