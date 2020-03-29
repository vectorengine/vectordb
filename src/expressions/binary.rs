// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::*;
use crate::errors::Error;

use super::*;

type Callback = Box<dyn Fn(&Datum, &Datum) -> Result<Datum, Error>>;

pub struct BinaryExpression {
    function: Callback,
    arguments: Vec<Expression>,
}

impl BinaryExpression {
    pub fn new(function: Callback, arguments: Vec<Expression>) -> Self {
        BinaryExpression {
            function,
            arguments,
        }
    }
}

impl IExpression for BinaryExpression {
    fn eval(&self) -> Result<Datum, Error> {
        let (ref left, ref right) = (self.arguments[0].eval()?, self.arguments[1].eval()?);
        (self.function)(left, right)
    }
}

pub fn add(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            arithmetic::add(left, right)
        }),
        args,
    ))
}

pub fn sub(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            arithmetic::sub(left, right)
        }),
        args,
    ))
}

pub fn mul(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            arithmetic::mul(left, right)
        }),
        args,
    ))
}

pub fn div(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            arithmetic::div(left, right)
        }),
        args,
    ))
}

pub fn gt(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            comparator::gt(left, right)
        }),
        args,
    ))
}

pub fn gte(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            comparator::gte(left, right)
        }),
        args,
    ))
}

pub fn lt(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            comparator::lt(left, right)
        }),
        args,
    ))
}

pub fn lte(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            comparator::lte(left, right)
        }),
        args,
    ))
}

pub fn eq(args: Vec<Expression>) -> Expression {
    Expression::from(BinaryExpression::new(
        Box::new(|left: &Datum, right: &Datum| -> Result<Datum, Error> {
            comparator::eq(left, right)
        }),
        args,
    ))
}

mod tests {
    #[test]
    fn test_add() {
        use super::{super::tests::Test, *};

        let mut tests = vec![
            Test {
                name: "add-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(10))),
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                ],
                expect: Datum::Int32(12),
                func: Box::new(add),
                error: None,
            },
            Test {
                name: "sub-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(10))),
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                ],
                expect: Datum::Int32(8),
                func: Box::new(sub),
                error: None,
            },
            Test {
                name: "mul-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(10))),
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                ],
                expect: Datum::Int32(20),
                func: Box::new(mul),
                error: None,
            },
            Test {
                name: "div-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(10))),
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                ],
                expect: Datum::Int32(5),
                func: Box::new(div),
                error: None,
            },
            Test {
                name: "gt-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(10))),
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                ],
                expect: Datum::Boolean(true),
                func: Box::new(gt),
                error: None,
            },
            Test {
                name: "gte-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                ],
                expect: Datum::Boolean(true),
                func: Box::new(gte),
                error: None,
            },
            Test {
                name: "lt-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                    Expression::from(ConstantExpression::new(Datum::Int32(10))),
                ],
                expect: Datum::Boolean(true),
                func: Box::new(lt),
                error: None,
            },
            Test {
                name: "lte-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                ],
                expect: Datum::Boolean(true),
                func: Box::new(lte),
                error: None,
            },
            Test {
                name: "eq-passed",
                args: vec![
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                    Expression::from(ConstantExpression::new(Datum::Int32(2))),
                ],
                expect: Datum::Boolean(true),
                func: Box::new(eq),
                error: None,
            },
        ];

        while let Some(t) = tests.pop() {
            let actual = (t.func.as_ref())(t.args).eval().unwrap();
            assert_eq!(t.expect, actual);
        }
    }
}
