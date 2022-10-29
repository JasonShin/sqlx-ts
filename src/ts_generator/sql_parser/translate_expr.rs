use crate::common::config::TransformConfig;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::information_schema::MySQLSchema;
use crate::ts_generator::types::{DBConn, TsFieldType};
use convert_case::{Case, Casing};
use sqlparser::ast::Expr;
use std::collections::HashMap;

pub fn format_column_name(column_name: String, config: &Option<TransformConfig>) -> String {
    let config = config.clone();
    if config.is_some() && config.unwrap().convert_to_camel_case_column_name {
        return column_name.to_case(Case::Camel);
    }
    column_name
}

pub fn translate_expr(
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
                    let table_details = &mysql_schema.fetch_table(&db_name, &table_name, &conn);
                    if let Some(table_details) = table_details {
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
        Expr::CompoundIdentifier(idents) => {
            // let table_name = get_table_name(a, )
            if idents.len() == 2 {
                let ident = idents[1].value.clone();
                match &db_conn {
                    DBConn::MySQLPooledConn(conn) => {
                        let table_details = &mysql_schema.fetch_table(&db_name, &table_name, &conn);
                        if let Some(table_details) = table_details {
                            let field = table_details.get(&ident).unwrap();

                            result
                                .insert(alias.unwrap().to_string(), vec![field.field_type.clone()]);
                        }
                        return Ok(());
                    }
                    _ => todo!(),
                }
            }
            unimplemented!()
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
        /*
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
         */
        _ => todo!(),
    }
}
