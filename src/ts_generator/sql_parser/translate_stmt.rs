use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::translate_insert::translate_insert;
use crate::ts_generator::sql_parser::translate_query::translate_query;
use crate::ts_generator::types::{DBConn, TsFieldType, TsQuery};

use sqlparser::ast::Statement;
use std::collections::HashMap;

pub fn translate_stmt(
    ts_query: &mut TsQuery,
    sql_statement: &Statement,
    db_name: &str,
    annotated_results: &HashMap<String, Vec<TsFieldType>>,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    match sql_statement {
        Statement::Query(query) => {
            translate_query(
                ts_query,
                None,
                sql_statement,
                query,
                db_name,
                annotated_results,
                db_conn,
                false,
            )?;
        }
        Statement::Update { .. } => {
            println!("UPDATE statement is not yet supported by TS type generator")
        }
        Statement::Delete { .. } => {
            println!("DELETE statement is not yet supported by TS type generator")
        }
        Statement::Insert {
            or: _,
            into: _,
            table_name,
            columns,
            overwrite: _,
            source,
            partitioned: _,
            after_columns: _,
            table,
            on: _,
        } => {
            let table_name = table_name.to_string();
            let table_name = table_name.as_str();
            translate_insert(ts_query, columns, source, db_name, table_name, db_conn)?;
        }
        _ => {
            println!("Unsupported SQL syntax detected, skipping the type generation")
        }
    }
    Ok(())
}
