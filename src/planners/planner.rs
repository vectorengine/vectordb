// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::planners::*;

#[derive(Debug)]
pub enum Planner {
    Null,
    Map(MapPlanner),

    Constant(ConstantPlanner),
    Variable(VariablePlanner),
    Source(SourcePlanner),
    Filter(FilterPlanner),
    Select(SelectPlanner),

    BinaryExpression(Box<BinaryExpressionPlanner>),
    ScalarExpression(Box<ScalarExpressionPlanner>),
}

impl From<MapPlanner> for Planner {
    fn from(v: MapPlanner) -> Self {
        Planner::Map(v)
    }
}

impl From<ConstantPlanner> for Planner {
    fn from(v: ConstantPlanner) -> Self {
        Planner::Constant(v)
    }
}

impl From<VariablePlanner> for Planner {
    fn from(v: VariablePlanner) -> Self {
        Planner::Variable(v)
    }
}

impl From<SourcePlanner> for Planner {
    fn from(v: SourcePlanner) -> Self {
        Planner::Source(v)
    }
}

impl From<FilterPlanner> for Planner {
    fn from(v: FilterPlanner) -> Self {
        Planner::Filter(v)
    }
}

impl From<SelectPlanner> for Planner {
    fn from(v: SelectPlanner) -> Self {
        Planner::Select(v)
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
