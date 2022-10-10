use crate::common::config::TransformConfig;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::translate_expr::translate_expr;
use crate::ts_generator::types::{DBConn, TsFieldType, TsQuery};
use sqlparser::ast::SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr};
use sqlparser::ast::{SetExpr, Statement, TableWithJoins};
use std::collections::HashMap;

pub fn get_table_name(
    // TODO: update table with join argument to be vec and make it work using an array
    table_with_joins: &Vec<TableWithJoins>,
    table_name: Option<String>,
) -> Option<String> {
    // if there
    // println!("checking joins {:?}", table_with_join.joins);
    /*if let Some(ci) = compound_identifier {
        let table_alias = ci.get(0).unwrap().to_string();
        // let mut table_name_result = None;

        for join in &table_with_joins.joins {
            println!("checking join {:?}", join);
        }
    }*/

    /*
    let found = &table_with_joins
    .into_iter()
    .find(|&table_with_join| {
        let result: Option<String> = match &table_with_join.relation {
            TableFactor::Table {
                name,
                alias,
                args,
                with_hints,
            } => {
                /*match name {
                    ObjectName(val) => {
                        let alias = alias
                            .clone()
                            .and_then(|alias| Some(alias.clone().name.to_string()));
                        let name = val.get(0).and_then(|val| Some(val.value.to_string()));

                        if alias.is_some() {
                            return alias;
                        } else if name.is_some() {
                            return name;
                        }
                        None
                    },
                    _ => None,
                }*/

                let test: Option<String> = match name {
                    ObjectName(val) => {
                        let alias = alias
                            .clone()
                            .and_then(|alias| Some(alias.clone().name.to_string()));
                        
                        let name = val.get(0).and_then(|val| Some(val.value.to_string()));

                        if alias.is_some() {
                            return alias;
                        } else if name.is_some() {
                            return name;
                        }
                        None
                    }
                };

                Some("test".to_string())
            },    
            _ => None,
        };

        true
    });
    */
    // otherwise always return the default table name
    /*
    match &table_with_joins.relation {
        TableFactor::Table {
            name,
            alias,
            args,
            with_hints,
        } => match name {
            ObjectName(val) => {
                let alias = alias
                    .clone()
                    .and_then(|alias| Some(alias.clone().name.to_string()));
                let name = val.get(0).and_then(|val| Some(val.value.to_string()));

                if alias.is_some() {
                    return alias;
                } else if name.is_some() {
                    return name;
                }
                None
            }
            // TODO: handle select fields for join tables
            _ => None,
        },
        _ => None,
    }
    */
    Some("default".to_string())
}

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
                        match select_item {
                            UnnamedExpr(unnamed_expr) => {
                                // TODO: refactor this to figure out proper table name even with JOINs
                                let default_table = table_with_joins.get(0).expect(
                                    format!(
                                        "Default FROM table is not found from the query {query}"
                                    )
                                    .as_str(),
                                );
                                // TODO: get table name should be against the column name t.id, items.id
                                let table_name = get_table_name(&table_with_joins, None).expect(
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
                                //let table_name = get_table_name(table_with_joins, Some(vec![expr]));
                                println!("checking expr {:#?}", expr);

                                translate_expr(
                                    &expr,
                                    &db_name,
                                    "",
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
