// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;
use crate::errors::Error;

use super::IExpression;

#[derive(Debug)]
pub struct ConstantExpression {
    pub val: Datum,
}

impl ConstantExpression {
    pub fn new(val: Datum) -> Self {
        ConstantExpression { val }
    }
}

impl IExpression for ConstantExpression {
    fn eval(&self) -> Result<Datum, Error> {
        Ok(self.val.clone())
    }
}

mod tests {
    #[test]
    fn test_constant() {
        use super::*;

        let expr = ConstantExpression::new(Datum::Int64(64));
        let result = expr.eval();
        assert_eq!(true, result.is_ok());
        assert_eq!(result.unwrap(), expr.val);
    }
}
