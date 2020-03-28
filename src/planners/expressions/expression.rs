// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use crate::errors::{Error, SQLError};
use crate::expressions::{factory, Constant as ConstantExpr, Expression, Variable as VariableExpr};
use crate::planners::Planner;

pub fn planner_to_expression(planner: Planner) -> Result<Expression, Error> {
    match planner {
        Planner::ConstantPlanner(v) => Ok(Expression::from(ConstantExpr::new(v.val))),
        Planner::VariablePlanner(v) => Ok(Expression::from(VariableExpr::new(v.val))),
        Planner::BinaryExpressionPlanner(v) => {
            let left = planner_to_expression(v.left).unwrap();
            let right = planner_to_expression(v.right).unwrap();
            factory::factory(v.op.as_str(), vec![left, right])
        }
        _ => Err(Error::SQL(SQLError::NotImplemented(format!(
            "{:?}",
            planner,
        )))),
    }
}

mod tests {
    #[test]
    fn test_planner_to_expression() {
        use crate::datums::Datum;
        use crate::expressions::IExpression;
        use crate::planners::{BinaryExpression, Constant, Planner};

        let c11 = Planner::from(Constant::new(Datum::Int32(5)));
        let c12 = Planner::from(Constant::new(Datum::Int32(3)));
        let b11 = Planner::from(BinaryExpression::new("+".to_string(), c11, c12));

        let c21 = Planner::from(Constant::new(Datum::Int32(6)));
        let c22 = Planner::from(Constant::new(Datum::Int32(4)));
        let b21 = Planner::from(BinaryExpression::new("+".to_string(), c21, c22));

        let b31 = Planner::from(BinaryExpression::new("+".to_string(), b11, b21));

        let expr = super::planner_to_expression(b31).unwrap();
        let actual = expr.eval().unwrap();
        let expect = Datum::Int32(18);
        assert_eq!(expect, actual);
    }
}
