// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;
use crate::errors::Error;

use super::*;

type Callback = Box<dyn Fn(&Datum, &Datum) -> Result<Datum, Error>>;

pub struct Binary {
    function: Callback,
    arguments: Vec<Box<dyn IExpression>>,
}

impl Binary {
    pub fn new(function: Callback, arguments: Vec<Box<dyn IExpression>>) -> Self {
        Binary {
            function,
            arguments,
        }
    }
}

impl IExpression for Binary {
    fn eval(&self) -> Result<Datum, Error> {
        let (ref left, ref right) = (self.arguments[0].eval()?, self.arguments[1].eval()?);
        (self.function)(left, right)
    }
}
