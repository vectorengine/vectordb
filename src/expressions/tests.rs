// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;
use crate::errors::Error;

use super::Expression;

#[allow(dead_code)]
pub struct Test {
    pub name: &'static str,
    pub args: Vec<Expression>,
    pub expect: Datum,
    pub func: Box<dyn Fn(Vec<Expression>) -> Expression>,
    pub error: Option<Error>,
}
