// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::planners::Planner;

#[derive(Debug)]
pub struct ScalarExpressionPlanner {
    pub op: String,
    pub arguments: Vec<Planner>,
}

impl ScalarExpressionPlanner {
    pub fn new(op: String, arguments: Vec<Planner>) -> Self {
        ScalarExpressionPlanner { op, arguments }
    }

    pub fn name(&self) -> &str {
        "ScalarExpressionPlanner"
    }
}
