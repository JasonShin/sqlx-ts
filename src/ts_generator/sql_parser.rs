use crate::common::config::TransformConfig;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::information_schema::MySQLSchema;
use crate::ts_generator::types::{DBConn, TsFieldType, TsQuery};
use convert_case::{Case, Casing};
use sqlparser::ast::SelectItem::{ExprWithAlias, QualifiedWildcard, UnnamedExpr};
use sqlparser::ast::{Expr, ObjectName, SetExpr, Statement, TableWithJoins};
use std::collections::HashMap;

pub fn get_table_name(table_with_join: &TableWithJoins) -> Option<String> {
    match &table_with_join.relation {
        sqlparser::ast::TableFactor::Table {
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
    result: &mut HashMap<String, TsFieldType>,
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
                    let table_details = &mysql_schema.fetch_table(&db_name, &table_name, &conn);
                    if let Some(table_details) = table_details {
                        let field = table_details.get(&column_name).unwrap();
                        result.insert(column_name.clone(), field.field_type.clone());
                    }
                    Ok(())
                }
                _ => todo!(),
            }
        }
        Expr::IsTrue(query)
        | Expr::IsFalse(query)
        | Expr::IsNull(query)
        | Expr::IsNotNull(query) => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), transformation_config);
                // throw error here
                result.insert(alias, TsFieldType::Boolean);
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
                result.insert(alias, TsFieldType::Boolean);
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
                result.insert(alias, TsFieldType::Any);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(
                    operator.to_string(),
                ))
            }
        }
        Expr::CompositeAccess { expr, key } => todo!(),
        Expr::IsDistinctFrom(_, _) => todo!(),
        Expr::IsNotDistinctFrom(_, _) => todo!(),
        Expr::InList {
            expr,
            list,
            negated,
        } => todo!(),
        Expr::InSubquery {
            expr,
            subquery,
            negated,
        } => todo!(),
        Expr::InUnnest {
            expr,
            array_expr,
            negated,
        } => todo!(),
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
                    // then fetch information schema to figure out each field's details
                    for select_item in projection {
                        match select_item {
                            UnnamedExpr(unnamed_expr) => {
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
                                    &mut ts_query.result,
                                    &db_conn,
                                    &transformation_config,
                                )?;
                            }
                            ExprWithAlias { expr, alias } => {
                                let alias = alias.to_string();
                                handle_sql_expr(
                                    &expr,
                                    &db_name,
                                    "",
                                    Some(alias.as_str()),
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
