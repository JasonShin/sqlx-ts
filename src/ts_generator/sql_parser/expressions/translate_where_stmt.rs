use sqlparser::ast::{Expr, TableWithJoins};

use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::{
    errors::TsGeneratorError,
    sql_parser::translate_query::translate_query,
    types::db_conn::DBConn,
    types::ts_query::{TsFieldType, TsQuery},
};

use super::translate_data_type::translate_data_type;
use super::{
    translate_expr::{get_expr_placeholder, translate_column_name_expr},
    translate_table_with_joins::translate_table_from_expr,
};

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

pub fn translate_where_stmt(
    ts_query: &mut TsQuery,
    // sql_statement is required
    expr: &Expr,
    // queries like DELETE and INSERT would never have table_with_joins
    single_table_name: &Option<&str>,
    // queries like SELECT might have table_with_joins and we need this explicitly
    table_with_joins: &Option<&Vec<TableWithJoins>>,
    db_conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    match expr {
        Expr::BinaryOp { left, op: _, right } => {
            let param = get_sql_query_param(left, right, single_table_name, table_with_joins, db_conn);
            if param.is_none() {
                translate_where_stmt(ts_query, left, single_table_name, table_with_joins, db_conn);
                translate_where_stmt(ts_query, right, single_table_name, table_with_joins, db_conn);
            } else {
                let (value, index) = param.unwrap();
                ts_query.insert_param(&value, &index);
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
        }
        Expr::InSubquery {
            expr: _,
            subquery,
            negated: _,
        } => {
            // You do not need an alias as we are processing a subquery within the WHERE clause
            translate_query(ts_query, subquery, db_conn, None, true);
        }
        Expr::Subquery(subquery) => {
            translate_query(ts_query, subquery, db_conn, None, true);
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
        }
        Expr::AnyOp(expr) | Expr::AllOp(expr) => {
            translate_where_stmt(ts_query, expr, single_table_name, table_with_joins, db_conn);
        }
        Expr::UnaryOp { op: _, expr } => {
            translate_where_stmt(ts_query, expr, single_table_name, table_with_joins, db_conn);
        }
        Expr::Value(placeholder) => {
            ts_query.insert_param(&TsFieldType::Boolean, &Some(placeholder.to_string()));
        }
        Expr::JsonAccess {
            left: _,
            operator: _,
            right: _,
        } => {
            ts_query.insert_param(&TsFieldType::Any, &None);
        }
        Expr::CompositeAccess { expr, key } => {
            todo!()
        }
        Expr::IsNotDistinctFrom(_, placeholder) | Expr::IsDistinctFrom(_, placeholder) => {
            ts_query.insert_param(&TsFieldType::String, &Some(placeholder.to_string()));
        }
        Expr::InUnnest {
            expr,
            array_expr,
            negated,
        } => todo!(),
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
            ts_query.insert_param(&data_type, &Some(expr.to_string()))
        }
        Expr::AtTimeZone { timestamp, time_zone } => {
            ts_query.insert_param(&TsFieldType::String, &Some(timestamp.to_string()));
            ts_query.insert_param(&TsFieldType::String, &Some(time_zone.to_string()));
            Ok(())
        }
        Expr::Extract { field, expr } => {
            ts_query.insert_param(&TsFieldType::String, &Some(field.to_string()));
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()));
            Ok(())
        }
        Expr::Floor { expr, field: _ } | Expr::Ceil { expr, field: _ } => {
            ts_query.insert_param(&TsFieldType::Number, &Some(expr.to_string()));
            Ok(())
        }
        Expr::Position { expr, r#in } => todo!(),
        Expr::Substring {
            expr,
            substring_from,
            substring_for,
        } => {
            ts_query.insert_param(&TsFieldType::String, &Some(expr.to_string()))
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
        _ => Ok(())
    }
}
