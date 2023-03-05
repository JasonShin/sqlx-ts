use std::collections::HashMap;

use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::{TsFieldType, TsQuery};
use sqlparser::ast::Expr;

use super::expressions::translate_where_stmt::translate_where_stmt;

pub fn translate_delete(
    ts_query: &mut TsQuery,
    where_conditions: &Expr, // WHERE conditions of the delete statement
    db_name: &str,
    table_name: &str,
    annotated_results: &HashMap<String, Vec<TsFieldType>>,
    db_conn: &DBConn,
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
