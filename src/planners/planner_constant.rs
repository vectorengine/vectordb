// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::planner::*;

#[derive(Debug, Default)]
pub struct ConstantPlanner<T> {
    val: T,
}

impl<T> ConstantPlanner<T> {
    pub fn new(t: T) -> Self {
        ConstantPlanner { val: t }
    }
}

impl<T> IPlanner for ConstantPlanner<T> {
    fn name(&self) -> &str {
        "ConstantPlanner"
    }
}

#[test]
fn test_constant_planner() {
    assert_eq!("ConstantPlanner", ConstantPlanner::new("").name());
    assert_eq!("String", ConstantPlanner::new("String").val);
    assert_eq!(1, ConstantPlanner::new(1).val);
}
