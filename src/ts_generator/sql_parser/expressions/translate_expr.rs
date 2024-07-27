use super::functions::{is_date_function, is_numeric_function};
use crate::common::lazy::DB_SCHEMA;
use crate::common::logger::warning;
use crate::core::connection::DBConn;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::expressions::translate_data_type::translate_value;
use crate::ts_generator::sql_parser::expressions::translate_table_with_joins::translate_table_from_expr;
use crate::ts_generator::sql_parser::expressions::{
  functions::is_string_function, translate_data_type::translate_data_type,
};
use crate::ts_generator::sql_parser::quoted_strings::DisplayIndent;
use crate::ts_generator::sql_parser::translate_query::translate_query;
use crate::ts_generator::types::ts_query::{TsFieldType, TsQuery};
use async_recursion::async_recursion;
use color_eyre::Result;
use regex::Regex;
use sqlparser::ast::{Assignment, Expr, TableWithJoins, Value};

/// Given an expression
/// e.g.
/// WHERE
///    some_column = ?
///
/// e.g.
/// WHERE
///     some_table.some_column = ?
///
/// it should receive `?` or `$1` and determine that it is a placeholder expression
///
/// also it should be able to process Postgres binding parameter expressions
///
/// e.g.
/// WHERE
///   some_table.some_column = $1
///
/// For binding parameters with index requirements such as PostgreSQL queries, it should return
/// the proper index value (e.g. 1, 2, 3). If the query is a query without indexed binding parameters
/// it should return None
pub fn get_expr_placeholder(expr: &Expr) -> Option<String> {
  let re = Regex::new(r"(\$\d+)").unwrap();
  if let Expr::Value(Value::Placeholder(placeholder)) = &expr {
    let indexed_binding_params = re.captures(placeholder);
    if placeholder == "?" {
      return Some("?".to_string());
    } else if indexed_binding_params.is_some() {
      // Rarely we will get an unwrap issue at this point because invalid syntax should be caught
      // during `prepare` step
      let placeholder = indexed_binding_params.unwrap().get(1).unwrap().as_str().to_string();

      return Some(placeholder);
    }
  }

  None
}

/// Given an expression
/// e.g.
/// WHERE
///    some_column = ?
///
/// or a compound identifier
///
/// e.g.
/// WHERE
///     some_table.some_column = ?
///
/// it should return the correct column name
pub fn translate_column_name_expr(expr: &Expr) -> Option<String> {
  match expr {
    Expr::Identifier(ident) => Some(DisplayIndent(ident).to_string()),
    Expr::CompoundIdentifier(comp) => Some(DisplayIndent(comp.get(1).unwrap()).to_string()),
    _ => None,
  }
}

pub fn translate_column_name_assignment(assignment: &Assignment) -> Option<String> {
  let left = assignment.id.first();
  let right = assignment.id.get(1);

  if left.is_some() && right.is_some() {
    return right.map(|x| x.to_string());
  } else if left.is_some() && right.is_none() {
    return left.map(|x| x.to_string());
  }
  None
}

/// handle an expression from where clauses (or it can be from anywhere)
/// pick up any expression from left and right that goes
/// some_field = ?
/// some_table.some_field = ?
///
/// or
///
/// some_field = $1
/// some_table.some_field = $1
pub async fn get_sql_query_param(
  left: &Box<Expr>,
  right: &Box<Expr>,
  single_table_name: &Option<&str>,
  table_with_joins: &Option<Vec<TableWithJoins>>,
  db_conn: &DBConn,
) -> Option<(TsFieldType, Option<String>)> {
  let table_name: Option<String>;

  if table_with_joins.is_some() {
    table_name = translate_table_from_expr(table_with_joins, &left.clone()).ok();
  } else if single_table_name.is_some() {
    table_name = single_table_name.map(|x| x.to_string());
  } else {
    panic!("failed to find an appropriate table name while processing WHERE statement")
  }

  let column_name = translate_column_name_expr(left);

  // If the right side of the expression is a placeholder `?` or `$n`
  // they are valid query parameter to process
  let expr_placeholder = get_expr_placeholder(right);

  match (column_name, expr_placeholder, table_name) {
    (Some(column_name), Some(expr_placeholder), Some(table_name)) => {
      let table_names = vec![table_name.as_str()];
      let columns = DB_SCHEMA
        .lock()
        .await
        .fetch_table(&table_names, db_conn)
        .await
        .unwrap_or_else(|| panic!("Failed to fetch columns for table {:?}", table_name));

      // get column and return TsFieldType
      let column = columns
        .get(column_name.as_str())
        .unwrap_or_else(|| panic!("Failed to find the column from the table schema of {:?}", table_name));
      Some((column.field_type.to_owned(), Some(expr_placeholder)))
    }
    _ => None,
  }
}

#[async_recursion]
pub async fn translate_expr(
  expr: &Expr,
  single_table_name: &Option<&str>,
  table_with_joins: &Option<Vec<TableWithJoins>>,
  alias: Option<&'async_recursion str>,
  ts_query: &mut TsQuery,
  db_conn: &DBConn,
  // is subquery determines if we can safely append result types into ts_query.results
  // subqueries on WHERE expression should no determine the SELECTIONs
  is_selection: bool,
) -> Result<(), TsGeneratorError> {
  let binding = expr.to_string();
  let expr_for_logging = &binding.as_str();

  match expr {
    Expr::Identifier(ident) => {
      let column_name = DisplayIndent(ident).to_string();
      let table_name = single_table_name.expect("Missing table name for identifier");
      let table_details = &DB_SCHEMA.lock().await.fetch_table(&vec![table_name], db_conn).await;

      // TODO: We can also memoize this method
      if let Some(table_details) = table_details {
        let field = table_details.get(&column_name).unwrap();

        let field_name = alias.unwrap_or(column_name.as_str());
        ts_query.insert_result(
          Some(field_name),
          &[field.field_type.to_owned()],
          is_selection,
          expr_for_logging,
        )?
      }
      Ok(())
    }
    Expr::CompoundIdentifier(idents) => {
      if idents.len() == 2 {
        let ident = DisplayIndent(&idents[1]).to_string();

        let table_name = translate_table_from_expr(table_with_joins, expr)?;

        let table_details = &DB_SCHEMA
          .lock()
          .await
          .fetch_table(&vec![table_name.as_str()], db_conn)
          .await;

        if let Some(table_details) = table_details {
          let field = table_details.get(&ident).unwrap();

          // if the select item is a compound identifier and does not has an alias, we should use `table_name.ident` as the key name
          let key_name = format!("{}_{}", table_name, ident);
          let key_name = &alias.unwrap_or_else(|| {
                        warning!(
                            "Missing an alias for a compound identifier, using {} as the key name. Prefer adding an alias for example: `{} AS {}`",
                            key_name, expr, ident
                        );
                        key_name.as_str()
                    });

          ts_query.insert_result(
            Some(key_name),
            &[field.field_type.to_owned()],
            is_selection,
            expr_for_logging,
          )?;
        }
      }
      Ok(())
    }
    /////////////////////
    // OPERATORS START //
    /////////////////////
    Expr::BinaryOp { left, op: _, right } => {
      let param = get_sql_query_param(left, right, single_table_name, table_with_joins, db_conn).await;
      if let Some((value, index)) = param {
        let _ = ts_query.insert_param(&value, &index);
        Ok(())
      } else {
        translate_expr(
          left,
          single_table_name,
          table_with_joins,
          alias,
          ts_query,
          db_conn,
          is_selection,
        )
        .await?;
        translate_expr(
          right,
          single_table_name,
          table_with_joins,
          alias,
          ts_query,
          db_conn,
          is_selection,
        )
        .await?;
        Ok(())
      }
    }
    Expr::InList { expr, list, negated: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Boolean], is_selection, expr_for_logging)?;
      // If the list is just a single `(?)`, then we should return the dynamic
      // If the list contains multiple `(?, ?...)` then we should return a fixed length array
      if list.len() == 1 {
        let right = list
          .first()
          .expect("Failed to find the first list item from the IN query");
        let result = get_sql_query_param(
          expr,
          &Box::new(right.to_owned()),
          single_table_name,
          table_with_joins,
          db_conn,
        )
        .await;

        if let Some((value, index)) = result {
          let array_item = TsFieldType::Array(Box::new(value));

          let _ = ts_query.insert_param(&array_item, &index);
          return Ok(());
        } else {
          return Ok(());
        }
      }
      Ok(())
    }
    Expr::InSubquery {
      expr: _,
      subquery,
      negated: _,
    } => {
      // You do not need an alias as we are processing a subquery within the WHERE clause
      translate_query(ts_query, &None, subquery, db_conn, None, false).await?;
      Ok(())
    }
    Expr::Between {
      expr,
      negated: _,
      low,
      high,
    } => {
      let low = get_sql_query_param(expr, low, single_table_name, table_with_joins, db_conn).await;
      let high = get_sql_query_param(expr, high, single_table_name, table_with_joins, db_conn).await;
      if let Some((value, placeholder)) = low {
        ts_query.insert_param(&value, &placeholder)?;
      }

      if let Some((value, placeholder)) = high {
        ts_query.insert_param(&value, &placeholder)?;
      }
      Ok(())
    }
    Expr::AnyOp {
      left: _,
      compare_op: _,
      right: expr,
    }
    | Expr::AllOp {
      left: _,
      compare_op: _,
      right: expr,
    } => {
      translate_expr(
        expr,
        single_table_name,
        table_with_joins,
        alias,
        ts_query,
        db_conn,
        is_selection,
      )
      .await
    }
    Expr::UnaryOp { op: _, expr } => {
      translate_expr(
        expr,
        single_table_name,
        table_with_joins,
        alias,
        ts_query,
        db_conn,
        is_selection,
      )
      .await
    }
    Expr::Value(placeholder) => {
      let ts_field_type = translate_value(placeholder);

      if let Some(ts_field_type) = ts_field_type {
        return ts_query.insert_result(alias, &[ts_field_type], is_selection, expr_for_logging);
      }
      ts_query.insert_param(&TsFieldType::Boolean, &Some(placeholder.to_string()))
    }
    Expr::JsonAccess {
      left: _,
      operator: _,
      right: _,
    } => {
      ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)?;
      ts_query.insert_param(&TsFieldType::Any, &None)
    }
    Expr::IsNotDistinctFrom(_, placeholder) | Expr::IsDistinctFrom(_, placeholder) => {
      // IsDistinctFrom and IsNotDistinctFrom are the same and can have a placeholder
      ts_query.insert_param(&TsFieldType::String, &Some(placeholder.to_string()))?;
      ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
    }
    Expr::SimilarTo {
      negated: _,
      expr: _,
      pattern,
      escape_char: _,
    }
    | Expr::ILike {
      negated: _,
      expr: _,
      pattern,
      escape_char: _,
    }
    | Expr::Like {
      negated: _,
      expr: _,
      pattern,
      escape_char: _,
    } => {
      // If the pattern has a placeholder, then we should append the param to ts_query
      ts_query.insert_param(&TsFieldType::String, &Some(pattern.to_string()))
    }
    Expr::TryCast {
      expr,
      data_type,
      format: _,
    }
    | Expr::SafeCast {
      expr,
      data_type,
      format: _,
    }
    | Expr::Cast {
      expr,
      data_type,
      format: _,
    } => {
      let data_type = translate_data_type(data_type);
      ts_query.insert_result(alias, &[data_type.clone()], is_selection, expr_for_logging)?;
      ts_query.insert_param(&data_type, &Some(expr.to_string()))?;
      Ok(())
    }
    Expr::AtTimeZone { timestamp, time_zone } => {
      ts_query.insert_result(alias, &[TsFieldType::Date], is_selection, expr_for_logging)?;
      ts_query.insert_param(&TsFieldType::String, &Some(timestamp.to_string()))?;
      ts_query.insert_param(&TsFieldType::String, &Some(time_zone.to_string()))?;
      Ok(())
    }
    Expr::Extract { field, expr } => {
      ts_query.insert_result(alias, &[TsFieldType::Date], is_selection, expr_for_logging)?;
      ts_query.insert_param(&TsFieldType::String, &Some(field.to_string()))?;
      ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))?;
      Ok(())
    }
    Expr::Floor { expr, field: _ } | Expr::Ceil { expr, field: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Number], is_selection, expr_for_logging)?;
      ts_query.insert_param(&TsFieldType::Number, &Some(expr.to_string()))
    }
    Expr::Position { expr: _, r#in: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Number], is_selection, expr_for_logging)
    }
    Expr::Substring {
      expr,
      substring_from: _,
      substring_for: _,
      special: _,
    } => {
      ts_query.insert_result(alias, &[TsFieldType::String], is_selection, expr_for_logging)?;
      ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))
    }
    Expr::Trim {
      expr,
      trim_where: _,
      trim_what: _,
      trim_characters: _,
    } => {
      ts_query.insert_result(alias, &[TsFieldType::String], is_selection, expr_for_logging)?;
      ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))
    }
    Expr::Overlay {
      expr,
      overlay_what,
      overlay_from,
      overlay_for: _,
    } => {
      ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))?;
      ts_query.insert_param(&TsFieldType::String, &Some(overlay_what.to_string()))?;
      ts_query.insert_param(&TsFieldType::Number, &Some(overlay_from.to_string()))?;
      ts_query.insert_result(alias, &[TsFieldType::String], is_selection, expr_for_logging)
    }
    Expr::Collate { expr: _, collation: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
    }
    Expr::IntroducedString {
      introducer: _,
      value: _,
    } => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging),
    Expr::TypedString { data_type: _, value: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
    }
    Expr::MapAccess { column: _, keys: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
    }
    Expr::AggregateExpressionWithFilter { expr: _, filter: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
    }
    Expr::Case {
      operand: _,
      conditions: _,
      results: _,
      else_result: _,
    } => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging),
    Expr::Exists { subquery, negated: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Boolean], is_selection, expr_for_logging)?;
      translate_query(ts_query, &None, subquery, db_conn, alias, false).await
    }
    Expr::ListAgg(_)
    | Expr::ArrayAgg(_)
    | Expr::GroupingSets(_)
    | Expr::Cube(_)
    | Expr::Rollup(_)
    | Expr::Tuple(_)
    | Expr::Array(_)
    | Expr::ArraySubquery(_) => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging),
    Expr::ArrayIndex { obj: _, indexes: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
    }
    Expr::Interval(_) => ts_query.insert_result(alias, &[TsFieldType::Number], is_selection, expr_for_logging),
    Expr::MatchAgainst {
      columns: _,
      match_value: _,
      opt_search_modifier: _,
    } => ts_query.insert_result(alias, &[TsFieldType::Number], is_selection, expr_for_logging),
    /////////////////////
    // OPERATORS ENDS  //
    /////////////////////

    /////////////////////
    // FUNCTIONS START //
    /////////////////////
    Expr::IsTrue(_query) | Expr::IsFalse(_query) | Expr::IsNull(_query) | Expr::IsNotNull(_query) => {
      ts_query.insert_result(alias, &[TsFieldType::Boolean], is_selection, expr.to_string().as_str())
    }
    Expr::Function(function) => {
      let function = function.name.to_string();
      let function = function.as_str();
      let alias = alias.ok_or(TsGeneratorError::FunctionWithoutAliasInSelectClause(expr.to_string()))?;
      if is_string_function(function) {
        ts_query.insert_result(Some(alias), &[TsFieldType::String], is_selection, expr_for_logging)?;
      } else if is_numeric_function(function) {
        ts_query.insert_result(Some(alias), &[TsFieldType::Number], is_selection, expr_for_logging)?;
      } else if is_date_function(function) {
        ts_query.insert_result(Some(alias), &[TsFieldType::String], is_selection, expr_for_logging)?;
      } else {
        return Err(TsGeneratorError::FunctionUnknown(expr.to_string()));
      }

      Ok(())
    }
    /////////////////////
    // FUNCTIONS END //
    /////////////////////
    Expr::CompositeAccess { expr: _, key: _ } => {
      ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
    }
    Expr::Subquery(sub_query) => {
      // For the first layer of subquery, we consider the first field selected as the result
      if is_selection && table_with_joins.clone().unwrap().len() == 1 {
        return translate_query(ts_query, table_with_joins, sub_query, db_conn, alias, true).await;
      }
      translate_query(ts_query, table_with_joins, sub_query, db_conn, alias, false).await
    }
    Expr::Nested(expr) => {
      translate_expr(
        expr,
        single_table_name,
        table_with_joins,
        alias,
        ts_query,
        db_conn,
        is_selection,
      )
      .await
    }
    Expr::InUnnest {
      expr: _,
      array_expr: _,
      negated: _,
    } => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging),
    _ => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging),
  }
}

pub async fn translate_assignment(
  assignment: &Assignment,
  table_name: &str,
  ts_query: &mut TsQuery,
  db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
  let value = get_expr_placeholder(&assignment.value);

  if value.is_some() {
    let table_details = &DB_SCHEMA
      .lock()
      .await
      .fetch_table(&vec![table_name], db_conn)
      .await
      .unwrap();
    let column_name = translate_column_name_assignment(assignment).unwrap();
    let field = table_details
      .get(&column_name)
      .unwrap_or_else(|| panic!("Failed to find the column detail for {column_name}"));
    let _ = ts_query.insert_param(&field.field_type, &value);
  }
  Ok(())
}
