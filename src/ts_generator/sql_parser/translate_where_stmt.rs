use sqlparser::{
    ast::{Expr, TableWithJoins, Value},
    test_utils::table,
};

use crate::ts_generator::{
    errors::TsGeneratorError,
    information_schema::DBSchema,
    types::{DBConn, TsFieldType, TsQuery},
};

use super::{
    translate_expr::{is_expr_placeholder, translate_column_name_expr},
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
    db_name: &str,
    table_with_joins: &Vec<TableWithJoins>,
    db_conn: &DBConn,
) -> Option<TsFieldType> {
    let db_schema = DBSchema::new();
    let table_name = translate_table_from_expr(&table_with_joins, &*left.clone());
    let column_name = translate_column_name_expr(&*left);

    // If the right side of the expression is a placeholder `?` or `$n`
    // they are valid query parameter to process
    if column_name.is_some() && is_expr_placeholder(&*right) && table_name.is_some() {
        let table_name = table_name.unwrap();
        let table_names = vec![table_name.as_str()];
        let column_name = column_name.unwrap();
        let columns = db_schema
            .fetch_table(db_name, &table_names, db_conn)
            .expect(&format!("Failed to fetch columns for table {:?}", table_name));

        // get column and return TsFieldType
        let column = columns.get(column_name.as_str()).expect(&format!(
            "Failed toe find the column from the table schema of {:?}",
            table_name
        ));
        return Some(column.field_type.clone());
    }

    None
}

pub fn translate_where_stmt(
    db_name: &str,
    ts_query: &mut TsQuery,
    expr: &Expr,
    table_with_joins: &Vec<TableWithJoins>,
    db_conn: &DBConn,
) {
    match expr {
        Expr::BinaryOp { left, op, right } => {
            let result = get_sql_query_param(&left, &right, &db_name, &table_with_joins, &db_conn);

            if (result.is_none()) {
                translate_where_stmt(db_name, ts_query, left, table_with_joins, db_conn);
                translate_where_stmt(db_name, ts_query, right, table_with_joins, db_conn);
            } else {
                ts_query.params.push(result.unwrap());
                return;
            }
        }
        _ => {}
    }
}
