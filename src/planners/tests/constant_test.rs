// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[cfg(test)]
mod tests {
    use crate::datums::Datum;
    use crate::planners::Constant;

    #[test]
    fn test_constant_planner() {
        assert_eq!(String::from("Constant"), Constant::new(Datum::Null).name());
        assert_eq!(
            Datum::String(String::from("String")),
            Constant::new(Datum::String(String::from("String"))).val
        );
        assert_eq!(Datum::Int32(1), Constant::new(Datum::Int32(1)).val);
    }
}
