// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::Datum;
use crate::errors::Error;

#[allow(dead_code)]
pub struct Test {
    pub name: &'static str,
    pub args: Vec<Datum>,
    pub expect: Datum,
    pub func: Box<dyn Fn(&Datum, &Datum) -> Result<Datum, Error>>,
    pub error: Option<Error>,
}
