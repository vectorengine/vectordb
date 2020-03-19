// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::planners::{Constant, Map, Source, Variable};

#[derive(Debug)]
pub enum Planner {
    VariablePlanner(Box<Variable>),
    ConstantPlanner(Box<Constant>),
    SourcePlanner(Box<Source>),
    MapPlanner(Box<Map>),
}
