// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::datums::Datum;

#[derive(Debug)]
pub struct ConstantPlanner {
    pub val: Datum,
}

impl ConstantPlanner {
    pub fn new(t: Datum) -> Self {
        ConstantPlanner { val: t }
    }

    pub fn name(&self) -> &str {
        "ConstantPlanner"
    }
}

mod tests {
    #[test]
    fn test_constant_planner() {
        use super::*;

        assert_eq!(
            String::from("ConstantPlanner"),
            ConstantPlanner::new(Datum::Null).name()
        );
        assert_eq!(
            Datum::String(String::from("String")),
            ConstantPlanner::new(Datum::String(String::from("String"))).val
        );
        assert_eq!(Datum::Int32(1), ConstantPlanner::new(Datum::Int32(1)).val);
    }
}
