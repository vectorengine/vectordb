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

#[test]
fn test_constant_planner() {
    assert_eq!(String::from("Constant"), Constant::new(Datum::Null).name());
    assert_eq!(
        Datum::String(String::from("String")),
        Constant::new(Datum::String(String::from("String"))).val
    );
    assert_eq!(Datum::Int32(1), Constant::new(Datum::Int32(1)).val);
}
