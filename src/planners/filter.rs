// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::*;

#[derive(Debug)]
pub struct FilterPlanner {
    pub planners: MapPlanner,
}

impl FilterPlanner {
    pub fn default() -> Self {
        FilterPlanner {
            planners: MapPlanner::new(),
        }
    }

    pub fn name(&self) -> &str {
        "FilterPlanner"
    }

    pub fn add(&mut self, planner: Planner) {
        self.planners.add(planner);
    }
}
