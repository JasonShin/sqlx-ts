use crate::common::config::TransformConfig;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::information_schema::MySQLSchema;
use crate::ts_generator::types::{DBConn, TsFieldType, TsQuery};
use convert_case::{Case, Casing};
use sqlparser::ast::SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr};
use sqlparser::ast::{Expr, ObjectName, SetExpr, Statement, TableWithJoins, TableFactor};
use std::collections::HashMap;

pub fn get_table_name(table_with_join: &TableWithJoins) -> Option<String> {
    match &table_with_join.relation {
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
            _ => None,
        },
        _ => None,
    }
}

pub fn format_column_name(column_name: String, config: &Option<TransformConfig>) -> String {
    let config = config.clone();
    if config.is_some() && config.unwrap().convert_to_camel_case_column_name {
        return column_name.to_case(Case::Camel);
    }
    column_name
}

pub fn handle_sql_expr(
    expr: &Expr,
    db_name: &str,
    table_name: &str,
    alias: Option<&str>,
    annotated_result: &HashMap<String, Vec<TsFieldType>>,
    result: &mut HashMap<String, Vec<TsFieldType>>,
    db_conn: &DBConn,
    transformation_config: &Option<TransformConfig>,
) -> Result<(), TsGeneratorError> {
    let mysql_schema = MySQLSchema::new();

    match expr {
        Expr::Identifier(ident) => {
            let column_name = format_column_name(ident.value.to_string(), transformation_config);

            match &db_conn {
                DBConn::MySQLPooledConn(conn) => {
                    // TODO: We can also memoize this method
                    println!("checking db_name {db_name} - tablename? {table_name}");
                    let table_details = &mysql_schema.fetch_table(&db_name, &table_name, &conn);
                    if let Some(table_details) = table_details {
                        println!("identifier handling {:?} - {:?} - {:?}", alias, column_name, table_details);

                        let field = table_details.get(&column_name).unwrap();

                        let field_name = alias.unwrap_or(column_name.as_str()).to_string();
                        result.insert(field_name, vec![field.field_type.clone()]);
                    }
                    Ok(())
                }
                // TODO: Support postgres
                _ => todo!(),
            }
        }
        Expr::IsTrue(query)
        | Expr::IsFalse(query)
        | Expr::IsNull(query)
        | Expr::IsNotNull(query) => {
            // TODO: we can move the follow logic, if alias exists then use alias otherwise throwing err into TsQuery
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), transformation_config);
                // throw error here
                result.insert(alias, vec![TsFieldType::Boolean]);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(
                    query.to_string(),
                ))
            }
        }
        Expr::Exists(query) => {
            // Handles all boolean return type methods
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), transformation_config);
                // throw error here
                result.insert(alias, vec![TsFieldType::Boolean]);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(
                    query.to_string(),
                ))
            }
        }
        Expr::CompoundIdentifier(_) => todo!(),
        Expr::JsonAccess {
            left,
            operator,
            right,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), transformation_config);
                result.insert(alias, vec![TsFieldType::Any]);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(
                    operator.to_string(),
                ))
            }
        }
        Expr::CompositeAccess { expr, key } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), transformation_config);
                result.insert(alias, vec![TsFieldType::Any]);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        /* IsDistinctForm and IsNotDistinctFrom are Postgres syntax, maybe only used in WHERE condition */
        Expr::IsDistinctFrom(_, _) => todo!(),
        Expr::IsNotDistinctFrom(_, _) => todo!(),
        Expr::InList {
            expr,
            list,
            negated,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), transformation_config);
                result.insert(alias, vec![TsFieldType::Boolean, TsFieldType::Null]);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::InSubquery {
            expr,
            subquery,
            negated,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), transformation_config);
                result.insert(alias, vec![TsFieldType::Any]);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::InUnnest {
            expr,
            array_expr,
            negated,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), transformation_config);
                result.insert(alias, vec![TsFieldType::Any]);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Between {
            expr,
            negated,
            low,
            high,
        } => todo!(),
        Expr::BinaryOp { left, op, right } => todo!(),
        Expr::AnyOp(_) => todo!(),
        Expr::AllOp(_) => todo!(),
        Expr::UnaryOp { op, expr } => todo!(),
        Expr::Cast { expr, data_type } => todo!(),
        Expr::TryCast { expr, data_type } => todo!(),
        Expr::Extract { field, expr } => todo!(),
        Expr::Position { expr, r#in } => todo!(),
        Expr::Substring {
            expr,
            substring_from,
            substring_for,
        } => todo!(),
        Expr::Trim { expr, trim_where } => todo!(),
        Expr::Collate { expr, collation } => todo!(),
        Expr::Nested(_) => todo!(),
        Expr::Value(_) => todo!(),
        Expr::TypedString { data_type, value } => todo!(),
        Expr::MapAccess { column, keys } => todo!(),
        Expr::Function(_) => todo!(),
        Expr::Case {
            operand,
            conditions,
            results,
            else_result,
        } => todo!(),
        Expr::Subquery(_) => todo!(),
        Expr::ListAgg(_) => todo!(),
        Expr::GroupingSets(_) => todo!(),
        Expr::Cube(_) => todo!(),
        Expr::Rollup(_) => todo!(),
        Expr::Tuple(_) => todo!(),
        Expr::ArrayIndex { obj, indexes } => todo!(),
        Expr::Array(_) => todo!(),
        _ => todo!(),
    }
}

pub fn handle_sql_statement(
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
                    println!("checking table with joins {:?}", table_with_joins);
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
                                let table_name = get_table_name(default_table).expect(
                                    format!(
                                        "Default FROM table is not found from the query {query}"
                                    )
                                    .as_str(),
                                );

                                // Handles SQL Expression and appends result
                                handle_sql_expr(
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
                                println!("checking expr {:?}", expr);
                                handle_sql_expr(
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
