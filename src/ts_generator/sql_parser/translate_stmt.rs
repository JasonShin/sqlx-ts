use crate::common::config::TransformConfig;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::translate_expr::translate_expr;
use crate::ts_generator::sql_parser::translate_table_with_joins::*;
use crate::ts_generator::types::{DBConn, TsFieldType, TsQuery};
use sqlparser::ast::SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr};
use sqlparser::ast::{SetExpr, Statement, TableWithJoins};
use std::collections::HashMap;

pub fn translate_stmt(
    ts_query: &mut TsQuery,
    sql_statement: &Statement,
    db_name: &str,
    annotated_results: &HashMap<String, Vec<TsFieldType>>,
    db_conn: &DBConn,
    transformation_config: &Option<TransformConfig>,
) -> Result<(), TsGeneratorError> {
    match sql_statement {
        Statement::Query(query) => {
            let body = &query.body;
            match body {
                SetExpr::Select(select) => {
                    let projection = select.clone().projection;
                    let table_with_joins = select.clone().from;
                    println!("checking table with joins {:#?}", table_with_joins);
                    // then fetch information schema to figure out each field's details
                    for select_item in projection {
                        match &select_item {
                            UnnamedExpr(unnamed_expr) => {
                                let table_name = translate_table_with_joins(&table_with_joins, &select_item).expect(
                                    format!(
                                        "Default FROM table is not found from the query {query}"
                                    )
                                    .as_str(),
                                );

                                // Handles SQL Expression and appends result
                                translate_expr(
                                    &unnamed_expr,
                                    &db_name,
                                    &table_name,
                                    None,
                                    &annotated_results,
                                    &mut ts_query.result,
                                    &db_conn,
                                    &transformation_config,
                                )?;
                            }
                            ExprWithAlias { expr, alias } => {
                                let alias = alias.to_string();
                                let table_name = translate_table_with_joins(&table_with_joins, &select_item);

                                translate_expr(
                                    &expr,
                                    &db_name,
                                    table_name.unwrap().as_str(),
                                    Some(alias.as_str()),
                                    &annotated_results,
                                    &mut ts_query.result,
                                    &db_conn,
                                    &transformation_config,
                                )?;
                            }
                            QualifiedWildcard(_) => todo!(),
                            Wildcard => todo!(),
                        }
                    }
                }
                SetExpr::Query(_) => todo!(),
                SetExpr::SetOperation {
                    op,
                    all,
                    left,
                    right,
                } => todo!(),
                SetExpr::Values(_) => todo!(),
                SetExpr::Insert(_) => todo!(),
            }
        }
        _ => {}
    }
    Ok(())
}
