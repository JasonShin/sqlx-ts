use crate::core::connection::DBConn;
use crate::ts_generator::errors::TsGeneratorError;

use crate::ts_generator::sql_parser::translate_delete::translate_delete;
use crate::ts_generator::sql_parser::translate_insert::translate_insert;
use crate::ts_generator::sql_parser::translate_query::translate_query;
use crate::ts_generator::sql_parser::translate_update::translate_update;
use crate::ts_generator::types::ts_query::TsQuery;

use sqlparser::ast::Statement;

pub fn translate_stmt(
    ts_query: &mut TsQuery,
    sql_statement: &Statement,
    alias: Option<&str>, // If the statement is originated from a subquery, it must have an alias provided
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    match sql_statement {
        Statement::Query(query) => {
            translate_query(ts_query, &mut None, query, db_conn, alias, true)?;
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
            returning: _,
        } => {
            let table_name = table_name.to_string();
            let table_name = table_name.as_str();
            translate_insert(ts_query, columns, source, table_name, db_conn)?;
        }
        Statement::Delete {
            table_name,
            selection,
            using: _,
            returning: _,
        } => {
            let table_name = table_name.to_string();
            let table_name = table_name.as_str();
            let selection = selection.to_owned().unwrap();
            translate_delete(ts_query, &selection, table_name, db_conn)?;
        }
        Statement::Update {
            table,
            assignments,
            from,
            selection,
            returning: _,
        } => {
            translate_update(ts_query, table, assignments, from, selection, db_conn)?;
        }
        _ => {}
    }
    Ok(())
}
