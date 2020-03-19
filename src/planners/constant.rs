// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::values::Value;

#[derive(Debug)]
pub struct Constant {
    pub val: Value,
}

impl Constant {
    pub fn new(t: Value) -> Self {
        Constant { val: t }
    }

    pub fn name(&self) -> &str {
        "Constant"
    }
}
