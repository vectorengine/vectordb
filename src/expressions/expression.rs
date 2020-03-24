// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;
use crate::errors::Error;

pub trait IExpression {
    fn eval(&self) -> Result<Datum, Error>;
}
