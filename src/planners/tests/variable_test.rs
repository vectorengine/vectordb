// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[cfg(test)]
mod tests {
    use crate::planners::Variable;

    #[test]
    fn test_variable_planner() {
        assert_eq!("Variable", Variable::new("").name());
        assert_eq!("a", Variable::new("a").val);
    }
}
