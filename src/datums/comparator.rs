// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::*;
use crate::errors::{DatumError, Error};

comparator!(
    gt,
    >,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64,
    Datum::String
);

comparator!(
    gte,
    >=,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64,
    Datum::String
);

comparator!(
    lt,
    <,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64,
    Datum::String
);

comparator!(
    lte,
    <=,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64,
    Datum::String
);

comparator!(
    eq,
    ==,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64,
    Datum::String
);

#[test]
fn test_comparator() {
    use tests::Test;

    let tests = [
        Test {
            name: "gt-passed",
            args: vec![Datum::Int32(3), Datum::Int32(2)],
            expect: Datum::Boolean(true),
            func: Box::new(gt),
            error: None,
        },
        Test {
            name: "gte-passed",
            args: vec![Datum::Int32(2), Datum::Int32(2)],
            expect: Datum::Boolean(true),
            func: Box::new(gte),
            error: None,
        },
        Test {
            name: "lt-passed",
            args: vec![Datum::Int32(1), Datum::Int32(2)],
            expect: Datum::Boolean(true),
            func: Box::new(lt),
            error: None,
        },
        Test {
            name: "lte-passed",
            args: vec![Datum::Int32(2), Datum::Int32(2)],
            expect: Datum::Boolean(true),
            func: Box::new(lte),
            error: None,
        },
        Test {
            name: "eq-passed",
            args: vec![Datum::Int32(1), Datum::Int32(1)],
            expect: Datum::Boolean(true),
            func: Box::new(eq),
            error: None,
        },
    ];

    for t in tests.iter() {
        let d3 = (t.func.as_ref())(&t.args[0], &t.args[1]);
        assert_eq!(t.expect, d3.unwrap());
    }
}
