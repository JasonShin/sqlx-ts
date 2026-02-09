use crate::common::lazy::DB_SCHEMA;
use crate::common::logger::warning;
use crate::core::connection::DBConn;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::quoted_strings::DisplayObjectName;
use crate::ts_generator::types::ts_query::TsFieldType;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;
use sqlparser::ast::{Join, Select, TableFactor};

pub fn get_all_table_names_from_select(select: &Select) -> Result<Vec<String>, TsGeneratorError> {
  let table_with_joins = select
    .from
    .first()
    .ok_or(TsGeneratorError::WildcardStatementWithoutTargetTables(
      select.to_string(),
    ))?
    .to_owned();

  let primary_table_name = match table_with_joins.relation {
    TableFactor::Table { name, .. } => {
      let name = DisplayObjectName(&name).to_string();
      Ok(name)
    }
    TableFactor::Function { .. } => {
      // Wildcard queries with table-valued functions are not supported
      // because we cannot query the database schema for function result types
      Err(TsGeneratorError::WildcardStatementUnsupportedTableExpr(
        select.to_string(),
      ))
    }
    _ => Err(TsGeneratorError::WildcardStatementUnsupportedTableExpr(
      select.to_string(),
    )),
  }?;

  let tables = &mut vec![primary_table_name];

  for join in &table_with_joins.joins {
    let Join { relation, .. } = join;
    match relation {
      TableFactor::Table { name, .. } => {
        let name = DisplayObjectName(name).to_string();
        tables.push(name);
      }
      _ => {
        return Err(TsGeneratorError::WildcardStatementDeadendExpression(
          relation.to_string(),
        ))
      }
    }
  }

  Ok(tables.clone())
}

/// Translates a wildcard expression of a SQL statement
/// @example
/// SELECT * FROM items
///
/// and it appends result into the hashmap for type generation
pub async fn translate_wildcard_expr(
  select: &Select,
  ts_query: &mut TsQuery,
  db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
  let table_names = get_all_table_names_from_select(select)?;

  // Check if the table is a CTE or table-valued function registered in table_valued_function_columns.
  // CTEs are processed before the main query body and their columns are stored there.
  for table_name in &table_names {
    if let Some(tvf_columns) = ts_query.table_valued_function_columns.get(table_name).cloned() {
      for (col_name, ts_type) in tvf_columns {
        ts_query.result.insert(col_name, vec![ts_type]);
      }
      return Ok(());
    }
  }

  if table_names.len() > 1 {
    warning!("Impossible to calculate appropriate field names of a wildcard query with multiple tables. Please use explicit field names instead. Query: {}", select.to_string());
  }

  let table_refs = table_names.iter().map(|s| s.as_ref()).collect();
  let all_fields = DB_SCHEMA.lock().await.fetch_table(&table_refs, db_conn).await;

  if let Some(all_fields) = all_fields {
    for key in all_fields.keys() {
      let field = all_fields.get(key).unwrap();
      let mut field_types = vec![field.field_type.clone()];
      if field.is_nullable {
        field_types.push(TsFieldType::Null);
      }

      ts_query.result.insert(key.to_owned(), field_types);
    }
  }
  Ok(())
}
