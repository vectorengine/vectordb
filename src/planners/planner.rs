// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

pub trait IPlanner {
    fn name(&self) -> &str;
}
