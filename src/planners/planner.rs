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
