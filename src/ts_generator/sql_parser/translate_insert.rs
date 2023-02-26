use crate::ts_generator::sql_parser::expressions::translate_expr::get_expr_placeholder;
use sqlparser::ast::{Ident, Query, SetExpr};

use crate::common::lazy::DB_SCHEMA;
use crate::ts_generator::{
    errors::TsGeneratorError,
    types::{DBConn, TsQuery},
};

pub fn translate_insert(
    ts_query: &mut TsQuery,
    columns: &Vec<Ident>,
    source: &Box<Query>,
    db_name: &str,
    table_name: &str,
    conn: &DBConn,
) -> Result<(), TsGeneratorError> {
    let table_details = DB_SCHEMA
        .fetch_table(db_name, &vec![table_name], conn)
        // Nearly impossible to panic at this point as we've already validated queries with prepare statements
        .unwrap();

    let values = &source.body;

    match values {
        SetExpr::Values(values) => {
            // Process the rows
            for (row, values) in values.0.iter().enumerate() {
                // Given a list of values
                // [ [?, 1, ?], [?, ?, ?] ]
                // Loop each value placeholder / actual values, if it finds the placeholder either `?` or `$n`
                // build the insert param in `types.rs`
                for (column, value) in values.iter().enumerate() {
                    let placeholder = get_expr_placeholder(value);

                    if placeholder.is_some() {
                        let match_col = columns
                            .get(column)
                            .unwrap_or_else(|| panic!("Matching column of idx {column} is not found while processing insert params"))
                            .to_string();

                        let field = table_details.get(match_col.as_str()).unwrap_or_else(|| panic!("Column {match_col} is not found while processing insert params"));

                        if value.to_string() == "?" {
                            // If the placeholder is `'?'`, we can process it using insert_value_params and generate nested params type
                            ts_query.insert_value_params(&field.field_type, &(row, column), &placeholder);
                        } else {
                            ts_query.insert_param(&field.field_type, &placeholder);
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    }

    Ok(())
}
