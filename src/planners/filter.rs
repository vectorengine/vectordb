// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::*;

#[derive(Debug)]
pub struct FilterPlanner {
    pub planners: MapPlanner,
}

impl FilterPlanner {
    pub fn new(planners: MapPlanner) -> Self {
        FilterPlanner { planners }
    }

    pub fn name(&self) -> &str {
        "FilterPlanner"
    }
}
