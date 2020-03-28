// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;
use crate::errors::{Error, ExpressionError};

use super::IExpression;

#[derive(Debug)]
pub struct VariableExpression {
    pub val: String,
}

impl VariableExpression {
    pub fn new(v: impl AsRef<str>) -> Self {
        VariableExpression {
            val: v.as_ref().to_string(),
        }
    }
}

impl IExpression for VariableExpression {
    fn eval(&self) -> Result<Datum, Error> {
        Err(Error::Expression(ExpressionError::UnsupportedOperation))
    }
}

mod tests {
    #[test]
    fn test_variable() {
        use super::*;

        let expr = VariableExpression::new("a");
        let result = expr.eval();
        assert_eq!(false, result.is_ok());
    }
}
