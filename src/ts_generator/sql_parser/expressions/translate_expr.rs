use crate::common::lazy::{CONFIG, DB_SCHEMA};
use crate::ts_generator::errors::TsGeneratorError;
use crate::ts_generator::sql_parser::expressions::translate_data_type::translate_value;
use crate::ts_generator::sql_parser::expressions::translate_table_with_joins::translate_table_from_expr;
use crate::ts_generator::sql_parser::expressions::{
    functions::is_string_function, translate_data_type::translate_data_type,
};
use crate::ts_generator::sql_parser::translate_query::translate_query;
use crate::ts_generator::types::db_conn::DBConn;
use crate::ts_generator::types::ts_query::{TsFieldType, TsQuery};
use convert_case::{Case, Casing};
use regex::Regex;
use sqlparser::ast::{Assignment, Expr, TableWithJoins, Value};

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
    table_with_joins: &Option<Vec<TableWithJoins>>,
    db_conn: &DBConn,
) -> Option<(TsFieldType, Option<String>)> {
    let table_name: Option<String>;

    if table_with_joins.is_some() {
        table_name = translate_table_from_expr(table_with_joins, &left.clone());
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
    table_with_joins: &Option<Vec<TableWithJoins>>,
    alias: Option<&str>,
    ts_query: &mut TsQuery,
    db_conn: &DBConn,
    // is subquery determines if we can safely append result types into ts_query.results
    // subqueries on WHERE expression should no determine the SELECTIONs
    is_selection: bool,
) -> Result<(), TsGeneratorError> {
    let binding = expr.to_string();
    let expr_for_logging = &binding.as_str();
    match expr {
        Expr::Identifier(ident) => {
            let column_name = ident.value.to_string();
            let table_name = single_table_name.expect("Missing table name for identifier");
            let table_details = &DB_SCHEMA.fetch_table(&vec![table_name], db_conn);

            // TODO: We can also memoize this method
            if let Some(table_details) = table_details {
                let field = table_details.get(&column_name).unwrap();

                let field_name = alias.unwrap_or(column_name.as_str());
                ts_query.insert_result(
                    Some(field_name),
                    &[field.field_type.to_owned()],
                    is_selection,
                    &expr_for_logging,
                )?
            }
            Ok(())
        }
        Expr::CompoundIdentifier(idents) => {
            if idents.len() == 2 {
                let ident = idents[1].value.clone();

                let table_name = translate_table_from_expr(table_with_joins, &expr)
                    .ok_or_else(|| TsGeneratorError::IndentifierWithoutTable(expr.to_string()))?;

                let table_details = &DB_SCHEMA.fetch_table(&vec![table_name.as_str()], db_conn);
                if let Some(table_details) = table_details {
                    let field = table_details.get(&ident).unwrap();

                    // if the select item is a compound identifier and does not has an alias, we should use `table_name.ident` as the key name
                    let key_name = format!("{}_{}", table_name, ident);
                    let key_name = alias.unwrap_or(key_name.as_str());
                    ts_query.insert_result(
                        Some(key_name),
                        &[field.field_type.to_owned()],
                        is_selection,
                        &expr_for_logging,
                    )?;
                }
            }
            Ok(())
        }
        /////////////////////
        // OPERATORS START //
        /////////////////////
        Expr::BinaryOp { left, op: _, right } => {
            let param = get_sql_query_param(left, right, single_table_name, table_with_joins, db_conn);
            if param.is_none() {
                translate_expr(
                    &*left,
                    single_table_name,
                    table_with_joins,
                    alias,
                    ts_query,
                    db_conn,
                    is_selection,
                )?;
                translate_expr(
                    &*right,
                    single_table_name,
                    table_with_joins,
                    alias,
                    ts_query,
                    db_conn,
                    is_selection,
                )?;
                Ok(())
            } else {
                let (value, index) = param.unwrap();
                ts_query.insert_param(&value, &index);
                Ok(())
            }
        }
        Expr::InList { expr, list, negated: _ } => {
            ts_query.insert_result(alias, &[TsFieldType::Boolean], is_selection, expr_for_logging)?;
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
            translate_query(ts_query, &mut None, subquery, db_conn, None, false)?;
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
        Expr::AnyOp(expr) | Expr::AllOp(expr) => translate_expr(
            &*expr,
            single_table_name,
            table_with_joins,
            alias,
            ts_query,
            db_conn,
            is_selection,
        ),
        Expr::UnaryOp { op: _, expr } => translate_expr(
            &*expr,
            single_table_name,
            table_with_joins,
            alias,
            ts_query,
            db_conn,
            is_selection,
        ),
        Expr::Value(placeholder) => {
            if placeholder.to_string() == "?" {
                ts_query.insert_param(&TsFieldType::Boolean, &Some("?".to_string()))
            } else {
                let ts_field_type = translate_value(&placeholder);
                ts_query.insert_result(alias, &[ts_field_type], is_selection, expr_for_logging)
            }
        },
        Expr::JsonAccess {
            left: _,
            operator: _,
            right: _,
        } => {
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)?;
            ts_query.insert_param(&TsFieldType::Any, &None)
        }
        Expr::IsNotDistinctFrom(_, placeholder) | Expr::IsDistinctFrom(_, placeholder) => {
            // IsDistinctFrom and IsNotDistinctFrom are the same and can have a placeholder
            ts_query.insert_param(&TsFieldType::String, &Some(placeholder.to_string()))?;
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
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
            ts_query.insert_param(&TsFieldType::String, &Some(pattern.to_string()))
        }
        Expr::TryCast { expr, data_type } | Expr::SafeCast { expr, data_type } | Expr::Cast { expr, data_type } => {
            let data_type = translate_data_type(data_type);
            ts_query.insert_result(alias, &[data_type.clone()], is_selection, expr_for_logging)?;
            ts_query.insert_param(&data_type, &Some(expr.to_string()))?;
            Ok(())
        }
        Expr::AtTimeZone { timestamp, time_zone } => {
            ts_query.insert_result(alias, &[TsFieldType::Date], is_selection, expr_for_logging)?;
            ts_query.insert_param(&TsFieldType::String, &Some(timestamp.to_string()))?;
            ts_query.insert_param(&TsFieldType::String, &Some(time_zone.to_string()))?;
            Ok(())
        }
        Expr::Extract { field, expr } => {
            ts_query.insert_result(alias, &[TsFieldType::Date], is_selection, expr_for_logging)?;
            ts_query.insert_param(&TsFieldType::String, &Some(field.to_string()))?;
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))?;
            Ok(())
        }
        Expr::Floor { expr, field: _ } | Expr::Ceil { expr, field: _ } => {
            ts_query.insert_result(alias, &[TsFieldType::Number], is_selection, expr_for_logging)?;
            ts_query.insert_param(&TsFieldType::Number, &Some(expr.to_string()))
        }
        Expr::Position { expr, r#in } => {
            ts_query.insert_result(alias, &[TsFieldType::Number], is_selection, expr_for_logging)
        }
        Expr::Substring {
            expr,
            substring_from,
            substring_for,
        } => {
            ts_query.insert_result(alias, &[TsFieldType::String], is_selection, expr_for_logging)?;
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))
        }
        Expr::Trim {
            expr,
            trim_where: _,
            trim_what: _,
        } => {
            ts_query.insert_result(alias, &[TsFieldType::String], is_selection, expr_for_logging)?;
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))
        }
        Expr::Overlay {
            expr,
            overlay_what,
            overlay_from,
            overlay_for: _,
        } => {
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))?;
            ts_query.insert_param(&TsFieldType::String, &Some(overlay_what.to_string()))?;
            ts_query.insert_param(&TsFieldType::Number, &Some(overlay_from.to_string()))?;
            ts_query.insert_result(alias, &[TsFieldType::String], is_selection, expr_for_logging)
        }
        Expr::Collate { expr, collation } => {
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
        }
        Expr::IntroducedString { introducer, value } => {
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
        }
        Expr::TypedString { data_type, value } => {
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
        }
        Expr::MapAccess { column, keys } => {
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
        }
        Expr::AggregateExpressionWithFilter { expr, filter } => {
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
        }
        Expr::Case {
            operand: _,
            conditions: _,
            results: _,
            else_result: _,
        } => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging),
        Expr::Exists { subquery, negated: _ } => {
            ts_query.insert_result(alias, &[TsFieldType::Boolean], is_selection, expr_for_logging)?;
            translate_query(ts_query, &mut None, *&subquery, db_conn, alias, false)
        }
        Expr::ListAgg(_)
        | Expr::ArrayAgg(_)
        | Expr::GroupingSets(_)
        | Expr::GroupingSets(_)
        | Expr::Cube(_)
        | Expr::Rollup(_)
        | Expr::Tuple(_)
        | Expr::Array(_)
        | Expr::ArraySubquery(_) => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging),
        Expr::ArrayIndex { obj, indexes } => {
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
        }
        Expr::Interval {
            value: _,
            leading_field: _,
            leading_precision: _,
            last_field: _,
            fractional_seconds_precision: _,
        } => ts_query.insert_result(alias, &[TsFieldType::Number], is_selection, expr_for_logging),
        Expr::MatchAgainst {
            columns,
            match_value,
            opt_search_modifier,
        } => ts_query.insert_result(alias, &[TsFieldType::Number], is_selection, expr_for_logging),
        /////////////////////
        // OPERATORS ENDS  //
        /////////////////////

        /////////////////////
        // FUNCTIONS START //
        /////////////////////
        Expr::IsTrue(query) | Expr::IsFalse(query) | Expr::IsNull(query) | Expr::IsNotNull(query) => {
            ts_query.insert_result(alias, &[TsFieldType::Boolean], is_selection, expr.to_string().as_str())
        }
        Expr::Function(function) => {
            let function = function.name.to_string();
            let function = function.as_str();
            let alias = alias.ok_or(TsGeneratorError::FunctionWithoutAliasInSelectClause(expr.to_string()))?;
            if is_string_function(function) {
                ts_query.insert_result(Some(alias), &[TsFieldType::String], is_selection, expr_for_logging)?;
            } else if is_numeric_function(function) {
                ts_query.insert_result(Some(alias), &[TsFieldType::Number], is_selection, expr_for_logging)?;
            } else if is_date_function(function) {
                ts_query.insert_result(Some(alias), &[TsFieldType::String], is_selection, expr_for_logging)?;
            } else {
                return Err(TsGeneratorError::FunctionUnknown(expr.to_string()));
            }

            Ok(())
        }
        /////////////////////
        // FUNCTIONS END //
        /////////////////////
        Expr::CompositeAccess { expr, key: _ } => {
            ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, expr_for_logging)
        }
        Expr::Subquery(sub_query) => {
            // For the first layer of subquery, we consider the first field selected as the result
            if is_selection && table_with_joins.clone().unwrap().len() == 1 {
                return translate_query(ts_query, &table_with_joins, sub_query, db_conn, alias, true);
            }
            translate_query(ts_query, &table_with_joins, sub_query, db_conn, alias, false)
        }
        Expr::Nested(expr) => translate_expr(
            &*expr,
            single_table_name,
            table_with_joins,
            alias,
            ts_query,
            db_conn,
            is_selection,
        ),
        Expr::InUnnest {
            expr,
            array_expr: _,
            negated: _,
        } => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, &expr_for_logging),
        _ => ts_query.insert_result(alias, &[TsFieldType::Any], is_selection, &expr_for_logging),
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
