use sqlparser::ast::Expr;
use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::{types::DBConn, errors::TsGeneratorError};

use super::expressions::translate_where_stmt;

pub fn translate_delete(
    selection: &Expr, // WHERE conditions of the delete statement
    db_name: &str,
    table_name: &str,
    conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    translate_where_stmt
    Ok(())
}
