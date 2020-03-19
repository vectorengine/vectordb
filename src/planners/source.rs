// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[derive(Debug)]
pub struct Source {
    pub schema: String,
    pub table: String,
}

impl Source {
    pub fn new(schema: String, table: String) -> Self {
        Source { schema, table }
    }

    pub fn name(&self) -> &str {
        "Source"
    }
}
