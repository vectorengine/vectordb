// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[derive(Debug)]
pub struct VariablePlanner {
    pub val: String,
}

impl VariablePlanner {
    pub fn new(v: impl AsRef<str>) -> Self {
        VariablePlanner {
            val: v.as_ref().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        "VariablePlanner"
    }
}

mod tests {
    #[test]
    fn test_variable_planner() {
        use super::*;

        assert_eq!("VariablePlanner", VariablePlanner::new("").name());
        assert_eq!("a", VariablePlanner::new("a").val);
    }
}
