use crate::common::lazy::DB_SCHEMA;
use crate::core::connection::DBConn;
use crate::ts_generator::sql_parser::expressions::translate_expr::{get_expr_placeholder, translate_expr};
use crate::ts_generator::sql_parser::quoted_strings::DisplayIndent;
use crate::ts_generator::{errors::TsGeneratorError, types::ts_query::TsQuery};
use color_eyre::Result;
use sqlparser::ast::{Ident, Query, Select, SelectItem, SetExpr};
use crate::ts_generator::sql_parser::translate_query::translate_query;

pub async fn translate_insert(
  ts_query: &mut TsQuery,
  columns: &[Ident],
  source: &Query,
  table_name: &str,
  conn: &DBConn,
) -> Result<(), TsGeneratorError> {
  let table_details = &DB_SCHEMA
    .lock()
    .await
    .fetch_table(&vec![table_name], conn)
    .await
    // Nearly impossible to panic at this point as we've already validated queries with prepare statements
    .unwrap();

  let values = *source.body.clone();

  match values {
    SetExpr::Values(values) => {
      // Process the rows
      for (row, values) in values.rows.iter().enumerate() {
        // Given a list of values
        // [ [?, 1, ?], [?, ?, ?] ]
        // Loop each value placeholder / actual values, if it finds the placeholder either `?` or `$n`
        // build the insert param in `types.rs`
        for (column, value) in values.iter().enumerate() {
          let placeholder = get_expr_placeholder(value);

          if placeholder.is_some() {
            let match_col = &columns
              .get(column)
              .unwrap_or_else(|| {
                panic!(
                  r#"
Failed to process values of insert statement as column names are not provided or incorrectly specified

Try specifying column names
```
INSERT INTO table_name (column1, column2, column3, ...)
VALUES (value1, value2, value3, ...);
```
              "#
                )
              })
              .value;

            let field = table_details
              .get(match_col.as_str())
              .unwrap_or_else(|| panic!("Column {match_col} is not found while processing insert params"));

            if value.to_string() == "?" {
              // If the placeholder is `'?'`, we can process it using insert_value_params and generate nested params type
              ts_query.insert_value_params(&field.field_type, &(row, column), &placeholder);
            } else {
              ts_query.insert_param(&field.field_type, &field.is_nullable, &placeholder)?;
            }
          }
        }
      }
    },
    SetExpr::Select(expr) => {
      let z = expr.from;
    }
    SetExpr::Query(query) => translate_query(ts_query, &None, &query, conn, None, false).await?,
    SetExpr::SetOperation { .. } => {}
    SetExpr::Insert(_) => {}
    SetExpr::Update(_) => {}
    SetExpr::Table(_) => {}
  }

  Ok(())
}

pub async fn translate_insert_returning(
  ts_query: &mut TsQuery,
  returning: &Vec<SelectItem>,
  table_name: &str,
  conn: &DBConn,
  query_for_logging: &str,
) -> Result<(), TsGeneratorError> {
  let table_details = &DB_SCHEMA
    .lock()
    .await
    .fetch_table(&vec![table_name], conn)
    .await
    // Nearly impossible to panic at this point as we've already validated queries with prepare statements
    .unwrap();

  for select_item in returning {
    match &select_item {
      SelectItem::UnnamedExpr(unnamed_expr) => {
        translate_expr(unnamed_expr, &Some(table_name), &None, None, ts_query, conn, true).await;
      }
      SelectItem::ExprWithAlias { expr, alias } => {
        let alias = DisplayIndent(alias).to_string();
        let alias = alias.as_str();
        translate_expr(expr, &Some(table_name), &None, Some(alias), ts_query, conn, true).await;
      }
      SelectItem::Wildcard(_) | SelectItem::QualifiedWildcard(_, _) => {
        let keys = table_details.keys();
        for key in keys {
          let field = table_details.get(key).unwrap();
          let value = vec![field.field_type.clone()];
          ts_query.insert_result(Some(key), &value, true, field.is_nullable, query_for_logging);
        }
      }
    }
  }

  Ok(())
}
