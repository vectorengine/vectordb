// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;
use crate::errors::Error;

use super::IExpression;

#[derive(Debug)]
pub struct Constant {
    pub val: Datum,
}

impl Constant {
    pub fn new(val: Datum) -> Self {
        Constant { val }
    }
}

impl IExpression for Constant {
    fn eval(&self) -> Result<Datum, Error> {
        Ok(self.val.clone())
    }
}

#[test]
fn test_constant() {
    let expr = Constant::new(Datum::Int64(64));
    let result = expr.eval();
    assert_eq!(true, result.is_ok());
    assert_eq!(result.unwrap(), expr.val);
}
