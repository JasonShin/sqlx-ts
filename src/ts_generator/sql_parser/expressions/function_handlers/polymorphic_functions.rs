use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::expressions::translate_data_type::translate_value;
use crate::ts_generator::sql_parser::expressions::translate_table_with_joins::translate_table_from_expr;
use crate::ts_generator::sql_parser::quoted_strings::DisplayIndent;
use crate::ts_generator::types::ts_query::{TsFieldType, TsQuery};
use sqlparser::ast::{Expr, Function, FunctionArg, FunctionArgExpr, FunctionArguments, TableWithJoins};

pub async fn handle_polymorphic_functions(
  ts_query: &mut TsQuery,
  single_table_name: &Option<&str>,
  table_with_joins: &Option<Vec<TableWithJoins>>,
  alias: &str,
  is_selection: bool,
  expr_for_logging: &str,
  func_obj: &Function,
  db_conn: &crate::core::connection::DBConn,
) -> Result<(), TsGeneratorError> {
  // In sqlparser 0.59.0, args is a FunctionArguments enum
  // Extract the first argument from the appropriate variant
  let first_arg = match &func_obj.args {
    FunctionArguments::List(arg_list) => arg_list.args.first(),
    FunctionArguments::None => None,
    FunctionArguments::Subquery(_) => None, // Can't infer type from subquery easily
  };

  if let Some(first_arg) = first_arg {
    let first_expr = match first_arg {
      FunctionArg::Unnamed(FunctionArgExpr::Expr(expr)) => Some(expr),
      FunctionArg::Named {
        arg: FunctionArgExpr::Expr(expr),
        ..
      } => Some(expr),
      _ => None,
    };

    if let Some(arg_expr) = first_expr {
      // Try to infer type from the first argument
      match arg_expr {
        Expr::Identifier(ident) => {
          let column_name = DisplayIndent(ident).to_string();
          if let Some(table_name) = single_table_name {
            let table_details = &DB_SCHEMA.lock().await.fetch_table(&vec![table_name], db_conn).await;

            if let Some(table_details) = table_details {
              if let Some(field) = table_details.get(&column_name) {
                return ts_query.insert_result(
                  Some(alias),
                  &[field.field_type.to_owned()],
                  is_selection,
                  false, // IFNULL/COALESCE removes nullability
                  expr_for_logging,
                );
              }
            }
          }
        }
        Expr::CompoundIdentifier(idents) if idents.len() == 2 => {
          let column_name = DisplayIndent(&idents[1]).to_string();
          if let Ok(table_name) = translate_table_from_expr(table_with_joins, arg_expr) {
            let table_details = &DB_SCHEMA
              .lock()
              .await
              .fetch_table(&vec![table_name.as_str()], db_conn)
              .await;

            if let Some(table_details) = table_details {
              if let Some(field) = table_details.get(&column_name) {
                return ts_query.insert_result(
                  Some(alias),
                  &[field.field_type.to_owned()],
                  is_selection,
                  false, // IFNULL/COALESCE removes nullability
                  expr_for_logging,
                );
              }
            }
          }
        }
        Expr::Value(val) => {
          // If first arg is a literal value, infer from that
          if let Some(ts_field_type) = translate_value(&val.value) {
            return ts_query.insert_result(Some(alias), &[ts_field_type], is_selection, false, expr_for_logging);
          }
        }
        _ => {}
      }
    }
  }

  // Fallback to Any if we couldn't infer the type
  ts_query.insert_result(Some(alias), &[TsFieldType::Any], is_selection, false, expr_for_logging)
}
