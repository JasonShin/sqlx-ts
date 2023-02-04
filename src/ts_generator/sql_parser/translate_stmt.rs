use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::translate_expr::translate_expr;
use crate::ts_generator::sql_parser::translate_table_with_joins::*;
use crate::ts_generator::sql_parser::translate_where_stmt::translate_where_stmt;
use crate::ts_generator::sql_parser::translate_wildcard_expr::translate_wildcard_expr;
use crate::ts_generator::types::{DBConn, TsFieldType, TsQuery};
use sqlparser::ast::SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr};
use sqlparser::ast::{Query, SetExpr, Statement};
use std::collections::HashMap;

pub fn translate_stmt(
    ts_query: &mut TsQuery,
    sql_statement: &Statement,
    db_name: &str,
    annotated_results: &HashMap<String, Vec<TsFieldType>>,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    match sql_statement {
        Statement::Query(query) => {
            translate_query(
                ts_query,
                None,
                sql_statement,
                query,
                db_name,
                annotated_results,
                db_conn,
                false,
            )?;
        }
        Statement::Insert { .. } => {
            println!("INSERT statement is not yet supported by TS type generator")
        }
        Statement::Update { .. } => {
            println!("UPDATE statement is not yet supported by TS type generator")
        }
        Statement::Delete { .. } => {
            println!("DELETE statement is not yet supported by TS type generator")
        }
        _ => {
            println!("Unsupported SQL syntax detected, skipping the type generation")
        }
    }
    Ok(())
}

/// translates query
pub fn translate_query(
    ts_query: &mut TsQuery,
    _alias: Option<&str>,
    sql_statement: &Statement,
    query: &Box<Query>,
    db_name: &str,
    annotated_results: &HashMap<String, Vec<TsFieldType>>,
    db_conn: &DBConn,
    is_subquery: bool,
) -> Result<(), TsGeneratorError> {
    let body = &query.body;
    match body {
        SetExpr::Select(select) => {
            let projection = select.clone().projection;
            let table_with_joins = select.clone().from;
            // Handle all select projects and figure out each field's type
            for select_item in projection {
                match &select_item {
                    UnnamedExpr(unnamed_expr) => {
                        let table_name = translate_table_with_joins(&table_with_joins, &select_item)
                            .expect("Default FROM table is not found from the query {query}");

                        // Handles SQL Expression and appends result
                        translate_expr(
                            unnamed_expr,
                            db_name,
                            &table_name,
                            None,
                            annotated_results,
                            ts_query,
                            sql_statement,
                            db_conn,
                            is_subquery,
                        )
                        .unwrap();
                    }
                    ExprWithAlias { expr, alias } => {
                        let alias = alias.to_string();
                        let table_name = translate_table_with_joins(&table_with_joins, &select_item);

                        translate_expr(
                            expr,
                            db_name,
                            table_name.unwrap().as_str(),
                            Some(alias.as_str()),
                            annotated_results,
                            ts_query,
                            sql_statement,
                            db_conn,
                            is_subquery,
                        )
                        .unwrap();
                    }
                    QualifiedWildcard(_) => todo!(),
                    _Wildcard => {
                        translate_wildcard_expr(db_name, sql_statement, &mut ts_query.result, db_conn).unwrap();
                    }
                }
            }

            // If there's any WHERE statements, process it
            if let Some(selection) = select.clone().selection {
                translate_where_stmt(
                    db_name,
                    ts_query,
                    sql_statement,
                    &selection,
                    &table_with_joins,
                    annotated_results,
                    db_conn,
                )?;
            }
            Ok(())
        }
        SetExpr::Query(_) => todo!(),
        SetExpr::SetOperation {
            op: _,
            all: _,
            left: _,
            right: _,
        } => todo!(),
        SetExpr::Values(_) => todo!(),
        SetExpr::Insert(_) => todo!(),
    }
}
