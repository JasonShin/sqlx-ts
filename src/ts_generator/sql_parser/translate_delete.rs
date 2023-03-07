use std::collections::HashMap;

use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::{TsFieldType, TsQuery};
use sqlparser::ast::Expr;

pub fn translate_delete(
    _ts_query: &mut TsQuery,
    _where_conditions: &Expr, // WHERE conditions of the delete statement
    _db_name: &str,
    _table_name: &str,
    _annotated_results: &HashMap<String, Vec<TsFieldType>>,
    _db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    /*
    translate_where_stmt(
        db_name,
        ts_query,
        sql_statement,
        where_conditions,
        &Some(db_name),
        &None,
        annotated_results,
        db_conn
    );
     */
    Ok(())
}
