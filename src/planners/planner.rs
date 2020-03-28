// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::planners::{BinaryExpression, Constant, Map, ScalarExpression, Source, Variable};

#[derive(Debug)]
pub enum Planner {
    NonePlanner,
    MapPlanner(Map),

    VariablePlanner(Variable),
    ConstantPlanner(Constant),
    SourcePlanner(Source),

    BinaryExpressionPlanner(Box<BinaryExpression>),
    ScalarExpressionPlanner(Box<ScalarExpression>),
}

impl From<Constant> for Planner {
    fn from(v: Constant) -> Self {
        Planner::ConstantPlanner(v)
    }
}

impl From<Variable> for Planner {
    fn from(v: Variable) -> Self {
        Planner::VariablePlanner(v)
    }
}

impl From<BinaryExpression> for Planner {
    fn from(v: BinaryExpression) -> Self {
        Planner::BinaryExpressionPlanner(Box::new(v))
    }
}
