// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::parsers::KeyWord;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Unknown,
    WhiteSpace,
    BareWord,
    StringLiteral,
    Number,
    Comma,
}

#[derive(Debug)]
pub struct Token {
    pub token: String,
    pub token_type: TokenType,
    pub begin: usize,
    pub length: usize,
    pub keyword: KeyWord,
}

impl Token {
    pub fn default(typ: TokenType) -> Token {
        Token {
            token: "".to_string(),
            token_type: typ,
            begin: 0,
            length: 0,
            keyword: KeyWord::NONE,
        }
    }

    pub fn is_significant(&self) -> bool {
        self.token_type != TokenType::WhiteSpace
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Tokens {
    pub fn default() -> Self {
        Tokens {
            tokens: vec![],
            pos: 0,
        }
    }

    pub fn peek_token(&self) -> Option<&Token> {
        Some(&self.tokens[self.pos])
    }
}

impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.pos += 1;
        self.tokens.pop()
    }
}
