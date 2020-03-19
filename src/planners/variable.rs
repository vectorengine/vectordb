// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[derive(Debug)]
pub struct Variable {
    pub val: String,
}

impl Variable {
    pub fn new(v: impl AsRef<str>) -> Self {
        Variable {
            val: v.as_ref().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        "Variable"
    }
}
