use crate::common::lazy::{CONFIG, DB_SCHEMA};
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::expressions::{
    functions::is_string_function, translate_data_type::translate_data_type,
};
use crate::ts_generator::sql_parser::translate_query::translate_query;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::{TsFieldType, TsQuery};
use convert_case::{Case, Casing};
use regex::Regex;
use sqlparser::ast::{Assignment, Expr, Value};

use super::functions::{is_date_function, is_numeric_function};

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
pub fn get_expr_placeholder(expr: &Expr) -> Option<String> {
    let re = Regex::new(r"(\$\d+)").unwrap();
    if let Expr::Value(value) = &expr {
        if let Value::Placeholder(placeholder) = value {
            let indexed_binding_params = re.captures(placeholder);
            if placeholder == "?" {
                return Some("?".to_string());
            } else if indexed_binding_params.is_some() {
                // Rarely we will get an unwrap issue at this point because invalid syntax should be caught
                // during `prepare` step
                let placeholder = indexed_binding_params.unwrap().get(1).unwrap().as_str().to_string();

                return Some(placeholder);
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

pub fn translate_column_name_assignment(assignment: &Assignment) -> Option<String> {
    let left = assignment.id.get(0);
    let right = assignment.id.get(1);

    if left.is_some() && right.is_some() {
        return right.map(|x| x.to_string());
    } else if left.is_some() && right.is_none() {
        return left.map(|x| x.to_string());
    }
    None
}

pub fn format_column_name(column_name: String) -> String {
    let convert_to_camel_case_column_name = &CONFIG
        .generate_types_config
        .to_owned()
        .map(|x| x.convert_to_camel_case_column_name);

    match convert_to_camel_case_column_name {
        Some(true) => column_name.to_case(Case::Camel),
        Some(false) | None => column_name,
    }
}

/// TODO: Add docs about translate expr
pub fn translate_expr(
    expr: &Expr,
    table_name: &str,
    alias: Option<&str>,
    ts_query: &mut TsQuery,
    db_conn: &DBConn,
    // is subquery determines if we can safely append result types into ts_query.results
    // subqueries on WHERE expression should no determine the SELECTIONs
    is_subquery: bool,
) -> Result<(), TsGeneratorError> {
    match expr {
        Expr::CompoundIdentifier(idents) => {
            // let table_name = get_table_name(a, )
            if idents.len() == 2 {
                let ident = idents[1].value.clone();

                let table_details = &DB_SCHEMA.fetch_table(&vec![table_name], db_conn);
                if let Some(table_details) = table_details {
                    let field = table_details.get(&ident).unwrap();

                    // if the select item is a compound identifier and does not has an alias, we should use `table_name.ident` as the key name
                    let key_name = format!("{}_{}", table_name, ident);
                    let key_name = alias.unwrap_or(key_name.as_str());
                    ts_query.insert_result(key_name.to_string(), &[field.field_type.to_owned()], is_subquery);
                }
                return Ok(());
            }
            unimplemented!()
        }
        Expr::Function(function) => {
            let function = function.name.to_string();
            let function = function.as_str();
            let alias = alias.ok_or(TsGeneratorError::FunctionWithoutAliasInSelectClause(expr.to_string()))?;

            if is_string_function(function) {
                ts_query.insert_result(alias.to_string(), &[TsFieldType::String], is_subquery);
            } else if is_numeric_function(function) {
                ts_query.insert_result(alias.to_string(), &[TsFieldType::Number], is_subquery);
            } else if is_date_function(function) {
                ts_query.insert_result(alias.to_string(), &[TsFieldType::String], is_subquery);
            } else {
                return Err(TsGeneratorError::FunctionUnknown(expr.to_string()));
            }

            Ok(())
        }
        Expr::Identifier(ident) => {
            let column_name = format_column_name(ident.value.to_string());

            let table_details = &DB_SCHEMA.fetch_table(&vec![table_name], db_conn);

            // TODO: We can also memoize this method
            if let Some(table_details) = table_details {
                let field = table_details.get(&column_name).unwrap();

                let field_name = alias.unwrap_or(column_name.as_str()).to_string();
                ts_query.insert_result(field_name, &[field.field_type.to_owned()], is_subquery);
            }
            Ok(())
        }

        Expr::IsTrue(query) | Expr::IsFalse(query) | Expr::IsNull(query) | Expr::IsNotNull(query) => {
            // TODO: we can move the follow logic, if alias exists then use alias otherwise throwing err into TsQuery
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                // throw error here
                ts_query.insert_result(alias, &[TsFieldType::Boolean], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(query.to_string()))
            }
        }
        Expr::Exists { negated: _, subquery } => {
            // Handles all boolean return type methods
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                // throw error here
                ts_query.insert_result(alias, &[TsFieldType::Boolean], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(subquery.to_string()))
            }
        }
        Expr::JsonAccess {
            left: _,
            operator,
            right: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Any], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(operator.to_string()))
            }
        }
        Expr::CompositeAccess { expr, key: _ } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Any], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::IsNotDistinctFrom(_, placeholder) | Expr::IsDistinctFrom(_, placeholder) => {
            // IsDistinctFrom and IsNotDistinctFrom are the same and can have a placeholder
            ts_query.insert_param(&TsFieldType::String, &Some(placeholder.to_string()));
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Any], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(placeholder.to_string()))
            }
        }
        Expr::InList {
            expr,
            list: _,
            negated: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Boolean, TsFieldType::Null], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Subquery(sub_query) => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                let alias = alias.as_str();
                translate_query(ts_query, sub_query, db_conn, Some(alias), false)?;
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Nested(expr) => {
            translate_expr(&*expr, table_name, alias, ts_query, db_conn, is_subquery)?;
            Ok(())
        }
        Expr::InSubquery {
            expr,
            subquery: _,
            negated: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Any], is_subquery);
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
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Any], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Cast { expr, data_type } | Expr::TryCast { expr, data_type } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                let data_type = translate_data_type(data_type);
                ts_query.insert_result(alias, &[data_type], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Extract { field: _, expr } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Date], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Position { expr: _, r#in: _ } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Number], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Substring {
            expr: _,
            substring_for: _,
            substring_from: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::String], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::Trim {
            expr: _,
            trim_what: _,
            trim_where: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::String], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        Expr::AtTimeZone { timestamp: _, time_zone: _ } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Date], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        _ => {
            // If nothing matches, we should simply fall back to any
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Any], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
    }
}

pub fn translate_assignment(
    assignment: &Assignment,
    table_name: &str,
    ts_query: &mut TsQuery,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    let value = get_expr_placeholder(&assignment.value);

    if value.is_some() {
        let table_details = &DB_SCHEMA.fetch_table(&vec![table_name], db_conn).unwrap();
        let column_name = translate_column_name_assignment(assignment).unwrap();
        let field = table_details
            .get(&column_name)
            .unwrap_or_else(|| panic!("Failed to find the column detail for {column_name}"));
        ts_query.insert_param(&field.field_type, &value);
    }
    Ok(())
}
