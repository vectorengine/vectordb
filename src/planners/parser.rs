// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use sqlparser::ast::{Query, SetExpr, Statement, TableFactor};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

use crate::errors::{Error, SQLError};
use crate::planners::planner::Planner::MapPlanner;
use crate::planners::{Map, Planner, Source};

pub fn parser(sql: String) -> Result<Statement, Error> {
    let dialect = GenericDialect {};

    let mut parsed = match Parser::parse_sql(&dialect, sql) {
        Ok(v) => v,
        Err(e) => return Err(Error::SQL(SQLError::ParserError(e))),
    };

    let ast = match parsed.pop() {
        Some(v) => v,
        None => return Err(Error::SQL(SQLError::UnsupportedOperation)),
    };

    Ok(ast)
}

pub fn handle_statement(stmt:Statement) ->Result<Planner, Error>  {
    match stmt {
        Statement::Query(query) => handle_query(*query),
        _=> Err(Error::SQL(SQLError::UnsupportedOperation)),
    }
}

pub fn handle_query(query: Query) -> Result<Planner, Error> {
    let sqlparser::ast::Query { body, .. } = query;

    let mut from = match body {
        SetExpr::Select(select) => select.from,
        _ => return Err(Error::SQL(SQLError::UnsupportedOperation)),
    };
    let table = from.pop().map(|t| t.relation);
    let source = parse_source_planner(table)?;

    let mut planners = Map::new();
    planners.planners.push(source);
    Ok(MapPlanner(Box::new(planners)))
}

pub fn parse_source_planner(relation: Option<TableFactor>) -> Result<Planner, Error> {
    let object_name = match relation {
        Some(TableFactor::Table { name, .. }) => name,
        Some(e) => {
            return Err(Error::SQL(SQLError::NotImplemented(format!(
                "Table: {}",
                e
            ))))
        }
        None => return Err(Error::SQL(SQLError::UnsupportedOperation)),
    };

    let (schema, table) = match object_name.0.len() {
        1 => ("", object_name.0.get(0).unwrap().as_str()),
        2 => (
            object_name.0.get(0).unwrap().as_str(),
            object_name.0.get(1).unwrap().as_str(),
        ),
        _ => return Err(Error::SQL(SQLError::UnsupportedOperation)),
    };

    Ok(Planner::SourcePlanner(Box::new(Source::new(
        schema.to_string(),
        table.to_string(),
    ))))
}

#[test]
fn test_select() {
    {
        let sql = "";
        let query = parser(sql.to_string());
        assert_eq!(true, query.is_err());
    }

    {
        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";
        let stmt = parser(sql.to_string());
        assert_eq!(true, stmt.is_ok());
        print!("{:#?}", stmt);
        let planner = handle_statement(stmt.unwrap());
        assert_eq!(true, planner.is_ok());

        print!("{:#?}", planner);
    }
}
