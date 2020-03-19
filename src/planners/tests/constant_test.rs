// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[cfg(test)]
mod tests {
    use crate::planners::Constant;
    use crate::values::Value;

    #[test]
    fn test_constant_planner() {
        assert_eq!(String::from("Constant"), Constant::new(Value::Null).name());
        assert_eq!(
            Value::Str(String::from("String")),
            Constant::new(Value::Str(String::from("String"))).val
        );
        assert_eq!(Value::Int(1), Constant::new(Value::Int(1)).val);
    }
}
