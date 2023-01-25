use crate::common::config::GenerateTypesConfig;
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::information_schema::DBSchema;
use crate::ts_generator::types::{DBConn, TsFieldType, TsQuery};
use crate::ts_generator::sql_parser::translate_stmt::translate_query;
use convert_case::{Case, Casing};
use regex::Regex;
use sqlparser::ast::{Expr, Value, Statement};
use std::collections::HashMap;

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
pub fn get_expr_placeholder(expr: &Expr) -> Option<i32> {
    let re = Regex::new(r"\$(\d+)").unwrap();
    if let Expr::Value(value) = &expr {
        if let Value::Placeholder(placeholder) = value {
            let indexed_binding_params = re.captures(placeholder);
            if placeholder == "?" {
                return None;
            } else if indexed_binding_params.is_some() {
                // Rarely we will get an unwrap issue at this point because invalid syntax should be caught
                // during `prepare` step
                let index = indexed_binding_params
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();

                return Some(index);
            }
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
        Expr::Identifier(ident) => Some(ident.to_string()),
        Expr::CompoundIdentifier(comp) => Some(comp.get(1).unwrap().to_string()),
        _ => None,
    }
}

pub fn format_column_name(column_name: String, config: &Option<GenerateTypesConfig>) -> String {
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
    _annotated_result: &HashMap<String, Vec<TsFieldType>>,
    ts_query: &mut TsQuery,
    sql_statement: &Statement,
    db_conn: &DBConn,
    generate_types_config: &Option<GenerateTypesConfig>,
    is_subquery: bool,
) -> Result<(), TsGeneratorError> {
    let db_schema = DBSchema::new();

    match expr {
        Expr::Identifier(ident) => {
            let column_name = format_column_name(ident.value.to_string(), generate_types_config);

            let table_details = &db_schema.fetch_table(db_name, &vec![table_name], db_conn);

            // TODO: We can also memoize this method
            if let Some(table_details) = table_details {
                let field = table_details.get(&column_name).unwrap();

                let field_name = alias.unwrap_or(column_name.as_str()).to_string();
                ts_query.insert_result(field_name, &vec![field.field_type], is_subquery);
            }
            Ok(())
        }
        Expr::CompoundIdentifier(idents) => {
            // let table_name = get_table_name(a, )
            if idents.len() == 2 {
                let ident = idents[1].value.clone();

                let table_details = &db_schema.fetch_table(db_name, &vec![table_name], db_conn);
                if let Some(table_details) = table_details {
                    let field = table_details.get(&ident).unwrap();

                    ts_query.insert_result(alias.unwrap().to_string(), &vec![field.field_type], is_subquery);
                }
                return Ok(());
            }
            unimplemented!()
        }
        Expr::IsTrue(query) | Expr::IsFalse(query) | Expr::IsNull(query) | Expr::IsNotNull(query) => {
            // TODO: we can move the follow logic, if alias exists then use alias otherwise throwing err into TsQuery
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), generate_types_config);
                // throw error here
                ts_query.insert_result(alias, &vec![TsFieldType::Boolean], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(query.to_string()))
            }
        }
        Expr::Exists(query) => {
            // Handles all boolean return type methods
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), generate_types_config);
                // throw error here
                ts_query.insert_result(alias, &vec![TsFieldType::Boolean], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(query.to_string()))
            }
        }
        Expr::JsonAccess {
            left: _,
            operator,
            right: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), generate_types_config);
                ts_query.insert_result(alias, &vec![TsFieldType::Any], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(operator.to_string()))
            }
        }
        Expr::CompositeAccess { expr, key: _ } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), generate_types_config);
                ts_query.insert_result(alias, &vec![TsFieldType::Any], is_subquery);
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
            list: _,
            negated: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), generate_types_config);
                ts_query.insert_result(alias, &vec![TsFieldType::Boolean, TsFieldType::Null], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Subquery(sub_query) => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), generate_types_config);
                translate_query(
                    ts_query,
                    sql_statement,
                    sub_query, 
                    db_name,
                    _annotated_result,
                    db_conn,
                    generate_types_config,
                    is_subquery
                )?;
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::InSubquery {
            expr,
            subquery: _,
            negated: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), generate_types_config);
                ts_query.insert_result(alias, &vec![TsFieldType::Any], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::InUnnest {
            expr,
            array_expr: _,
            negated: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string(), generate_types_config);
                ts_query.insert_result(alias, &vec![TsFieldType::Any], is_subquery);
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
