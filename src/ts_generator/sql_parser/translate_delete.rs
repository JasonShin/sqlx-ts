use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::expressions::translate_expr::translate_expr;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::TsQuery;
use sqlparser::ast::Expr;

pub fn translate_delete(
    ts_query: &mut TsQuery,
    where_conditions: &Expr, // WHERE conditions of the delete statement
    table_name: &str,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    translate_expr(
        where_conditions,
        &Some(table_name),
        &None,
        None,
        ts_query,
        db_conn,
        false,
    )
}
