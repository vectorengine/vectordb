// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::errors::Error;
use crate::parsers::{Select, Tokens, IAST};

#[derive(Debug)]
pub struct Explain {
    pub name: String,
    pub select: Select,
}

impl Explain {
    pub fn default() -> Self {
        Explain {
            name: "Explain".to_string(),
            select: Select::default(),
        }
    }
}

impl IAST for Explain {
    fn parse(&self, _tokens: Tokens) -> Option<Error> {
        self.select.parse(_tokens)
    }
}
