// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::planners::{
    BinaryExpressionPlanner, ConstantPlanner, MapPlanner, ScalarExpressionPlanner, SourcePlanner,
    VariablePlanner,
};

#[derive(Debug)]
pub enum Planner {
    Null,
    Map(MapPlanner),

    Variable(VariablePlanner),
    Constant(ConstantPlanner),
    Source(SourcePlanner),

    BinaryExpression(Box<BinaryExpressionPlanner>),
    ScalarExpression(Box<ScalarExpressionPlanner>),
}

impl From<ConstantPlanner> for Planner {
    fn from(v: ConstantPlanner) -> Self {
        Planner::Constant(v)
    }
}

impl From<SourcePlanner> for Planner {
    fn from(v: SourcePlanner) -> Self {
        Planner::Source(v)
    }
}

impl From<VariablePlanner> for Planner {
    fn from(v: VariablePlanner) -> Self {
        Planner::Variable(v)
    }
}

impl From<BinaryExpressionPlanner> for Planner {
    fn from(v: BinaryExpressionPlanner) -> Self {
        Planner::BinaryExpression(Box::new(v))
    }
}

impl From<ScalarExpressionPlanner> for Planner {
    fn from(v: ScalarExpressionPlanner) -> Self {
        Planner::ScalarExpression(Box::new(v))
    }
}
