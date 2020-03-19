// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::planners::Planner;

#[derive(Debug, Default)]
pub struct Map {
    pub planners: Vec<Planner>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            planners: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        "Map"
    }
}
