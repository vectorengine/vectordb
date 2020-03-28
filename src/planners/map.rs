// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::planners::Planner;

#[derive(Debug, Default)]
pub struct MapPlanner {
    pub planners: Vec<Planner>,
}

impl MapPlanner {
    pub fn new() -> Self {
        MapPlanner {
            planners: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        "MapPlanner"
    }
}
