// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

macro_rules! arithmetic {
    ($func:ident, $op:tt, $( $y:path ),*) => (
        pub fn $func(x: &Datum, y: &Datum) -> Result<Datum,Error> {
            match (x, y) {
                    $(($y(a), $y(b)) => Ok($y(a $op b)),)+
                    _ => Err(Error::from(DatumError::UnsupportedOperation)),
            }
        }
    )
}

macro_rules! comparator{
    ($func:ident, $op:tt, $( $y:path ),*) => (
        pub fn $func(x: &Datum, y: &Datum) -> Result<Datum,Error> {
            match (x, y) {
                    $(($y(a), $y(b)) => Ok(Datum::Boolean(a $op b)),)+
                    _ => Err(Error::from(DatumError::UnsupportedOperation)),
            }
        }
    )
}
