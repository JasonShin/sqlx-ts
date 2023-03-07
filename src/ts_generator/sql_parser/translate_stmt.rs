use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::translate_delete::translate_delete;
use crate::ts_generator::sql_parser::translate_insert::translate_insert;
use crate::ts_generator::sql_parser::translate_query::translate_query;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::{TsFieldType, TsQuery};

use sqlparser::ast::Statement;

pub fn translate_stmt(
    ts_query: &mut TsQuery,
    sql_statement: &Statement,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    match sql_statement {
        Statement::Query(query) => {
            translate_query(ts_query, None, &query, db_conn, false)?;
        }
        Statement::Update { .. } => {
            println!("UPDATE statement is not yet supported by TS type generator")
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
            table: _,
            on: _,
        } => {
            let table_name = table_name.to_string();
            let table_name = table_name.as_str();
            translate_insert(ts_query, &columns, &source, table_name, db_conn)?;
        }
        Statement::Delete { table_name, selection } => {
            let table_name = table_name.to_string();
            let table_name = table_name.as_str();
            let selection = selection.to_owned().unwrap();
            // translate_delete(&selection, db_name, table_name, db_conn)?;
        }
        _ => {}
    }
    Ok(())
}
