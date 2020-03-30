// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::*;

#[derive(Debug)]
pub struct SelectPlanner {
    pub planners: MapPlanner,
}

impl SelectPlanner {
    pub fn default() -> Self {
        SelectPlanner {
            planners: MapPlanner::new(),
        }
    }

    pub fn name(&self) -> &str {
        "SelectPlanner"
    }

    pub fn add(&mut self, planner: Planner) {
        self.planners.add(planner);
    }
}
