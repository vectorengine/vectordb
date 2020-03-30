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

    #[fail(display = "Datum error: `{}`", _0)]
    Datum(#[cause] DatumError),
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

#[derive(Debug, Fail)]
pub enum DatumError {
    #[fail(display = "Unsupported operation.")]
    UnsupportedOperation,
}

impl From<SQLError> for Error {
    fn from(v: SQLError) -> Self {
        Error::SQL(v)
    }
}

impl From<ExpressionError> for Error {
    fn from(v: ExpressionError) -> Self {
        Error::Expression(v)
    }
}

impl From<DatumError> for Error {
    fn from(v: DatumError) -> Self {
        Error::Datum(v)
    }
}
