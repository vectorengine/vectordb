// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::{arithmetic, Datum};
use crate::errors::Error;

use super::*;

type Callback = Box<dyn Fn(&Datum, &Datum) -> Result<Datum, Error>>;

pub struct Binary {
    function: Callback,
    arguments: Vec<Expression>,
}

impl Binary {
    pub fn new(function: Callback, arguments: Vec<Expression>) -> Self {
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

pub fn add(args: Vec<Expression>) -> Expression {
    Expression::BinaryExpression(Binary::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            arithmetic::add(left, right)
        }),
        args,
    ))
}

pub fn sub(args: Vec<Expression>) -> Expression {
    Expression::BinaryExpression(Binary::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            arithmetic::sub(left, right)
        }),
        args,
    ))
}

pub fn mul(args: Vec<Expression>) -> Expression {
    Expression::BinaryExpression(Binary::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            arithmetic::mul(left, right)
        }),
        args,
    ))
}

pub fn div(args: Vec<Expression>) -> Expression {
    Expression::BinaryExpression(Binary::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            arithmetic::div(left, right)
        }),
        args,
    ))
}

#[test]
fn test_add() {
    let l = Expression::ConstantExpression(Constant::new(Datum::Int32(1)));
    let r = Expression::ConstantExpression(Constant::new(Datum::Int32(3)));
    let fn1 = add(vec![l, r]);

    let l = Expression::ConstantExpression(Constant::new(Datum::Int32(5)));
    let r = Expression::ConstantExpression(Constant::new(Datum::Int32(7)));
    let fn2 = add(vec![l, r]);

    let func = add(vec![fn1, fn2]);
    let result = func.eval();

    assert_eq!(true, result.is_ok());
    assert_eq!(Datum::Int32(16), result.unwrap());
}
