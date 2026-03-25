use crate::common::lazy::DB_SCHEMA;
use crate::core::connection::DBConn;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::expressions::function_handlers::FunctionHandlersContext;
use crate::ts_generator::sql_parser::expressions::translate_data_type::translate_value;
use crate::ts_generator::sql_parser::expressions::translate_table_with_joins::translate_table_from_expr;
use crate::ts_generator::sql_parser::quoted_strings::DisplayIndent;
use crate::ts_generator::types::ts_query::TsFieldType;
use sqlparser::ast::{Expr, FunctionArg, FunctionArgExpr, TableWithJoins, Value};

/// Extract key name from a function argument (should be a string literal)
fn extract_key_name(arg: &FunctionArg) -> Option<String> {
  match arg {
    FunctionArg::Unnamed(FunctionArgExpr::Expr(Expr::Value(val))) => match &val.value {
      Value::SingleQuotedString(s) | Value::DoubleQuotedString(s) => Some(s.clone()),
      _ => None,
    },
    _ => None,
  }
}

/// Extract expression from a function argument
fn extract_expr_from_arg(arg: &FunctionArg) -> Option<&Expr> {
  match arg {
    FunctionArg::Unnamed(FunctionArgExpr::Expr(expr)) => Some(expr),
    FunctionArg::Named {
      arg: FunctionArgExpr::Expr(expr),
      ..
    } => Some(expr),
    _ => None,
  }
}

/// Infer the TypeScript type from an SQL expression
pub async fn infer_type_from_expr(
  expr: &Expr,
  single_table_name: &Option<&str>,
  table_with_joins: &Option<Vec<TableWithJoins>>,
  db_conn: &DBConn,
) -> Option<(TsFieldType, bool)> {
  match expr {
    Expr::Identifier(ident) => {
      let column_name = DisplayIndent(ident).to_string();
      if let Some(table_name) = single_table_name {
        let table_details = DB_SCHEMA.lock().await.fetch_table(&vec![table_name], db_conn).await;

        if let Some(table_details) = table_details {
          if let Some(field) = table_details.get(&column_name) {
            Some((field.field_type.to_owned(), field.is_nullable))
          } else {
            Some((TsFieldType::Any, false))
          }
        } else {
          Some((TsFieldType::Any, false))
        }
      } else {
        Some((TsFieldType::Any, false))
      }
    }
    Expr::CompoundIdentifier(idents) if idents.len() == 2 => {
      let column_name = DisplayIndent(&idents[1]).to_string();
      if let Ok(table_name) = translate_table_from_expr(table_with_joins, expr) {
        let table_details = DB_SCHEMA
          .lock()
          .await
          .fetch_table(&vec![table_name.as_str()], db_conn)
          .await;

        if let Some(table_details) = table_details {
          if let Some(field) = table_details.get(&column_name) {
            Some((field.field_type.to_owned(), field.is_nullable))
          } else {
            Some((TsFieldType::Any, false))
          }
        } else {
          Some((TsFieldType::Any, false))
        }
      } else {
        Some((TsFieldType::Any, false))
      }
    }
    Expr::Value(val) => {
      if let Some(ts_field_type) = translate_value(&val.value) {
        Some((ts_field_type, false))
      } else {
        Some((TsFieldType::Any, false))
      }
    }
    _ => Some((TsFieldType::Any, false)),
  }
}

/// Process key-value pairs from JSON build object arguments
pub async fn process_json_build_object_args(
  args: &[FunctionArg],
  single_table_name: &Option<&str>,
  table_with_joins: &Option<Vec<TableWithJoins>>,
  db_conn: &DBConn,
) -> Option<Vec<(String, TsFieldType, bool)>> {
  if !args.len().is_multiple_of(2) {
    // Invalid number of arguments
    return None;
  }

  let mut object_fields = vec![];

  // Process key-value pairs
  for i in (0..args.len()).step_by(2) {
    let key_arg = &args[i];
    let value_arg = &args[i + 1];

    // Extract key name
    let key_name = extract_key_name(key_arg)?;

    // Extract value expression
    let value_expr = extract_expr_from_arg(value_arg)?;

    // Infer value type
    let (value_type, is_nullable) =
      infer_type_from_expr(value_expr, single_table_name, table_with_joins, db_conn).await?;

    object_fields.push((key_name, value_type, is_nullable));
  }

  Some(object_fields)
}

/// Handle JSON build functions (jsonb_build_object, json_build_object, etc.)
pub async fn handle_json_build_function(
  function_name: &str,
  args: &[FunctionArg],
  ctx: &mut FunctionHandlersContext<'_>,
) -> Result<(), TsGeneratorError> {
  let expr_log = ctx.expr_for_logging.unwrap_or("");

  // Handle jsonb_build_object / json_build_object / json_object (MySQL)
  let func_upper = function_name.to_uppercase();
  if func_upper == "JSONB_BUILD_OBJECT" || func_upper == "JSON_BUILD_OBJECT" || func_upper == "JSON_OBJECT" {
    if let Some(object_fields) =
      process_json_build_object_args(args, ctx.single_table_name, ctx.table_with_joins, ctx.db_conn).await
    {
      let object_type = TsFieldType::StructuredObject(object_fields);
      return ctx
        .ts_query
        .insert_result(Some(ctx.alias), &[object_type], ctx.is_selection, false, expr_log);
    }
  }

  // For other build functions or on failure, return Any
  ctx.ts_query.insert_result(
    Some(ctx.alias),
    &[TsFieldType::Unknown],
    ctx.is_selection,
    false,
    expr_log,
  )
}

/// Handle JSON aggregation functions (jsonb_agg, json_agg, etc.)
pub async fn handle_json_agg_function(
  args: &[FunctionArg],
  ctx: &mut FunctionHandlersContext<'_>,
) -> Result<(), TsGeneratorError> {
  use super::super::functions::is_json_build_function;
  use sqlparser::ast::FunctionArguments;

  let expr_log = ctx.expr_for_logging.unwrap_or("");

  // jsonb_agg typically takes a single expression
  if args.len() != 1 {
    return ctx
      .ts_query
      .insert_result(Some(ctx.alias), &[TsFieldType::Any], ctx.is_selection, false, expr_log);
  }

  let arg_expr = extract_expr_from_arg(&args[0]);

  // Check if the argument is a jsonb_build_object function
  if let Some(Expr::Function(inner_func)) = arg_expr {
    let inner_func_name = inner_func.name.to_string();
    if is_json_build_function(inner_func_name.as_str()) {
      // Extract arguments from the inner function
      let inner_args = match &inner_func.args {
        FunctionArguments::List(arg_list) => &arg_list.args,
        _ => {
          return ctx
            .ts_query
            .insert_result(Some(ctx.alias), &[TsFieldType::Any], ctx.is_selection, false, expr_log);
        }
      };

      // Process the inner jsonb_build_object
      if let Some(object_fields) =
        process_json_build_object_args(inner_args, ctx.single_table_name, ctx.table_with_joins, ctx.db_conn).await
      {
        let object_type = TsFieldType::StructuredObject(object_fields);
        let array_type = TsFieldType::Array(Box::new(object_type));
        return ctx
          .ts_query
          .insert_result(Some(ctx.alias), &[array_type], ctx.is_selection, false, expr_log);
      }
    }
  }

  // If we can't infer the type, return Any
  ctx
    .ts_query
    .insert_result(Some(ctx.alias), &[TsFieldType::Any], ctx.is_selection, false, expr_log)
}
