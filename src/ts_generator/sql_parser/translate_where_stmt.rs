use std::collections::HashMap;

use sqlparser::ast::{Expr, Statement, TableWithJoins};

use crate::{
    common::config::GenerateTypesConfig,
    ts_generator::{
        errors::TsGeneratorError,
        information_schema::DBSchema,
        types::{DBConn, TsFieldType, TsQuery},
    },
};

use super::{
    translate_expr::{is_expr_placeholder, translate_column_name_expr},
    translate_stmt::translate_query,
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
    db_name: &str,
    table_with_joins: &Vec<TableWithJoins>,
    db_conn: &DBConn,
) -> Option<TsFieldType> {
    let db_schema = DBSchema::new();
    let table_name = translate_table_from_expr(table_with_joins, &*left.clone());
    let column_name = translate_column_name_expr(left);

    // If the right side of the expression is a placeholder `?` or `$n`
    // they are valid query parameter to process
    if column_name.is_some() && is_expr_placeholder(right) && table_name.is_some() {
        let table_name = table_name.unwrap();
        let table_names = vec![table_name.as_str()];
        let column_name = column_name.unwrap();
        let columns = db_schema
            .fetch_table(db_name, &table_names, db_conn)
            .unwrap_or_else(|| panic!("Failed to fetch columns for table {:?}", table_name));

        // get column and return TsFieldType
        let column = columns
            .get(column_name.as_str())
            .unwrap_or_else(|| panic!("Failed toe find the column from the table schema of {:?}", table_name));
        return Some(column.field_type);
    }

    None
}

pub fn translate_where_stmt(
    db_name: &str,
    ts_query: &mut TsQuery,
    sql_statement: &Statement,
    expr: &Expr,
    table_with_joins: &Vec<TableWithJoins>,
    annotated_results: &HashMap<String, Vec<TsFieldType>>,
    db_conn: &DBConn,
    generate_types_config: &Option<GenerateTypesConfig>,
) -> Result<(), TsGeneratorError> {
    match expr {
        Expr::BinaryOp { left, op: _, right } => {
            let result = get_sql_query_param(left, right, db_name, table_with_joins, db_conn);

            if result.is_none() {
                translate_where_stmt(
                    db_name,
                    ts_query,
                    sql_statement,
                    left,
                    table_with_joins,
                    annotated_results,
                    db_conn,
                    generate_types_config,
                )?;
                translate_where_stmt(
                    db_name,
                    ts_query,
                    sql_statement,
                    right,
                    table_with_joins,
                    annotated_results,
                    db_conn,
                    generate_types_config,
                )?;
            } else {
                ts_query.params.push(result.unwrap());
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
                let result = get_sql_query_param(expr, &Box::new(right.to_owned()), db_name, table_with_joins, db_conn);

                if result.is_some() {
                    let array_item = result.unwrap().to_array_item();

                    ts_query.params.push(array_item);
                    return Ok(());
                } else {
                    return Ok(());
                }
            }
            Ok(())
        }
        Expr::InSubquery {
            expr,
            subquery,
            negated,
        } => {
            translate_query(
                ts_query,
                sql_statement,
                subquery,
                db_name,
                annotated_results,
                db_conn,
                generate_types_config,
            );
            Ok(())
        }
        Expr::Subquery(subquery) => {
            // translate query here as well
            println!("checking sub query {:?}", subquery);
            Ok(())
        }
        _ => Ok(()),
    }
}
