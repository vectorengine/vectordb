// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::{binary::*, Expression};
use crate::errors::{Error, ExpressionError};

type Factory = Box<dyn Fn(Vec<Expression>) -> Expression + Sync>;

struct Function {
    pub factory: Factory,
}

impl Function {
    pub fn op(factory: Factory) -> Function {
        Function { factory }
    }
}

pub fn expression_factory(op: &str, args: Vec<Expression>) -> Result<Expression, Error> {
    match FACTORY.get(op) {
        Some(v) => Ok((v.factory)(args)),
        None => Err(Error::from(ExpressionError::UnsupportedOperation)),
    }
}

lazy_static! {
    static ref FACTORY: HashMap<&'static str, Function> = vec![
        ("+", Function::op(Box::new(add))),
        ("-", Function::op(Box::new(sub))),
        ("*", Function::op(Box::new(mul))),
        ("/", Function::op(Box::new(div))),
        ("=", Function::op(Box::new(eq))),
        (">", Function::op(Box::new(gt))),
        (">=", Function::op(Box::new(gte))),
        ("<", Function::op(Box::new(lt))),
        ("<=", Function::op(Box::new(lte))),
    ]
    .into_iter()
    .collect();
}

mod tests {
    #[test]
    fn test_factory() {
        use super::{super::*, *};
        use crate::datums::Datum;

        let l = Expression::from(ConstantExpression::new(Datum::Int32(1)));
        let r = Expression::from(ConstantExpression::new(Datum::Int32(3)));
        let expr = expression_factory("+", vec![l, r]).unwrap();
        let result = expr.eval();

        assert_eq!(true, result.is_ok());
        assert_eq!(Datum::Int32(4), result.unwrap());
    }
}
