// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;
use crate::errors::Error;

use super::{BinaryExpression, ConstantExpression, VariableExpression};

pub trait IExpression {
    fn eval(&self) -> Result<Datum, Error>;
}

pub enum Expression {
    Constant(ConstantExpression),
    Variable(VariableExpression),
    Binary(BinaryExpression),
}

impl From<ConstantExpression> for Expression {
    fn from(v: ConstantExpression) -> Self {
        Expression::Constant(v)
    }
}

impl From<VariableExpression> for Expression {
    fn from(v: VariableExpression) -> Self {
        Expression::Variable(v)
    }
}

impl From<BinaryExpression> for Expression {
    fn from(v: BinaryExpression) -> Self {
        Expression::Binary(v)
    }
}

impl IExpression for Expression {
    fn eval(&self) -> Result<Datum, Error> {
        match self {
            Expression::Constant(v) => v.eval(),
            Expression::Variable(v) => v.eval(),
            Expression::Binary(v) => v.eval(),
        }
    }
}
