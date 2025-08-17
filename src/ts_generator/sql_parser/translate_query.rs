use sqlparser::ast::{Query, Select, SelectItem, SetExpr, TableWithJoins};

use super::expressions::{
  translate_expr::translate_expr, translate_table_with_joins::translate_table_with_joins,
  translate_wildcard_expr::translate_wildcard_expr,
};
use crate::ts_generator::sql_parser::quoted_strings::DisplayIndent;
use crate::{
  core::connection::DBConn,
  ts_generator::{
    errors::TsGeneratorError, sql_parser::expressions::translate_table_with_joins::get_default_table,
    types::ts_query::TsQuery,
  },
};

pub async fn translate_select(
  ts_query: &mut TsQuery,
  table_with_joins: &Option<Vec<TableWithJoins>>,
  select: &Box<Select>,
  db_conn: &DBConn,
  alias: Option<&str>,
  is_selection: bool,
) -> Result<(), TsGeneratorError> {
  let projection = select.clone().projection;

  // We create a new table with joins within the scope of this select (it could be within a subquery)
  // full_table_with_joins should contain all tables from the parent and keeping it within this query's scope
  let mut full_table_with_joins: Vec<TableWithJoins> = vec![];
  let child_table_with_joins = select.clone().from;

  // The most inner table should be extended first as it will be the default table within the subquery
  full_table_with_joins.extend(child_table_with_joins.clone());

  if table_with_joins.is_some() {
    let table_with_joins = table_with_joins.as_ref().unwrap();
    full_table_with_joins.extend(table_with_joins.clone());
  }

  let full_table_with_joins = &Some(full_table_with_joins.clone());

  // Handle all select projects and figure out each field's type
  for select_item in projection {

    // Determine the default table name within the scope of this select item
    let mut table_name_owned: Option<String> = None;
    let mut table_name: Option<&str> = None;
    if full_table_with_joins.is_some() && !full_table_with_joins.as_ref().unwrap().is_empty() {
      table_name_owned = Some(
        translate_table_with_joins(full_table_with_joins, &select_item)
          .unwrap_or_else(|_| panic!("{}", format!("Default FROM table is not found from the query {select}")))
      );
      table_name = table_name_owned.as_deref();
    }

    match &select_item {
      SelectItem::UnnamedExpr(unnamed_expr) => {

        translate_expr(
          unnamed_expr,
          &table_name,
          full_table_with_joins,
          alias,
          ts_query,
          db_conn,
          is_selection,
        )
        .await?;
      }
      SelectItem::ExprWithAlias { expr, alias } => {
        let alias = DisplayIndent(alias).to_string();

        translate_expr(
          expr,
          &table_name,
          full_table_with_joins,
          Some(alias.as_str()),
          ts_query,
          db_conn,
          is_selection,
        )
        .await?;
      }
      SelectItem::QualifiedWildcard(_, _) => {
        // TODO: If there's are two tables and two qualifieid wildcards are provided
        // It will simply generate types for both tables' columns
        // Should we namespace each field based on the table alias? e.g. table1_field1, table2_field1
        if is_selection {
          translate_wildcard_expr(select, ts_query, db_conn).await?;
        }
      }
      SelectItem::Wildcard(_) => {
        if is_selection {
          translate_wildcard_expr(select, ts_query, db_conn).await?;
        }
      }
    }
  }

  // If there's any WHERE statements, process it
  if let Some(selection) = &select.selection {
    let current_scope_table_name = get_default_table(&child_table_with_joins);
    let current_scope_table_name = current_scope_table_name.as_str();
    translate_expr(
      selection,
      &Some(current_scope_table_name),
      full_table_with_joins,
      None,
      ts_query,
      db_conn,
      false,
    )
    .await?;
  }
  Ok(())
}

/// Translates a query and workout ts_query's results and params
pub async fn translate_query(
  ts_query: &mut TsQuery,
  // this parameter is used to stack table_with_joins while recursing through subqueries
  // If there is only 1 entry of table_with_joins, it means it's processing the top level entity
  table_with_joins: &Option<Vec<TableWithJoins>>,
  query: &Query,
  db_conn: &DBConn,
  alias: Option<&str>,
  is_selection: bool,
) -> Result<(), TsGeneratorError> {
  let body = *query.body.clone();
  match body {
    SetExpr::Select(select) => {
      translate_select(ts_query, table_with_joins, &select, db_conn, alias, is_selection).await
    }
    _ => Err(TsGeneratorError::Unknown(format!(
      "Unknown query type while processing query: {query}"
    ))),
  }
}
