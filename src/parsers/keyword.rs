// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::parsers::Tokens;

#[derive(Debug)]
pub enum KeyWord {
    NONE,
    EXPLAIN,
    FROM,
    SELECT,
}

impl KeyWord {
    pub fn get_keyword(word: &str) -> Self {
        match word.to_uppercase().as_ref() {
            "EXPLAIN" => Self::EXPLAIN,
            "FROM" => Self::FROM,
            "SELECT" => Self::SELECT,
            _ => Self::NONE,
        }
    }
}

pub struct ParserKeyword {
    pub keyword: String,
}

impl ParserKeyword {
    pub fn default(keyword: String) -> Self {
        ParserKeyword { keyword }
    }

    pub fn ignore(&self, tokens: Tokens) -> bool {
        let token = tokens.peek_token().unwrap();
        self.keyword == token.token
    }
}
