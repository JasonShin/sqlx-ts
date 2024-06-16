use crate::core::connection::DBConn;
use crate::ts_generator::sql_parser::expressions::translate_expr::get_expr_placeholder;
use sqlparser::ast::{Ident, Query, SelectItem, SetExpr};

use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::{errors::TsGeneratorError, types::ts_query::TsQuery};

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
                                panic!("Matching column of idx {column} is not found while processing insert params")
                            })
                            .value;

                        let field = table_details.get(match_col.as_str()).unwrap_or_else(|| {
                            panic!("Column {match_col} is not found while processing insert params")
                        });

                        if value.to_string() == "?" {
                            // If the placeholder is `'?'`, we can process it using insert_value_params and generate nested params type
                            ts_query.insert_value_params(&field.field_type, &(row, column), &placeholder);
                        } else {
                            ts_query.insert_param(&field.field_type, &placeholder)?;
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    }

    Ok(())
}

pub async fn translate_insert_returning(
    ts_query: &mut TsQuery,
    returning: &Vec<SelectItem>,
    table_name: &str,
    conn: &DBConn,
    query_for_logging: &str,
) {
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
                // ts_query.insert_result(alias, value, is_selection, expr_for_logging)
                let name = unnamed_expr.to_string();
                let name = name.as_str();
                let match_col = table_details
                    .get(name)
                    .unwrap_or_else(|| panic!("Column {name} is not found while processing insert params"));
                let value = vec![match_col.field_type.clone()];
                ts_query.insert_result(Some(name), &value, true, query_for_logging);
            }
            SelectItem::ExprWithAlias { expr, alias } => {
                let name = expr.to_string();
                let match_col = table_details
                    .get(name.as_str())
                    .unwrap_or_else(|| panic!("Column {name} is not found while processing insert params"));
                let alias = alias.to_string();
                let alias = alias.as_str();
                let value = vec![match_col.field_type.clone()];
                ts_query.insert_result(Some(alias), &value, true, query_for_logging);
            }
            SelectItem::Wildcard(wildcard) => {
                let keys = table_details.keys();
                for key in keys {
                    let value = vec![table_details.get(key).unwrap().field_type.clone()];
                    ts_query.insert_result(Some(key), &value, true, query_for_logging);
                }
            }
            _ => {}
        }
    }
}
