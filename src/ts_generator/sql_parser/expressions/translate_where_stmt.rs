use std::collections::HashMap;

use sqlparser::ast::{Expr, Statement, TableWithJoins};

use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::{
    errors::TsGeneratorError,
    sql_parser::translate_query::translate_query,
    types::db_conn::DBConn,
    types::ts_query::{TsFieldType, TsQuery},
};

use super::{
    translate_expr::{get_expr_placeholder, translate_column_name_expr},
    translate_table_with_joins::translate_table_from_expr,
};

/// handle an expression from where clauses (or it can be from anywhere)
/// pick up any expression from left and right that goes
/// some_field = ?
/// some_table.some_field = ?
///
/// or
///
/// some_field = $1
/// some_table.some_field = $1
pub fn get_sql_query_param(
    left: &Box<Expr>,
    right: &Box<Expr>,
    single_table_name: &Option<&str>,
    table_with_joins: &Option<&Vec<TableWithJoins>>,
    db_conn: &DBConn,
) -> Option<(TsFieldType, Option<String>)> {
    let table_name: Option<String>;

    if table_with_joins.is_some() {
        table_name = translate_table_from_expr(table_with_joins.unwrap(), &*left.clone());
    } else if single_table_name.is_some() {
        table_name = single_table_name.map(|x| x.to_string());
    } else {
        panic!("failed to find an appropriate table name while processing WHERE statement")
    }

    let column_name = translate_column_name_expr(left);

    // If the right side of the expression is a placeholder `?` or `$n`
    // they are valid query parameter to process
    let expr_placeholder = get_expr_placeholder(right);

    if column_name.is_some() && expr_placeholder.is_some() && table_name.is_some() {
        let table_name = table_name.unwrap();
        let table_names = vec![table_name.as_str()];
        let column_name = column_name.unwrap();
        let columns = DB_SCHEMA
            .fetch_table(&table_names, db_conn)
            .unwrap_or_else(|| panic!("Failed to fetch columns for table {:?}", table_name));

        // get column and return TsFieldType
        let column = columns
            .get(column_name.as_str())
            .unwrap_or_else(|| panic!("Failed toe find the column from the table schema of {:?}", table_name));
        return Some((column.field_type.to_owned(), expr_placeholder));
    }

    None
}

pub fn translate_where_stmt<'a>(
    ts_query: &'a mut TsQuery,
    // sql_statement is required
    expr: &Expr,
    // queries like DELETE and INSERT would never have table_with_joins
    single_table_name: &Option<&str>,
    // queries like SELECT might have table_with_joins and we need this explicitly
    table_with_joins: &Option<&Vec<TableWithJoins>>,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    match expr {
        Expr::BinaryOp { left, op: _, right } => {
            let param = get_sql_query_param(left, right, single_table_name, table_with_joins, db_conn);

            if param.is_none() {
                translate_where_stmt(ts_query, left, single_table_name, table_with_joins, db_conn)?;
                translate_where_stmt(ts_query, right, single_table_name, table_with_joins, db_conn)?;
            } else {
                let (value, index) = param.unwrap();
                ts_query.insert_param(&value, &index);
            }
            Ok(())
        }
        Expr::InList { expr, list, negated: _ } => {
            // If the list is just a single `(?)`, then we should return the dynamic
            // If the list contains multiple `(?, ?...)` then we should return a fixed length array
            if list.len() == 1 {
                let right = list
                    .get(0)
                    .expect("Failed to find the first list item from the IN query");
                let result = get_sql_query_param(
                    expr,
                    &Box::new(right.to_owned()),
                    single_table_name,
                    table_with_joins,
                    db_conn,
                );

                if result.is_some() {
                    let (value, index) = result.unwrap();
                    let array_item = value.to_array_item();

                    ts_query.insert_param(&array_item, &index);
                    return Ok(());
                } else {
                    return Ok(());
                }
            }
            Ok(())
        }
        Expr::InSubquery {
            expr: _,
            subquery,
            negated: _,
        } => {
            translate_query(ts_query, subquery, db_conn, true)?;
            Ok(())
        }
        Expr::Subquery(subquery) => {
            translate_query(ts_query, subquery, db_conn, true)?;
            Ok(())
        }
        _ => Ok(()),
    }
}
