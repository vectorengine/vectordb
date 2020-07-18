// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::errors::Error;
use crate::parsers::{Select, Tokens};

pub trait IAST {
    fn parse(&self, tokens: Tokens) -> Option<Error>;
}

#[derive(Debug)]
pub enum Statement {
    Select(Box<Select>),
}
