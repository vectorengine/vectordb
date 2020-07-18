// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::errors::{Error, SQLError};
use crate::parsers::keyword::ParserKeyword;
use crate::parsers::{Tokens, IAST};

#[derive(Debug)]
pub struct Select {
    pub name: String,
}

impl Select {
    pub fn default() -> Self {
        Select {
            name: "".to_string(),
        }
    }
}

impl IAST for Select {
    fn parse(&self, _tokens: Tokens) -> Option<Error> {
        let _s_select = ParserKeyword::default("SELECT".to_string());
        if !_s_select.ignore(_tokens) {
            return Some(Error::from(SQLError::NotImplemented(format!(
                "Unsupported select:{:?}",
                "",
            ))));
        }
        None
    }
}
