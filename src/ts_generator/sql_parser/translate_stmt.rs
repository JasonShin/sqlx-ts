use crate::ts_generator::errors::TsGeneratorError;
use crate::{core::connection::DBConn, ts_generator::sql_parser::translate_insert::translate_insert_returning};

use crate::ts_generator::sql_parser::quoted_strings::DisplayObjectName;
use crate::ts_generator::sql_parser::translate_delete::translate_delete;
use crate::ts_generator::sql_parser::translate_insert::translate_insert;
use crate::ts_generator::sql_parser::translate_query::translate_query;
use crate::ts_generator::sql_parser::translate_update::translate_update;
use crate::ts_generator::types::ts_query::TsQuery;

use sqlparser::ast::{FromTable, Statement};
use super::expressions::translate_table_with_joins::get_default_table;

pub async fn translate_stmt(
  ts_query: &mut TsQuery,
  sql_statement: &Statement,
  alias: Option<&str>, // If the statement is originated from a subquery, it must have an alias provided
  db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
  match sql_statement {
    Statement::Query(query) => {
      translate_query(ts_query, &None, query, db_conn, alias, true).await?;
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
      returning,
      ignore: _,
      table_alias: _,
      replace_into: _,
      priority: _,
      insert_alias: _,
    } => {
      let source = *source.to_owned().unwrap();
      let table_name = DisplayObjectName(table_name).to_string();
      let table_name = table_name.as_str();
      let query_for_logging = sql_statement.to_string();
      let query_for_logging = &query_for_logging.as_str();
      translate_insert(ts_query, columns, &source, table_name, db_conn).await?;
      if returning.is_some() {
        let returning = returning.clone();
        let returning = &returning.unwrap();
        translate_insert_returning(ts_query, returning, table_name, db_conn, query_for_logging).await;
      }
    }
    Statement::Delete {
      tables: _,
      selection,
      using: _,
      returning: _,
      from,
      order_by: _,
      limit: _,
    } => match &from {
      FromTable::WithFromKeyword(from) => {
        let table_name = get_default_table(from);
        let table_name = table_name.as_str();
        let selection = selection.to_owned().unwrap();
        translate_delete(ts_query, &selection, table_name, db_conn).await?;
      }
      FromTable::WithoutKeyword(_) => Err(TsGeneratorError::FromWithoutKeyword(sql_statement.to_string()))?,
    },
    Statement::Update {
      table,
      assignments,
      from,
      selection,
      returning: _,
    } => {
      translate_update(ts_query, table, assignments, from, selection, db_conn).await?;
    }
    _ => {}
  }
  Ok(())
}
