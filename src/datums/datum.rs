// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[derive(Debug, PartialEq)]
pub enum Datum {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    Null,
}
