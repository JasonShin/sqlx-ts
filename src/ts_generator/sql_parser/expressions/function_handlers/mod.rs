use sqlparser::ast::TableWithJoins;
use crate::core::connection::DBConn;
use crate::ts_generator::types::ts_query::TsQuery;

pub mod json_functions;
pub mod polymorphic_functions;

/// Context for function type inference
pub struct FunctionHandlersContext<'a> {
  pub ts_query: &'a mut TsQuery,
  pub single_table_name: &'a Option<&'a str>,
  pub table_with_joins: &'a Option<Vec<TableWithJoins>>,
  pub db_conn: &'a DBConn,
  pub alias: &'a str,
  pub is_selection: bool,
  pub expr_for_logging: Option<&'a str>,
}