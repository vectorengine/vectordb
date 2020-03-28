// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::planners::Planner;

#[derive(Debug)]
pub struct BinaryExpressionPlanner {
    pub op: String,
    pub left: Planner,
    pub right: Planner,
}

impl BinaryExpressionPlanner {
    pub fn new(op: String, left: Planner, right: Planner) -> Self {
        BinaryExpressionPlanner { op, left, right }
    }

    pub fn name(&self) -> &str {
        "BinaryExpressionPlanner"
    }
}
