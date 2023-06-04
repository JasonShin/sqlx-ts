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
use sqlparser::ast::{Assignment, Expr, Value, TableWithJoins};
use sqlparser::test_utils::table;

use super::functions::{is_date_function, is_numeric_function};
use super::translate_table_with_joins::translate_table_from_expr;

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

/// handle an expression from where clauses (or it can be from anywhere)
/// pick up any expression from left and right that goes
/// some_field = ?
/// some_table.some_field = ?
///
/// or
///
/// some_field = $1
/// some_table.some_field = $1
pub fn get_sql_query_param(
    left: &Box<Expr>,
    right: &Box<Expr>,
    single_table_name: &Option<&str>,
    table_with_joins: &Option<&Vec<TableWithJoins>>,
    db_conn: &DBConn,
) -> Option<(TsFieldType, Option<String>)> {
    let table_name: Option<String>;

    if table_with_joins.is_some() {
        table_name = translate_table_from_expr(table_with_joins.unwrap(), &left.clone());
    } else if single_table_name.is_some() {
        table_name = single_table_name.map(|x| x.to_string());
    } else {
        panic!("failed to find an appropriate table name while processing WHERE statement")
    }

    let column_name = translate_column_name_expr(left);

    // If the right side of the expression is a placeholder `?` or `$n`
    // they are valid query parameter to process
    let expr_placeholder = get_expr_placeholder(right);

    if column_name.is_some() && expr_placeholder.is_some() && table_name.is_some() {
        let table_name = table_name.unwrap();
        let table_names = vec![table_name.as_str()];
        let column_name = column_name.unwrap();
        let columns = DB_SCHEMA
            .fetch_table(&table_names, db_conn)
            .unwrap_or_else(|| panic!("Failed to fetch columns for table {:?}", table_name));

        // get column and return TsFieldType
        let column = columns
            .get(column_name.as_str())
            .unwrap_or_else(|| panic!("Failed toe find the column from the table schema of {:?}", table_name));
        return Some((column.field_type.to_owned(), expr_placeholder));
    }

    None
}

/// TODO: Add docs about translate expr
pub fn translate_expr(
    expr: &Expr,
    single_table_name: &Option<&str>,
    table_with_joins: &Option<&Vec<TableWithJoins>>,
    alias: Option<&str>,
    ts_query: &mut TsQuery,
    db_conn: &DBConn,
    // is subquery determines if we can safely append result types into ts_query.results
    // subqueries on WHERE expression should no determine the SELECTIONs
    is_subquery: bool,
) -> Result<(), TsGeneratorError> {
    match expr {
        Expr::Identifier(ident) => {
            let column_name = format_column_name(ident.value.to_string());
            let table_name = single_table_name.expect("Missing table name for identifier");

            let table_details = &DB_SCHEMA.fetch_table(&vec![table_name], db_conn);

            // TODO: We can also memoize this method
            if let Some(table_details) = table_details {
                let field = table_details.get(&column_name).unwrap();

                let field_name = alias.unwrap_or(column_name.as_str()).to_string();
                ts_query.insert_result(field_name, &[field.field_type.to_owned()], is_subquery);
            }
            Ok(())
        }
        Expr::CompoundIdentifier(idents) => {
            // let table_name = get_table_name(a, )
            if idents.len() == 2 {
                let ident = idents[1].value.clone();
                let table_name = single_table_name.expect("Missing table name for compound identifier");

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
        /////////////////////
        // OPERATORS START //
        /////////////////////
        Expr::BinaryOp { left, op: _, right } => {
            let param = get_sql_query_param(left, right, single_table_name, table_with_joins, db_conn);
            if param.is_none() {
                translate_expr(&*left, single_table_name, table_with_joins, alias, ts_query, db_conn, is_subquery)?;
                translate_expr(&*right, single_table_name, table_with_joins, alias, ts_query, db_conn, is_subquery)?;
                Ok(())
            } else {
                let (value, index) = param.unwrap();
                ts_query.insert_param(&value, &index);
                Ok(())
            }
        }
        Expr::InList { expr, list, negated: _ } => {
            // If the list is just a single `(?)`, then we should return the dynamic
            // If the list contains multiple `(?, ?...)` then we should return a fixed length array
            if list.len() == 1 {
                let right = list
                    .get(0)
                    .expect("Failed to find the first list item from the IN query");
                let result = get_sql_query_param(
                    expr,
                    &Box::new(right.to_owned()),
                    single_table_name,
                    table_with_joins,
                    db_conn,
                );

                if result.is_some() {
                    let (value, index) = result.unwrap();
                    let array_item = value.to_array_item();

                    ts_query.insert_param(&array_item, &index);
                    return Ok(());
                } else {
                    return Ok(());
                }
            }
            Ok(())
        }
        Expr::InSubquery {
            expr: _,
            subquery,
            negated: _,
        } => {
            // You do not need an alias as we are processing a subquery within the WHERE clause
            translate_query(ts_query, subquery, db_conn, None, true)?;
            Ok(())
        }
        Expr::Subquery(subquery) => {
            translate_query(ts_query, subquery, db_conn, None, true)?;
            Ok(())
        }
        Expr::Between {
            expr,
            negated: _,
            low,
            high,
        } => {
            let low = get_sql_query_param(expr, low, single_table_name, table_with_joins, db_conn);
            let high = get_sql_query_param(expr, high, single_table_name, table_with_joins, db_conn);
            if low.is_some() {
                let (value, placeholder) = low.unwrap();
                ts_query.insert_param(&value, &placeholder);
            }

            if high.is_some() {
                let (value, placeholder) = high.unwrap();
                ts_query.insert_param(&value, &placeholder);
            }
            Ok(())
        }
        Expr::AnyOp(expr) | Expr::AllOp(expr) => {
            translate_expr(&*expr, single_table_name, table_with_joins, alias, ts_query, db_conn, is_subquery);
            Ok(())
        }
        Expr::UnaryOp { op: _, expr } => {
            translate_expr(&*expr, single_table_name, table_with_joins, alias, ts_query, db_conn, is_subquery);
            Ok(())
        }
        Expr::Value(placeholder) => {
            ts_query.insert_param(&TsFieldType::Boolean, &Some(placeholder.to_string()));
            Ok(())
        }
        Expr::JsonAccess {
            left: _,
            operator: _,
            right: _,
        } => {
            ts_query.insert_param(&TsFieldType::Any, &None);
            Ok(())
        }
        Expr::CompositeAccess { expr, key } => {
            todo!()
        }
        Expr::IsNotDistinctFrom(_, placeholder) | Expr::IsDistinctFrom(_, placeholder) => {
            ts_query.insert_param(&TsFieldType::String, &Some(placeholder.to_string()));
            Ok(())
        }
        Expr::SimilarTo {
            negated: _,
            expr: _,
            pattern,
            escape_char: _,
        }
        | Expr::ILike {
            negated: _,
            expr: _,
            pattern,
            escape_char: _,
        }
        | Expr::Like {
            negated: _,
            expr: _,
            pattern,
            escape_char: _,
        } => {
            // If the pattern has a placeholder, then we should append the param to ts_query
            ts_query.insert_param(&TsFieldType::String, &Some(pattern.to_string()));
            Ok(())
        }
        Expr::TryCast { expr, data_type } | Expr::SafeCast { expr, data_type } | Expr::Cast { expr, data_type } => {
            let data_type = translate_data_type(data_type);
            ts_query.insert_param(&data_type, &Some(expr.to_string()));
        }
        Expr::AtTimeZone { timestamp, time_zone } => {
            ts_query.insert_param(&TsFieldType::String, &Some(timestamp.to_string()));
            ts_query.insert_param(&TsFieldType::String, &Some(time_zone.to_string()));
        }
        Expr::Extract { field, expr } => {
            ts_query.insert_param(&TsFieldType::String, &Some(field.to_string()));
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()));
        }
        Expr::Floor { expr, field: _ } | Expr::Ceil { expr, field: _ } => {
            ts_query.insert_param(&TsFieldType::Number, &Some(expr.to_string()));
        }
        Expr::Position { expr, r#in } => todo!(),
        Expr::Substring {
            expr,
            substring_from,
            substring_for,
        } => {
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()));
        }
        Expr::Trim {
            expr,
            trim_where: _,
            trim_what: _,
        } => {
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()));
        }
        Expr::Overlay {
            expr,
            overlay_what,
            overlay_from,
            overlay_for,
        } => todo!(),
        Expr::Collate { expr, collation } => todo!(),
        Expr::Nested(_) => todo!(),
        Expr::IntroducedString { introducer, value } => todo!(),
        Expr::TypedString { data_type, value } => todo!(),
        Expr::MapAccess { column, keys } => todo!(),
        Expr::Function(_) => todo!(),
        Expr::AggregateExpressionWithFilter { expr, filter } => todo!(),
        Expr::Case {
            operand,
            conditions,
            results,
            else_result,
        } => todo!(),
        Expr::Exists { subquery, negated } => todo!(),
        Expr::ArraySubquery(_) => todo!(),
        Expr::ListAgg(_) => todo!(),
        Expr::ArrayAgg(_) => todo!(),
        Expr::GroupingSets(_) => todo!(),
        Expr::Cube(_) => todo!(),
        Expr::Rollup(_) => todo!(),
        Expr::Tuple(_) => todo!(),
        Expr::ArrayIndex { obj, indexes } => todo!(),
        Expr::Array(_) => todo!(),
        Expr::Interval {
            value,
            leading_field,
            leading_precision,
            last_field,
            fractional_seconds_precision,
        } => todo!(),
        Expr::MatchAgainst {
            columns,
            match_value,
            opt_search_modifier,
        } => todo!(),
        /////////////////////
        // OPERATORS ENDS  //
        /////////////////////

        /////////////////////
        // FUNCTIONS START //
        /////////////////////
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
        Expr::Floor { expr, field } | Expr::Ceil { expr, field } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Number], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
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
        Expr::Exists { negated: _, subquery } => {
            // Handles all boolean return type methods
            if alias.is_some() {
                translate_query(ts_query, subquery, db_conn, alias, true)?;
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
                ts_query.insert_result(alias, &[TsFieldType::Boolean], is_subquery);
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
        Expr::AtTimeZone {
            timestamp: _,
            time_zone: _,
        } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Date], is_subquery);
                Ok(())
            } else {
                Err(TsGeneratorError::MissingAliasForFunctions(expr.to_string()))
            }
        }
        /////////////////////
        // FUNCTIONS END //
        /////////////////////
        Expr::CompositeAccess { expr, key: _ } => {
            if alias.is_some() {
                let alias = format_column_name(alias.unwrap().to_string());
                ts_query.insert_result(alias, &[TsFieldType::Any], is_subquery);
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
            translate_expr(&*expr, single_table_name, table_with_joins, alias, ts_query, db_conn, is_subquery)?;
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
