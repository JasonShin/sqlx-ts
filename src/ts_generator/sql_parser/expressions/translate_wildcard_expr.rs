use crate::common::lazy::DB_SCHEMA;
use crate::common::logger::warning;
use crate::core::connection::DBConn;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::quoted_strings::DisplayObjectName;
use crate::ts_generator::types::ts_query::TsFieldType;
use crate::ts_generator::types::ts_query::TsQuery;
use color_eyre::eyre::Result;
use sqlparser::ast::{Join, Select, TableFactor};

pub fn get_all_table_names_from_select(select: &Box<Select>) -> Result<Vec<String>, TsGeneratorError> {
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
  select: &Box<Select>,
  ts_query: &mut TsQuery,
  db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
  let table_with_joins = get_all_table_names_from_select(select)?;

  if table_with_joins.len() > 1 {
    warning!("Impossible to calculate appropriate field names of a wildcard query with multiple tables. Please use explicit field names instead. Query: {}", select.to_string());
  }

  let table_with_joins = table_with_joins.iter().map(|s| s.as_ref()).collect();
  let all_fields = DB_SCHEMA.lock().await.fetch_table(&table_with_joins, db_conn).await;

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
