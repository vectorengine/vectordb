// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

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
