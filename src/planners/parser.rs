// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[test]
fn test_select() {
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    let sql = "SELECT a, b, 123, funcxx(b) \
               FROM table_1 \
               WHERE a > b AND b < 100 \
               ORDER BY a DESC, b";
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, sql.to_string());
    println!("SQL:{}\nAST:\n{:#?}", sql, ast.unwrap());
}
