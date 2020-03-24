// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use failure::*;
use sqlparser::parser::ParserError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "SQL error: `{}`", _0)]
    SQL(#[cause] SQLError),

    #[fail(display = "Expression error: `{}`", _0)]
    Expression(#[cause] ExpressionError),
}

#[derive(Debug, Fail)]
pub enum SQLError {
    #[fail(display = "Parser error: {}", _0)]
    ParserError(ParserError),

    #[fail(display = "Not implemented: {}", _0)]
    NotImplemented(String),

    #[fail(display = "Unsupported operation.")]
    UnsupportedOperation,
}

#[derive(Debug, Fail)]
pub enum ExpressionError {
    #[fail(display = "Not implemented: {}", _0)]
    NotImplemented(String),

    #[fail(display = "Unsupported operation.")]
    UnsupportedOperation,
}
