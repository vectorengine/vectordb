// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[derive(Debug)]
pub struct SourcePlanner {
    pub schema: String,
    pub table: String,
}

impl SourcePlanner {
    pub fn new(schema: String, table: String) -> Self {
        SourcePlanner { schema, table }
    }

    pub fn name(&self) -> &str {
        "SourcePlanner"
    }
}
