// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;

#[derive(Debug)]
pub struct Constant {
    pub val: Datum,
}

impl Constant {
    pub fn new(t: Datum) -> Self {
        Constant { val: t }
    }

    pub fn name(&self) -> &str {
        "Constant"
    }
}
