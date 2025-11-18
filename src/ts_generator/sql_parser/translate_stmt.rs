use crate::ts_generator::errors::TsGeneratorError;
use crate::{core::connection::DBConn, ts_generator::sql_parser::translate_insert::translate_insert_returning};
use async_recursion::async_recursion;

use crate::ts_generator::sql_parser::quoted_strings::DisplayObjectName;
use crate::ts_generator::sql_parser::translate_delete::translate_delete;
use crate::ts_generator::sql_parser::translate_insert::translate_insert;
use crate::ts_generator::sql_parser::translate_query::translate_query;
use crate::ts_generator::sql_parser::translate_update::translate_update;
use crate::ts_generator::types::ts_query::TsQuery;

use super::expressions::translate_table_with_joins::get_default_table;
use sqlparser::ast::{FromTable, Statement, TableObject, UpdateTableFromKind};

#[async_recursion]
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
    Statement::Insert(insert) => {
      let source = *insert.source.to_owned().unwrap();

      // Extract table name from TableObject
      let table_name = match &insert.table {
        TableObject::TableName(obj_name) => DisplayObjectName(obj_name).to_string(),
        TableObject::TableFunction(_) => {
          return Err(TsGeneratorError::Unknown(
            "INSERT into table function is not supported".to_string(),
          ));
        }
      };

      let table_name_str = table_name.as_str();
      let query_for_logging = sql_statement.to_string();
      let query_for_logging_str = &query_for_logging.as_str();

      translate_insert(ts_query, &insert.columns, &source, table_name_str, db_conn).await?;

      if insert.returning.is_some() {
        let returning = insert.returning.clone().unwrap();
        translate_insert_returning(ts_query, &returning, table_name_str, db_conn, query_for_logging_str).await?;
      }
    }
    Statement::Delete(delete) => match &delete.from {
      FromTable::WithFromKeyword(from) => {
        let table_name = get_default_table(from);
        let table_name_str = table_name.as_str();
        let selection = delete.selection.to_owned().unwrap();
        translate_delete(ts_query, &selection, table_name_str, db_conn).await?;

        // Handle RETURNING clause if present
        if delete.returning.is_some() {
          let returning = delete.returning.clone().unwrap();
          let query_for_logging = sql_statement.to_string();
          let query_for_logging_str = &query_for_logging.as_str();
          translate_insert_returning(ts_query, &returning, table_name_str, db_conn, query_for_logging_str).await?;
        }
      }
      FromTable::WithoutKeyword(_) => Err(TsGeneratorError::FromWithoutKeyword(sql_statement.to_string()))?,
    },
    Statement::Update {
      table,
      assignments,
      from,
      selection,
      returning,
      or: _,
      limit: _,
    } => {
      // Convert UpdateTableFromKind to Option<TableWithJoins>
      let from_table = match from {
        Some(UpdateTableFromKind::AfterSet(tables)) => {
          // For AfterSet, we take the first TableWithJoins if available
          tables.first().cloned()
        }
        Some(UpdateTableFromKind::BeforeSet(tables)) => {
          // For BeforeSet, we take the first TableWithJoins if available
          tables.first().cloned()
        }
        None => None,
      };

      translate_update(ts_query, table, assignments, &from_table, selection, db_conn).await?;

      // Handle RETURNING clause if present
      if returning.is_some() {
        let returning_items = returning.clone().unwrap();
        // Extract table name from TableWithJoins
        let table_name = get_default_table(&vec![table.clone()]);
        let table_name_str = table_name.as_str();
        let query_for_logging = sql_statement.to_string();
        let query_for_logging_str = &query_for_logging.as_str();
        translate_insert_returning(ts_query, &returning_items, table_name_str, db_conn, query_for_logging_str).await?;
      }
    }
    _ => {}
  }
  Ok(())
}
