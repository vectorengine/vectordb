// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;
use crate::errors::{Error, ExpressionError};

use super::IExpression;

#[derive(Debug)]
pub struct Variable {
    pub val: String,
}

impl Variable {
    pub fn new(v: impl AsRef<str>) -> Self {
        Variable {
            val: v.as_ref().to_string(),
        }
    }
}

impl IExpression for Variable {
    fn eval(&self) -> Result<Datum, Error> {
        Err(Error::Expression(ExpressionError::UnsupportedOperation))
    }
}

#[test]
fn test_variable() {
    let expr = Variable::new("a");
    let result = expr.eval();
    assert_eq!(false, result.is_ok());
}
